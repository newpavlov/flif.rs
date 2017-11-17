use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "error reading from stream: {}", _0)] Io(#[cause] io::Error),
    #[fail(display = "Flif header was invalid: {}", _0)] InvalidHeader(&'static str),
    #[fail(display = "Unknown critical metadata header encountered: {:?}", _0)]
    UnknownCriticalMetadata([u8; 4]),
    #[fail(display = "unknown required metadata section with byte header: {}", _0)]
    UnknownRequiredMetadata(u8),
    #[fail(display = "error inflating compressed metadata: {}", _0)] Deflate(String),
    #[fail(display = "varint was too large to store in the given datatype")] InvalidVarint,
    #[fail(display = "{}", _0)] Unimplemented(&'static str),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
