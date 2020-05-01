use std::fs::read_to_string;

use toml::from_str;

use serde::Deserialize;

use crate::{
    input::{
        Meta,
        Item,
        Group,
    },
    filters::{
        Filter,
        IsInGroups,
        IsInDistances,
        IsInTemperatures,
    },
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

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn filters<'f, I>(groups: Option<I>,
                      distances: Option<I>,
                      temperatures: Option<I>) -> Vec<Filter>
        where I: Iterator<Item = &'f str>
    {
        let mut filters = Vec::new();

        if let Some(groups) = groups
        {
            filters.push(Filter::IsInGroups(IsInGroups::from(groups)));
        }

        if let Some(distances) = distances
        {
            filters.push(Filter::IsInDistances(IsInDistances::from(distances)));
        }

        if let Some(temperatures) = temperatures
        {
            filters.push(Filter::IsInTemperatures(IsInTemperatures::from(temperatures)))
        }

        filters
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn filter<'f, I>(&self, is_all: bool,
                                is_base: bool,
                                is_consumables: bool,
                                groups: Option<I>,
                                distances: Option<I>,
                                temperatures: Option<I>) -> Vec<&'_ Item>
        where I: Iterator<Item = &'f str>
    {
        let filters = Self::filters(groups, distances, temperatures);
        let mut results = Vec::new();
        if is_all || is_base
        {
            results.extend(self.base()
                               .iter()
                               .filter(|item| item.filter(&filters)));
        }

        if is_all || is_consumables
        {
            results.extend(self.consumables()
                               .iter()
                               .filter(|item| item.filter(&filters)));
        }

        results
    }
}
