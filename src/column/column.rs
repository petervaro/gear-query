use crate::column::{
    Alignment,
    content::Content,
    fitted::FittedColumn,
};


/*----------------------------------------------------------------------------*/
pub struct Column
{
    alignment: Alignment,
    content: Content,
}


/*----------------------------------------------------------------------------*/
impl Column
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new(alignment: Alignment,
               content: Content) -> Self
    {
        Self { alignment, content }
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn width(&self) -> usize
    {
        self.content.width()
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn as_fitted(&self, width: usize) -> FittedColumn<'_>
    {
        FittedColumn::new(self.alignment, &self.content, width)
    }
}
