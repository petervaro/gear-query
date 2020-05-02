use std::fmt::{
    self,
    Display,
    Formatter,
};

use crate::column::{
    content::Content,
    aligned::{
        Alignment,
        LeftAlignedFittedColumn,
        CentreAlignedFittedColumn,
        RightAlignedFittedColumn,
    },
};


/*----------------------------------------------------------------------------*/
enum AlignedFitted<'a>
{
    Left(LeftAlignedFittedColumn<'a>),
    Centre(CentreAlignedFittedColumn<'a>),
    Right(RightAlignedFittedColumn<'a>),
}


/*----------------------------------------------------------------------------*/
pub struct FittedColumn<'a>(AlignedFitted<'a>);


/*----------------------------------------------------------------------------*/
impl<'a> FittedColumn<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new(alignment: Alignment,
               content: &'a Content,
               width: usize) -> Self
    {
        use AlignedFitted::*;
        let fitted =
            match alignment
            {
                Alignment::Left =>
                    Left(LeftAlignedFittedColumn::new(content, width)),
                Alignment::Centre =>
                    Centre(CentreAlignedFittedColumn::new(content, width)),
                Alignment::Right =>
                    Right(RightAlignedFittedColumn::new(content, width)),
            };

        Self(fitted)
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> Display for FittedColumn<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        use AlignedFitted::*;
        match &self.0
        {
            Left(column) => write!(f, "{}", column),
            Centre(column) => write!(f, "{}", column),
            Right(column) => write!(f, "{}", column),
        }
    }
}
