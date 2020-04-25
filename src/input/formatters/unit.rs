use std::fmt::{
    Display,
    Write,
};

use serde::Deserialize;


/*----------------------------------------------------------------------------*/
#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum Position
{
    Prefix,
    Suffix,
}


/*----------------------------------------------------------------------------*/
#[derive(Deserialize)]
pub struct Unit
{
    symbol: String,
    position: Position,
}


/*----------------------------------------------------------------------------*/
impl Unit
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn format_to_string<T>(&self, value: T,
                                      buffer: &mut String)
        where T: Display
    {
        use Position::*;
        match self.position
        {
            Prefix => write!(buffer, "{}{}", self.symbol, value).unwrap(),
            Suffix => write!(buffer, "{}{}", value, self.symbol).unwrap(),
        }
    }
}
