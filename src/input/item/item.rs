use std::{
    cmp::Ordering,
    cell::Cell,
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
#[derive(Deserialize)]
pub struct Item
{
    kind: String,
    name: Option<String>,
    group: Option<String>,
    weight: Option<i32>,
    price: Option<f32>,
    distances: HashSet<String>,
    temperatures: HashSet<String>,

    #[serde(skip)]
    ordered_distances: Cell<Option<Vec<String>>>,
    #[serde(skip)]
    ordered_temperatures: Cell<Option<Vec<String>>>,
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
    pub const fn default_field() -> &'static str
    {
        Self::FIELDS[1]
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn group(&self) -> Option<&str>
    {
        self.group.as_ref().map(String::as_str)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn weight(&self) -> i32
    {
        self.weight.unwrap_or_default()
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn price(&self) -> f32
    {
        self.price.unwrap_or_default()
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
    pub(super) fn column(&self, column: &str) -> Column
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
                "distances" =>
                {
                    let distances =
                        if self.distances.is_empty()
                        {
                            None
                        }
                        else
                        {
                            Some(self.ordered_distances().join(" / "))
                        };

                    (Left, distances)
                },
                "temperatures" =>
                {
                    let temperatures =
                        if self.temperatures.is_empty()
                        {
                            None
                        }
                        else
                        {
                            Some(self.ordered_temperatures().join(" / "))
                        };

                    (Left, temperatures)
                },
                _ => unreachable!(),
            };

        Column::new(alignment, content.into())
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn comparer_by(field: &str) -> fn(&&Self, &&Self) -> Ordering
    {
        match field
        {
            "kind" => compare_by_kind,
            "name" => compare_by_name,
            "group" => compare_by_group,
            "weight" => compare_by_weight,
            "price" => compare_by_price,
            "distances" => compare_by_distances,
            "temperatures" => compare_by_temperatures,
            _ => unreachable!(),
        }
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn _ordered_distances(&self) -> &Option<Vec<String>>
    {
        unsafe
        {
            self.ordered_distances.as_ptr().as_ref().unwrap()
        }
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn ordered_distances(&self) -> &Vec<String>
    {
        match self._ordered_distances()
        {
            Some(ordered_distances) => ordered_distances,
            None =>
            {
                let ordered_distances = order_hash_set(&self.distances);
                self.ordered_distances.replace(Some(ordered_distances));
                self._ordered_distances().as_ref().unwrap()
            }
        }
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn _ordered_temperatures(&self) -> &Option<Vec<String>>
    {
        unsafe
        {
            self.ordered_temperatures.as_ptr().as_ref().unwrap()
        }
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn ordered_temperatures(&self) -> &Vec<String>
    {
        match self._ordered_temperatures()
        {
            Some(ordered_temperatures) => ordered_temperatures,
            None =>
            {
                let ordered_temperatures = order_hash_set(&self.temperatures);
                self.ordered_temperatures.replace(Some(ordered_temperatures));
                self._ordered_temperatures().as_ref().unwrap()
            }
        }
    }
}


/*----------------------------------------------------------------------------*/
fn order_hash_set(hash_set: &HashSet<String>) -> Vec<String>
{
    if hash_set.is_empty()
    {
        Vec::new()
    }
    else
    {
        let mut ordered = hash_set.iter()
                                  .map(Clone::clone)
                                  .collect::<Vec<String>>();
        ordered.sort_unstable();
        ordered
    }
}


/*----------------------------------------------------------------------------*/
fn compare_by_kind(left: &&Item,
                   right: &&Item) -> Ordering
{
    left.kind.cmp(&right.kind)
}


/*----------------------------------------------------------------------------*/
fn compare_by_name(left: &&Item,
                   right: &&Item) -> Ordering
{
    left.name.cmp(&right.name)
}


/*----------------------------------------------------------------------------*/
fn compare_by_group(left: &&Item,
                    right: &&Item) -> Ordering
{
    left.group.cmp(&right.group)
}


/*----------------------------------------------------------------------------*/
fn compare_by_weight(left: &&Item,
                     right: &&Item) -> Ordering
{
    left.weight.cmp(&right.weight)
}


/*----------------------------------------------------------------------------*/
fn compare_by_price(left: &&Item,
                    right: &&Item) -> Ordering
{
    left.price.partial_cmp(&right.price).unwrap_or(Ordering::Equal)
}


/*----------------------------------------------------------------------------*/
fn compare_by_distances(left: &&Item,
                        right: &&Item) -> Ordering
{
    let left = left.ordered_distances();
    let right = right.ordered_distances();
    left.cmp(&right)
}


/*----------------------------------------------------------------------------*/
fn compare_by_temperatures(left: &&Item,
                           right: &&Item) -> Ordering
{
    let left = left.ordered_temperatures();
    let right = right.ordered_temperatures();
    left.cmp(&right)
}
