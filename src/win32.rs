use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum WindowsSystemError
{
    Unknown,
    Code(u32)
}

impl super::SystemErrorLike for WindowsSystemError {}

impl Error for WindowsSystemError {}

impl fmt::Display for WindowsSystemError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            WindowsSystemError::Unknown =>
            {
                write!(f, "Windows system error: unknown")
            },
            WindowsSystemError::Code(value) =>
            {
                write!(f, "Windows system error: code {}", value)
            }
        }
    }
}