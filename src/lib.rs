use std::error::Error;
use std::fmt;

#[cfg(target_os = "windows")]
mod win32;

#[cfg(target_os = "linux")]
mod linux;

trait SystemErrorLike: Error + fmt::Display + fmt::Debug {}

pub struct SystemError
{
    error: Box<dyn SystemErrorLike>
}

impl SystemError
{
    pub fn create_from_code(code: u32) -> Self
    {
        #[cfg(target_os = "windows")]
        return SystemError {
            error: Box::new(win32::WindowsSystemError::Code(code))
        };

        #[cfg(target_os = "linux")]
        return SystemError {
            error: Box::new(linux::LinuxSystemError::Code(code))
        };
    }

    pub fn create_unknown() -> Self
    {
        #[cfg(target_os = "windows")]
        return SystemError {
            error: Box::new(win32::WindowsSystemError::Unknown)
        };

        #[cfg(target_os = "linux")]
        return SystemError {
            error: Box::new(linux::LinuxSystemError::Unknown)
        };
    }
}

impl Error for SystemError
{
    fn source(&self) -> Option<&(dyn Error + 'static )>
    {
        self.error.source()
    }
}

impl fmt::Display for SystemError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        self.error.fmt(f)
    }
}

impl fmt::Debug for SystemError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        self.error.fmt(f)
    }
}

#[derive(Debug)]
pub struct InvalidValueError<E, A>
{
    expected: E,
    actual: A
}

impl<E, A> Error for InvalidValueError<E, A>
    where E: fmt::Debug + fmt::Display, A: fmt::Debug + fmt::Display {}

impl<E, A> fmt::Display for InvalidValueError<E, A>
    where E: fmt::Debug + fmt::Display, A: fmt::Debug + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Invalid value. Expected {}. Found {}", self.expected, self.actual)
    }
}