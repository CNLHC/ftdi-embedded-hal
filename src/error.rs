use ftdi;
use libftd2xx;
use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, FtHalError>;

#[derive(Debug)]
pub enum FtHalError {
    HAL(ErrorKind),
    Io(io::Error),
    FTDI(ftdi::Error),
    FTD2XX(libftd2xx::TimeoutError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    InvalidParams,
    InvalidClock,
    BusBusy,
    I2cNoAck,
    GpioPinBusy,
    GpioInvalidPin,
    SpiModeNotSupported,
}

impl ErrorKind {
    fn as_str(&self) -> &str {
        match *self {
            ErrorKind::InvalidParams => "Invalid input params",
            ErrorKind::BusBusy => "Bus is busy",
            ErrorKind::InvalidClock => "Clock is not valid",
            ErrorKind::I2cNoAck => "No ACK from slave",
            ErrorKind::GpioPinBusy => "GPIO pin is already in use",
            ErrorKind::GpioInvalidPin => "No such GPIO pin",
            ErrorKind::SpiModeNotSupported => "Mode not supported",
        }
    }
}

impl fmt::Display for FtHalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FtHalError::Io(ref err) => err.fmt(f),
            FtHalError::FTDI(ref err) => err.fmt(f),
            FtHalError::FTD2XX(ref err) => err.fmt(f),
            FtHalError::HAL(ref err) => write!(f, "A regular error occurred {:?}", err.as_str()),
        }
    }
}

impl From<io::Error> for FtHalError {
    fn from(e: io::Error) -> Self {
        FtHalError::Io(e)
    }
}

impl From<ftdi::Error> for FtHalError {
    fn from(e: ftdi::Error) -> Self {
        FtHalError::FTDI(e)
    }
}

impl From<libftd2xx::TimeoutError> for FtHalError{
    fn from(e: libftd2xx::TimeoutError) -> Self {
        FtHalError::FTD2XX(e)
    }
}
