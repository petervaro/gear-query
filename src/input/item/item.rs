use std::{
    ops::Deref,
    collections::HashSet,
};

use serde::Deserialize;

use crate::{
    column::{
        Column,
        Alignment,
    },
    input::item::Columns,
};


/*----------------------------------------------------------------------------*/
#[derive(Debug, Deserialize)]
pub struct Item
{
    kind: String,
    name: Option<String>,
    group: Option<String>,
    weight: Option<i32>,
    price: Option<f64>,
    distances: HashSet<String>,
    temperatures: HashSet<String>,
}


/*----------------------------------------------------------------------------*/
impl Item
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub const FIELDS: [&'static str; 7] = ["kind",
                                           "name",
                                           "group",
                                           "weight",
                                           "price",
                                           "distances",
                                           "temperatures"];

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn group(&self) -> Option<&str>
    {
        self.group.as_ref().map(String::as_str)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn is_distance(&self, distance: &str) -> bool
    {
        self.distances.contains(distance)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn is_temperature(&self, temperature: &str) -> bool
    {
        self.temperatures.contains(temperature)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn columns<'a>(&'a self, columns: &'a [&'a str]) -> Columns<'a>
    {
        Columns::new(self, columns)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn column(&self, column: &str) -> Column
    {
        use Alignment::*;
        let (alignment, content) =
            match column
            {
                "kind" => (Left, Some(self.kind.clone())),
                "name" => (Left, self.name.as_ref().map(String::clone)),
                "group" => (Left, self.group.as_ref().map(String::clone)),
                "weight" => (Right, self.weight.map(|w| format!("{}g", w))),
                "price" => (Right, self.price.map(|p| format!("\u{00A3}{:.2}", p))),
                "distances" => (Left, join_hash_set(&self.distances, " / ")),
                "temperatures" => (Left, join_hash_set(&self.temperatures, " / ")),
                _ => panic!("Invalid column: {}", column),
            };

        Column::new(alignment, content.into())
    }
}


/*----------------------------------------------------------------------------*/
fn join_hash_set(items: &HashSet<String>, separator: &str) -> Option<String>
{
    if items.is_empty()
    {
        None
    }
    else
    {
        let mut items = items.iter().collect::<Vec<&String>>();
        items.sort_unstable();
        let mut items = items.iter().map(Deref::deref);
        let mut joined = String::from(items.next().unwrap());
        for item in items
        {
            joined.push_str(separator);
            joined.push_str(item);
        }

        Some(joined)
    }
}
