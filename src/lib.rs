mod error;
mod arguments;
mod input;
mod filter;
mod filters;
mod table;
mod column;

use clap::ArgMatches;

pub use error::{
    Result,
    Error,
};
use arguments::arguments;
use input::{
    Gear,
    Item,
};
use filters::{
    Filter,
    IsInGroups,
    IsInDistances,
    IsInTemperatures,
};
use table::Table;


/*----------------------------------------------------------------------------*/
fn build_filters(arguments: &ArgMatches<'_>) -> Vec<Filter>
{
    let mut filters = Vec::new();

    if let Some(groups) = arguments.values_of("groups")
    {
        filters.push(Filter::IsInGroups(IsInGroups::from(groups)));
    }

    if let Some(distances) = arguments.values_of("distances")
    {
        filters.push(Filter::IsInDistances(IsInDistances::from(distances)));
    }

    if let Some(temperatures) = arguments.values_of("temperatures")
    {
        filters.push(Filter::IsInTemperatures(IsInTemperatures::from(temperatures)))
    }

    filters
}


/*----------------------------------------------------------------------------*/
fn filtered(item: &Item,
            filters: &Vec<Filter>) -> bool
{
    use crate::filter::Filter;
    filters.iter().all(|filter| filter.filter(item))
}


/*----------------------------------------------------------------------------*/
fn filtered_gear<'a>(gear: &'a Gear,
                     arguments: &ArgMatches<'_>) -> Vec<&'a Item>
{
    let filters = build_filters(arguments);
    let mut results = Vec::new();
    let is_all = arguments.is_present("all");
    if is_all || arguments.is_present("base")
    {
        results.extend(gear.base()
                           .iter()
                           .filter(|i| filtered(i, &filters)));
    }

    if is_all || arguments.is_present("consumables")
    {
        results.extend(gear.consumables()
                           .iter()
                           .filter(|i| filtered(i, &filters)));
    }

    results
}


/*----------------------------------------------------------------------------*/
pub fn main() -> Result<()>
{
    let arguments = arguments();
    let gear = Gear::from_toml(arguments.value_of("path")
                                        .unwrap_or("gear.toml"))?;
    let result = filtered_gear(&gear, &arguments);

    if result.is_empty()
    {
        println!("No results found");
    }
    else
    {
        let headers =
            {
                let mut headers = Vec::with_capacity(Item::FIELDS.len());
                match arguments.values_of("columns")
                {
                    Some(columns) => headers.extend(columns),
                    None => headers.extend_from_slice(&Item::FIELDS),
                }

                headers
            };
        println!("{}", Table::new(headers, &result));
        match result.len()
        {
            1 => println!("1 result found"),
            len => println!("{} results found", len),
        }
    }

    Ok(())
}
