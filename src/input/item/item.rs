use std::{
    cmp::Ordering,
    cell::Cell,
    collections::HashSet,
};

use serde::Deserialize;

use crate::{
    filter::Filter,
    filters,
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
    fn inner_of(cell: &Cell<Option<Vec<String>>>) -> &Option<Vec<String>>
    {
        unsafe { cell.as_ptr().as_ref().unwrap() }
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn ordered_distances(&self) -> &Vec<String>
    {
        Self::inner_of(&self.ordered_distances).as_ref().unwrap_or_else(
            ||
            {
                let ordered_distances = order_hash_set(&self.distances);
                self.ordered_distances.replace(Some(ordered_distances));
                Self::inner_of(&self.ordered_distances).as_ref().unwrap()
            })
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn ordered_temperatures(&self) -> &Vec<String>
    {
        Self::inner_of(&self.ordered_temperatures).as_ref().unwrap_or_else(
            ||
            {
                let ordered_temperatures = order_hash_set(&self.temperatures);
                self.ordered_temperatures.replace(Some(ordered_temperatures));
                Self::inner_of(&self.ordered_temperatures).as_ref().unwrap()
            })
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn filter(&self, filters: &Vec<filters::Filter>) -> bool
    {
        filters.iter().all(|filter| filter.filter(self))
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn comparer_by(field: &str) -> fn(&Self, &Self) -> Ordering
    {
        match field
        {
            "kind" => Self::compare_by_kind,
            "name" => Self::compare_by_name,
            "group" => Self::compare_by_group,
            "weight" => Self::compare_by_weight,
            "price" => Self::compare_by_price,
            "distances" => Self::compare_by_distances,
            "temperatures" => Self::compare_by_temperatures,
            _ => unreachable!(),
        }
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn compare_by_kind(&self, other: &Self) -> Ordering
    {
        self.kind.cmp(&other.kind)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn compare_by_name(&self, other: &Self) -> Ordering
    {
        self.name.cmp(&other.name)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn compare_by_group(&self, other: &Self) -> Ordering
    {
        self.group.cmp(&other.group)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn compare_by_weight(&self, other: &Self) -> Ordering
    {
        self.weight.cmp(&other.weight)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn compare_by_price(&self, other: &Self) -> Ordering
    {
        self.price.partial_cmp(&other.price).unwrap_or(Ordering::Equal)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn compare_by_distances(&self, other: &Self) -> Ordering
    {
        self.ordered_distances().cmp(other.ordered_distances())
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn compare_by_temperatures(&self, other: &Self) -> Ordering
    {
        self.ordered_temperatures().cmp(other.ordered_temperatures())
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
