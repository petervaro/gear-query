use std::fs::read_to_string;

use toml::from_str;

use serde::Deserialize;

use crate::{
    input::{
        Meta,
        Item,
    },
    filters::Filter,
    validators::Validator,
};


/*----------------------------------------------------------------------------*/
#[derive(Deserialize)]
pub struct Gear
{
    meta: Meta,
    base: Option<Vec<Item>>,
    consumables: Option<Vec<Item>>,
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
    fn filters<'f, I>(groups: Option<I>,
                      distances: Option<I>,
                      temperatures: Option<I>) -> Vec<Filter>
        where I: Iterator<Item = &'f str>
    {
        use Filter::*;

        let mut filters = Vec::new();

        if let Some(groups) = groups
        {
            filters.push(IsInGroups(groups.into()));
        }

        if let Some(distances) = distances
        {
            filters.push(IsInDistances(distances.into()));
        }

        if let Some(temperatures) = temperatures
        {
            filters.push(IsInTemperatures(temperatures.into()))
        }

        filters
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn validators(&self) -> Vec<Validator<'_>>
    {
        let mut validators = Vec::new();

        if let Some(groups) = self.meta.groups()
        {
            validators.push(groups.into());
        }

        if let Some(distances) = self.meta.distances()
        {
            validators.push(distances.into());
        }

        if let Some(temperatures) = self.meta.temperatures()
        {
            validators.push(temperatures.into());
        }

        validators
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn filter_and_validate<'f, I>(&self, is_all: bool,
                                             is_base: bool,
                                             is_consumables: bool,
                                             groups: Option<I>,
                                             distances: Option<I>,
                                             temperatures: Option<I>)
        -> crate::Result<Vec<&'_ Item>>
        where I: Iterator<Item = &'f str>
    {
        let filters = Self::filters(groups, distances, temperatures);
        let validators = self.validators();

        let mut results = Vec::new();
        if is_all || is_base
        {
            if let Some(base) = self.base.as_ref()
            {
                for item in base.iter()
                                .filter(|item| item.filter(&filters))
                {
                    validators.iter()
                              .try_for_each(|validator| validator.validate(item))?;
                    results.push(item);
                }
            }
        }

        if is_all || is_consumables
        {
            if let Some(consumables) = self.consumables.as_ref()
            {
                for item in consumables.iter()
                                       .filter(|item| item.filter(&filters))
                {
                    validators.iter()
                              .try_for_each(|validator| validator.validate(item))?;
                    results.push(item);
                }
            }
        }

        Ok(results)
    }
}
