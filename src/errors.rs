use std::io;
use std::num;

#[derive(Debug)]
pub enum RouteError {
    Io(io::Error),
    Parse(num::ParseIntError),
    BadInput
}

impl From<io::Error> for RouteError {
    fn from(err: io::Error) -> RouteError {
        RouteError::Io(err)
    }
}

impl From<num::ParseIntError> for RouteError {
    fn from(err: num::ParseIntError) -> RouteError {
        RouteError::Parse(err)
    }
}

