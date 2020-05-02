use std::collections::HashSet;

use crate::{
    input::Item,
    filter::Filter,
};


/*----------------------------------------------------------------------------*/
pub struct IsInTemperatures
{
    temperatures: HashSet<String>
}


/*----------------------------------------------------------------------------*/
impl<'a, I> From<I> for IsInTemperatures
    where I: Iterator<Item=&'a str>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(temperatures: I) -> Self
    {
        Self { temperatures: temperatures.map(String::from).collect() }
    }
}


/*----------------------------------------------------------------------------*/
impl Filter for IsInTemperatures
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn filter(&self, item: &Item) -> bool
    {
        self.temperatures.iter().any(|temperature| item.is_temperature(temperature))
    }
}
