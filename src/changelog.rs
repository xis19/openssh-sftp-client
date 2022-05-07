#[allow(unused_imports)]
use crate::*;

/// ## Added
///  - Export mod [`highlevel::file`]
///  - Export mod [`highlevel::fs`]
///  - Export mod [`highlevel::metadata`]
///
/// ## Changed
///  - [`lowlevel::WriteEnd`] now requires `W: AsyncWrite + Unpin`
///  - [`lowlevel::SharedData`] now requires `W: AsyncWrite + Unpin`
///  - [`lowlevel::ReadEnd`] now requires `W: AsyncWrite + Unpin`
///  - [`lowlevel::connect`] now requires `W: AsyncWrite + Unpin`
///  - [`lowlevel::connect_with_auxiliary`] now requires `W: AsyncWrite + Unpin`
///  - All types in [`highlevel`] now requires `W: AsyncWrite + Unpin`
///    except for
///     - the re-exported type [`highlevel::CancellationToken`]
///     - [`highlevel::SftpOptions`]
///     - [`highlevel::fs::DirEntry`]
///     - [`highlevel::fs::ReadDir`]
///
/// ## Removed
///  - Trait `Writer`.
///  - `lowlevel::WriteEnd::send_write_request_direct_atomic`
///  - `lowlevel::WriteEnd::send_write_request_direct_atomic_vectored`
///  - `lowlevel::WriteEnd::send_write_request_direct_atomic_vectored2`
///  - export of `highlevel::file::TokioCompactFile`
///  - export of `highlevel::file::TokioCompatFile`
///  - export of `highlevel::file::DEFAULT_BUFLEN`
///  - export of `highlevel::file::DEFAULT_MAX_BUFLEN`
///  - export of `highlevel::file::File`
///  - export of `highlevel::file::OpenOptions`
///  - export of `highlevel::fs::DirEntry`
///  - export of `highlevel::fs::ReadDir`
///  - export of `highlevel::fs::Dir`
///  - export of `highlevel::fs::DirBuilder`
///  - export of `highlevel::fs::Fs`
#[doc(hidden)]
pub mod unreleased {}

/// ## Added
///  - Type [`highlevel::TokioCompatFile`] to Replace
///    [`highlevel::TokioCompactFile`].
pub mod v0_8_3 {}

/// ## Fixed
///  - Fix possible panic in [`highlevel::max_atomic_write_len`]
pub mod v0_8_2 {}

/// ## Added
///  - Reexport [`highlevel::CancellationToken`].
pub mod v0_8_1 {}

/// ## Added
///  - Associated function [`highlevel::FileType::is_fifo`].
///  - Associated function [`highlevel::FileType::is_socket`].
///  - Associated function [`highlevel::FileType::is_block_device`].
///  - Associated function [`highlevel::FileType::is_char_device`].
///  - Trait [`Writer`].
///
/// ## Changed
///  - Replace all use of [`tokio_pipe::PipeRead`] with generic bound
///    [`tokio::io::AsyncRead`] + [`Unpin`].
///  - Replace all use of [`tokio_pipe::PipeWrite`] with generic bound
///    [`Writer`].
///  - Replace constant `highlevel::MAX_ATOMIC_WRITE_LEN` with
///    non-`const` function [`highlevel::max_atomic_write_len`].
///  - Associated function [`highlevel::Sftp::fs`] now only takes `&self`
///    as parameter.
///
/// ## Removed
///  - Trait [`std::os::unix::fs::FileTypeExt`] implementation for
///    [`highlevel::FileType`].
///  - Trait [`std::os::unix::fs::PermissionsExt`] implementation for
///    [`highlevel::Permissions`].
///  - Associated function `lowlevel::WriteEnd::send_write_request_direct`.
///  - Associated function
///    `lowlevel::WriteEnd::send_write_request_direct_vectored`.
pub mod v0_8_0 {}
