use std::collections::HashSet;

use crate::{
    input::Item,
    filter::Filter,
};


/*----------------------------------------------------------------------------*/
pub struct IsInDistances
{
    distances: HashSet<String>
}


/*----------------------------------------------------------------------------*/
impl<'a, I> From<I> for IsInDistances
    where I: Iterator<Item=&'a str>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(distances: I) -> Self
    {
        Self { distances: distances.map(String::from).collect() }
    }
}


/*----------------------------------------------------------------------------*/
impl Filter for IsInDistances
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn filter(&self, item: &Item) -> bool
    {
        self.distances.iter().any(|d| item.is_distance(d))
    }
}
