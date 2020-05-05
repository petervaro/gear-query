use serde::Deserialize;

use crate::{
    input::formatters::Unit,
    validators::{
        GroupValues,
        DistancesValues,
        TemperaturesValues,
    },
};


/*----------------------------------------------------------------------------*/
pub struct Formatters<'a>
{
    pub weight: &'a Unit,
    pub price: &'a Unit,
}


/*----------------------------------------------------------------------------*/
#[derive(Deserialize)]
pub struct Meta
{
    weight: Unit,
    price: Unit,
    groups: Option<GroupValues>,
    distances: Option<DistancesValues>,
    temperatures: Option<TemperaturesValues>,
}


/*----------------------------------------------------------------------------*/
impl Meta
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn weight(&self) -> &Unit
    {
        &self.weight
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn price(&self) -> &Unit
    {
        &self.price
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn groups(&self) -> Option<&GroupValues>
    {
        self.groups.as_ref().map_or(
            None,
            |groups|
                if groups.is_empty() { None }
                else { Some(groups) })
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn distances(&self) -> Option<&DistancesValues>
    {
        self.distances.as_ref().map_or(
            None,
            |distances|
                if distances.is_empty() { None }
                else { Some(distances) })
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn temperatures(&self) -> Option<&TemperaturesValues>
    {
        self.temperatures.as_ref().map_or(
            None,
            |temperatures|
                if temperatures.is_empty() { None }
                else { Some(temperatures) })
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn formatters(&self) -> Formatters<'_>
    {
        Formatters
        {
            weight: &self.weight,
            price: &self.price,
        }
    }
}
