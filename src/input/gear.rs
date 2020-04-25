use std::fs::read_to_string;

use toml::from_str;

use serde::Deserialize;

use crate::input::{
    Meta,
    Item,
    Group,
};


/*----------------------------------------------------------------------------*/
#[derive(Deserialize)]
pub struct Gear
{
    meta: Meta,
    base: Vec<Item>,
    consumables: Vec<Item>,
    groups: Vec<Group>,
}


/*----------------------------------------------------------------------------*/
impl Gear
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn from_toml(file_name: &str) -> crate::Result<Self>
    {
        let input = read_to_string(file_name)?;
        let input = from_str(&input)?;
        Ok(input)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn meta(&self) -> &Meta
    {
        &self.meta
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn base(&self) -> &Vec<Item>
    {
        &self.base
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn consumables(&self) -> &Vec<Item>
    {
        &self.consumables
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn groups(&self) -> &Vec<Group>
    {
        &self.groups
    }
}
