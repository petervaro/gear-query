use std::fmt::{
    self,
    Display,
    Formatter,
};

use crate::column::content::Content;


/*----------------------------------------------------------------------------*/
pub struct LeftAlignedFittedColumn<'a>
{
    content: &'a Content,
    width: usize,
}


/*----------------------------------------------------------------------------*/
impl<'a> LeftAlignedFittedColumn<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new(content: &'a Content,
               width: usize) -> Self
    {
        Self { content, width }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> Display for LeftAlignedFittedColumn<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.content)?;

        debug_assert!(self.width >= self.content.width());
        for _ in 0..self.width - self.content.width()
        {
            write!(f, " ")?;
        }

        Ok(())
    }
}
