use std::fmt::{
    self,
    Display,
    Formatter,
};

use crate::column::content::Content;


/*----------------------------------------------------------------------------*/
pub struct RightAlignedFittedColumn<'a>
{
    content: &'a Content,
    width: usize,
}


/*----------------------------------------------------------------------------*/
impl<'a> RightAlignedFittedColumn<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new(content: &'a Content,
               width: usize) -> Self
    {
        Self { content, width }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> Display for RightAlignedFittedColumn<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        debug_assert!(self.width >= self.content.width());
        for _ in 0..self.width - self.content.width()
        {
            write!(f, " ")?;
        }

        write!(f, "{}", self.content)?;

        Ok(())
    }
}
