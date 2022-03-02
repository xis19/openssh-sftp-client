use std::cmp::min;
use std::future::Future;
use std::io;
use std::ops::Deref;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Buf, BytesMut};

use tokio::io::{AsyncBufRead, AsyncRead, ReadBuf};
use tokio_io_utility::{read_exact_to_bytes, read_to_bytes_rng, ready};

const BUFFER_LEN: usize = 4096;

#[derive(Debug)]
pub(super) struct ReaderBuffered<R> {
    reader: R,

    /// Use `BytesMut` here to avoid frequent copy when consuming.
    ///
    /// If we use `Vec<u8>` here, then every consume would have to
    /// copy the buffer back to the start, which is very inefficient.
    ///
    /// `BytesMut` avoids this as consuming is simply bumping the start
    /// counter, and the content is still stored continously.
    buffer: BytesMut,
}

impl<R: AsyncRead + Unpin> ReaderBuffered<R> {
    pub(super) fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: BytesMut::with_capacity(BUFFER_LEN),
        }
    }

    pub(super) async fn read_exact_into_buffer(&mut self, size: usize) -> io::Result<Drain<'_>> {
        let len = self.buffer.len();

        if len < size {
            if size < BUFFER_LEN {
                read_to_bytes_rng(&mut self.reader, &mut self.buffer, size..BUFFER_LEN).await?;
            } else {
                read_exact_to_bytes(&mut self.reader, &mut self.buffer, size - len).await?;
            }
        }

        Ok(Drain {
            buffer: &mut self.buffer,
            n: size,
        })
    }
}

impl<R: AsyncRead + Unpin> AsyncBufRead for ReaderBuffered<R> {
    fn poll_fill_buf(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        let this = &mut *self;

        let buffer = &mut this.buffer;
        let reader = &mut this.reader;

        // If we've reached the end of our internal buffer then we need to fetch
        // some more data from the underlying reader.
        // Branch using `>=` instead of the more correct `==`
        // to tell the compiler that the pos..cap slice is always valid.
        if buffer.is_empty() {
            let mut future = read_to_bytes_rng(reader, buffer, ..BUFFER_LEN);
            let future = Pin::new(&mut future);
            match ready!(future.poll(cx)) {
                Ok(()) => (),
                Err(error) => match error.kind() {
                    io::ErrorKind::UnexpectedEof => (),
                    _ => return Poll::Ready(Err(error)),
                },
            }
        }

        Poll::Ready(Ok(&Pin::into_inner(self).buffer))
    }

    fn consume(mut self: Pin<&mut Self>, amt: usize) {
        let buffer = &mut self.buffer;

        let len = buffer.len();
        let amt = min(len, amt);

        buffer.advance(amt);
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for ReaderBuffered<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let this = &mut *self;

        let buffer = &mut this.buffer;
        let reader = &mut this.reader;

        // If we don't have any buffered data and we're doing a massive read
        // (larger than our internal buffer), bypass our internal buffer
        // entirely.
        if buffer.is_empty() && buf.remaining() >= BUFFER_LEN {
            let res = ready!(Pin::new(reader).poll_read(cx, buf));
            return Poll::Ready(res);
        }

        let rem = ready!(self.as_mut().poll_fill_buf(cx))?;
        let amt = std::cmp::min(rem.len(), buf.remaining());
        buf.put_slice(&rem[..amt]);
        self.consume(amt);

        Poll::Ready(Ok(()))
    }
}

/// Similar to [`std::vec::Drain`], but can be safely [`mem::forget`]
/// or create a sub[`Drain`].
#[derive(Debug)]
pub(super) struct Drain<'a> {
    buffer: &'a mut BytesMut,
    /// Number of bytes to remove on drop
    n: usize,
}

impl Drain<'_> {
    /// Create a new `Drain` that contains `0..min(self.n, n)`.
    pub(super) fn subdrain(mut self, n: usize) -> Self {
        self.n = min(self.n, n);
        self
    }
}

impl Deref for Drain<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.buffer[..self.n]
    }
}

impl Drop for Drain<'_> {
    fn drop(&mut self) {
        self.buffer.advance(self.n);
    }
}