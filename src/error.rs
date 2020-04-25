use std::{
    io,
    error,
    fmt::{
        self,
        Display,
        Formatter,
    },
};

use toml::de;


/*----------------------------------------------------------------------------*/
pub type Result<T> = ::std::result::Result<T, Error>;


/*----------------------------------------------------------------------------*/
#[derive(Debug)]
pub enum Error
{
    IoError(io::Error),
    TomlDeError(de::Error)
}


/*----------------------------------------------------------------------------*/
impl error::Error for Error {}


/*----------------------------------------------------------------------------*/
impl Display for Error
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        use Error::*;
        match self
        {
            IoError(error) => write!(f, "{}", error),
            TomlDeError(error) => write!(f, "{}", error),
        }
    }
}


/*----------------------------------------------------------------------------*/
impl From<io::Error> for Error
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(error: io::Error) -> Self
    {
        Self::IoError(error)
    }
}


/*----------------------------------------------------------------------------*/
impl From<de::Error> for Error
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(error: de::Error) -> Self
    {
        Self::TomlDeError(error)
    }
}
