use std::slice::Iter;

use crate::{
    input::Item,
    column::Column,
};


/*----------------------------------------------------------------------------*/
pub struct Columns<'a>
{
    item: &'a Item,
    columns: Iter<'a, &'a str>,
}


/*----------------------------------------------------------------------------*/
impl<'a> Columns<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new(item: &'a Item, columns: &'a [&'a str]) -> Self
    {
        Self
        {
            item,
            columns: columns.iter(),
        }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> Iterator for Columns<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    type Item = Column;

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn next(&mut self) -> Option<Self::Item>
    {
        self.columns.next().map(|name| self.item.column(name))
    }
}
