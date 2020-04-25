use std::fmt::{
    self,
    Display,
    Formatter,
};

use crate::column::content::Content;


/*----------------------------------------------------------------------------*/
pub struct CentreAlignedFittedColumn<'a>
{
    content: &'a Content,
    width: usize,
}


/*----------------------------------------------------------------------------*/
impl<'a> CentreAlignedFittedColumn<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new(content: &'a Content,
               width: usize) -> Self
    {
        Self { content, width }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> Display for CentreAlignedFittedColumn<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        let (left, right) =
            {
                debug_assert!(self.width >= self.content.width());
                let available = self.width - self.content.width();
                let quotient = available/2;
                let remainder = available%2;

                (quotient, quotient + remainder)
            };

        for _ in 0..left
        {
            write!(f, " ")?;
        }

        write!(f, "{}", self.content)?;

        for _ in 0..right
        {
            write!(f, " ")?;
        }

        Ok(())
    }
}
