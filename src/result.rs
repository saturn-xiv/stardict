use std::result::Result as StdResult;

use failure::{Error as FailureError, Fail};

pub type Result<T> = StdResult<T, FailureError>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Io(#[fail(cause)] std::io::Error),
    #[fail(display = "{}", _0)]
    Utf8(#[fail(cause)] std::str::Utf8Error),
}
