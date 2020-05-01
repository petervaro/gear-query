mod error;
mod arguments;
mod input;
mod filter;
mod filters;
mod table;
mod column;
mod sum;

use std::ops::Deref;

pub use error::{
    Result,
    Error,
};
use arguments::arguments;
use input::{
    Gear,
    Item,
};
use table::Table;
use sum::Sum;


/*----------------------------------------------------------------------------*/
pub fn main() -> Result<()>
{
    let arguments = arguments();
    let gear = Gear::from_toml(arguments.value_of("path").unwrap())?;
    let results =
        {
            let mut results = gear.filter(arguments.is_present("all"),
                                          arguments.is_present("base"),
                                          arguments.is_present("consumables"),
                                          arguments.values_of("groups"),
                                          arguments.values_of("distances"),
                                          arguments.values_of("temperatures"));
            let comparer = Item::comparer_by(arguments.value_of("sort").unwrap());
            results.sort_unstable_by(|&left, &right| comparer(left, right));
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
