use super::{lowlevel, Error};

pub use lowlevel::{UnixTimeStamp, UnixTimeStampError};

use bytes::BytesMut;

mod sftp;
pub use sftp::Sftp;

mod cancel_utility;
use cancel_utility::BoxedWaitForCancellationFuture;

mod options;
pub use options::SftpOptions;

mod tasks;

mod auxiliary;
use auxiliary::Auxiliary;

mod cache;
use cache::WriteEndWithCachedId;

mod handle;
use handle::OwnedHandle;

mod file;
pub use file::TokioCompactFile;
pub use file::{File, OpenOptions};

mod fs;
pub use fs::DirEntry;
pub use fs::ReadDir;
pub use fs::{Dir, DirBuilder, Fs};

mod metadata;
pub use metadata::{FileType, MetaData, MetaDataBuilder, Permissions};

type Buffer = BytesMut;

type WriteEnd = lowlevel::WriteEnd<Buffer, Auxiliary>;
type ReadEnd = lowlevel::ReadEnd<Buffer, Auxiliary>;
type SharedData = lowlevel::SharedData<Buffer, Auxiliary>;
type Id = lowlevel::Id<Buffer>;
type Data = lowlevel::Data<Buffer>;