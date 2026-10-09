use std::{fmt, io, path::PathBuf};
use token_db::TokenId;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Postcard(postcard::Error),
    TokenDb(token_db::Error),
    InvalidPath(PathBuf),
    InvalidTokenId(TokenId),
    MissingGlobalToken(String),
    EmptyCorpus,
}
pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Postcard(e) => write!(f, "postcard error: {e}"),
            Self::TokenDb(e) => write!(f, "token database error: {e}"),
            Self::InvalidPath(p) => write!(f, "invalid path: {}", p.display()),
            Self::InvalidTokenId(id) => write!(f, "invalid token ID: {}", id.get()),
            Self::MissingGlobalToken(s) => write!(f, "missing global token: {s}"),
            Self::EmptyCorpus => write!(f, "no input *_glob.tok files found"),
        }
    }
}
impl std::error::Error for Error {}
impl From<io::Error> for Error { fn from(e: io::Error) -> Self { Self::Io(e) } }
impl From<postcard::Error> for Error { fn from(e: postcard::Error) -> Self { Self::Postcard(e) } }
impl From<token_db::Error> for Error { fn from(e: token_db::Error) -> Self { Self::TokenDb(e) } }
