use std::fmt::{
    self,
    Display,
    Formatter,
};

use crate::column::{
    Alignment,
    content::Content,
    left::LeftAlignedFittedColumn,
    centre::CentreAlignedFittedColumn,
    right::RightAlignedFittedColumn,
};


/*----------------------------------------------------------------------------*/
pub enum FittedColumn<'a>
{
    LeftAligned(LeftAlignedFittedColumn<'a>),
    CentreAligned(CentreAlignedFittedColumn<'a>),
    RightAligned(RightAlignedFittedColumn<'a>),
}


/*----------------------------------------------------------------------------*/
impl<'a> FittedColumn<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new(alignment: Alignment,
               content: &'a Content,
               width: usize) -> Self
    {
        use Alignment::*;
        use FittedColumn::*;
        match alignment
        {
            Left => LeftAligned(LeftAlignedFittedColumn::new(content, width)),
            Centre => CentreAligned(CentreAlignedFittedColumn::new(content, width)),
            Right => RightAligned(RightAlignedFittedColumn::new(content, width)),
        }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> Display for FittedColumn<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        use FittedColumn::*;
        match self
        {
            LeftAligned(column) => column.fmt(f),
            CentreAligned(column) => column.fmt(f),
            RightAligned(column) => column.fmt(f),
        }
    }
}
