use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::net;
use std::net::Ipv6Addr;

#[derive(Debug)]
enum UpstreamError {
    IO(io::Error),
    Parsing(net::AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for UpstreamError {}

impl From<io::Error> for UpstreamError {
    fn from(e: io::Error) -> Self {
        UpstreamError::IO(e)
    }
}

impl From<net::AddrParseError> for UpstreamError {
    fn from(e: net::AddrParseError) -> Self {
        UpstreamError::Parsing(e)
    }
}

fn main() -> Result<(), UpstreamError> {
    let _f = File::open("invisible.txt")?;

    let _localhost = "invalid address::1".parse::<Ipv6Addr>()?;

    Ok(())
}
