mod error;
mod arguments;
mod input;
mod filter;
mod filters;
mod table;
mod column;
mod sum;

use std::ops::Deref;

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
use sum::Sum;


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
    let gear = Gear::from_toml(arguments.value_of("path").unwrap())?;
    let results =
        {
            let mut results = filtered_gear(&gear, &arguments);
            let sort_by = arguments.value_of("sort").unwrap();
            results.sort_unstable_by(Item::comparer_by(sort_by));
            results
        };

    if results.is_empty()
    {
        println!("No items found");
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

        let table =
            match arguments.value_of("order").unwrap()
            {
                "ascending" =>
                    Table::new(headers, results.iter().map(Deref::deref)),
                "descending" =>
                    Table::new(headers, results.iter().rev().map(Deref::deref)),
                _ => unreachable!(),
            };

        println!("{}", table);
        match results.len()
        {
            1 => println!("1 item found"),
            len => println!("{} items found", len),
        }

        if let Some(column) = arguments.value_of("sum")
        {
            println!("{}", Sum::new(column, gear.meta(), &results));
        }
    }

    Ok(())
}
