use std::collections::HashSet;

use crate::{
    input::Item,
    filter::Filter,
};


/*----------------------------------------------------------------------------*/
pub struct IsInGroups
{
    groups: HashSet<String>
}


/*----------------------------------------------------------------------------*/
impl<'a, I> From<I> for IsInGroups
    where I: Iterator<Item=&'a str>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(groups: I) -> Self
    {
        Self { groups: groups.map(String::from).collect() }
    }
}


/*----------------------------------------------------------------------------*/
impl Filter for IsInGroups
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn filter(&self, item: &Item) -> bool
    {
        item.group().map_or(false, |group| self.groups.contains(group))
    }
}
