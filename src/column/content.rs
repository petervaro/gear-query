use std::fmt::{
    self,
    Display,
    Formatter,
};


/*----------------------------------------------------------------------------*/
pub struct Content(Option<String>);


/*----------------------------------------------------------------------------*/
impl Content
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn width(&self) -> usize
    {
        self.0.as_ref().map_or(0, |inner| inner.chars().count())
    }
}


/*----------------------------------------------------------------------------*/
impl From<Option<String>> for Content
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(content: Option<String>) -> Self
    {
        Self(content)
    }
}


/*----------------------------------------------------------------------------*/
impl Display for Content
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        if let Some(inner) = self.0.as_ref()
        {
            write!(f, "{}", inner)
        }
        else
        {
            Ok(())
        }
    }
}
