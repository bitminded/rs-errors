use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LinuxSystemError
{
    Unknown,
    Code(u32)
}

impl super::SystemErrorLike for LinuxSystemError {}

impl Error for LinuxSystemError {}

impl fmt::Display for LinuxSystemError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            LinuxSystemError::Unknown =>
            {
                write!(f, "Linux system error: unknown")
            },
            LinuxSystemError::Code(value) =>
            {
                write!(f, "Linux system error: code {}", value)
            }
        }
    }
}