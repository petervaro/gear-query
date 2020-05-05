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
    input::{
        Formatters,
        item::Columns,
    },
    validators::Validity,
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

    distances: Option<HashSet<String>>,
    temperatures: Option<HashSet<String>>,

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
    pub fn id(&self) -> &str
    {
        self.name.as_ref().unwrap_or(&self.kind).as_str()
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn group(&self) -> Option<&str>
    {
        self.group.as_ref().map(String::as_str)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn group_validity<'f>(
        &'f self, valid_groups: &'f HashSet<String>) -> Validity<&'f String>
    {
        use Validity::*;

        debug_assert!(!valid_groups.is_empty());

        match self.group.as_ref()
        {
            None => Missing,
            Some(group) =>
                if valid_groups.contains(group) { Valid }
                else { Invalid(group) },
        }
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
        self.distances.as_ref().map_or(
            false,
            |distances| distances.contains(distance))
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn distances_validity<'f>(
        &'f self, valid_distances: &'f HashSet<String>) -> Validity<&'f String>
    {
        use Validity::*;

        debug_assert!(!valid_distances.is_empty());

        self.distances.as_ref().map_or(
            Missing,
            |distances|
                if distances.is_empty() { Missing }
                else { distances.difference(valid_distances)
                                .next()
                                .map_or(Valid, Invalid) })
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn is_temperature(&self, temperature: &str) -> bool
    {
        self.temperatures.as_ref().map_or(
            false,
            |temperatures| temperatures.contains(temperature))
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn temperatures_validity<'f>(
        &'f self, valid_temperatures: &'f HashSet<String>) -> Validity<&'f String>
    {
        use Validity::*;

        debug_assert!(!valid_temperatures.is_empty());

        self.temperatures.as_ref().map_or(
            Missing,
            |temperatures|
                if temperatures.is_empty() { Missing }
                else { temperatures.difference(valid_temperatures)
                                   .next()
                                   .map_or(Valid, Invalid) })
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn columns<'a>(&'a self, columns: &'a [&'a str],
                                 formatters: &'a Formatters<'a>) -> Columns<'a>
    {
        Columns::new(self, columns, formatters)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub(super) fn column(&self, column: &str,
                                formatters: &Formatters<'_>) -> Column
    {
        use Alignment::*;
        let (alignment, content) =
            match column
            {
                "kind" => (Left, Some(self.kind.clone())),
                "name" => (Left, self.name.as_ref().map(String::clone)),
                "group" => (Left, self.group.as_ref().map(String::clone)),
                "weight" =>
                {
                    let weight = self.weight.map(
                        |weight|
                        {
                            let mut formatted = String::new();
                            formatters.weight.format_to_string(weight, &mut formatted);
                            formatted
                        });

                    (Right, weight)
                },
                "price" =>
                {
                    let price = self.price.map(
                        |price|
                        {
                            let mut formatted = String::new();
                            formatters.price.format_to_string(price, &mut formatted);
                            formatted
                        });

                    (Right, price)
                },
                "distances" =>
                {
                    let distances = self.distances.as_ref().map_or(
                        None,
                        |distances|
                            if distances.is_empty() { None }
                            else { Some(self.ordered_distances().join(" / ")) });

                    (Left, distances)
                },
                "temperatures" =>
                {
                    let temperatures = self.temperatures.as_ref().map_or(
                        None,
                        |temperatures|
                            if temperatures.is_empty() { None }
                            else { Some(self.ordered_temperatures().join(" / ")) });

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
                let distances = self.distances.as_ref().unwrap();
                let ordered_distances = order_hash_set(distances);
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
                let temperatures = self.temperatures.as_ref().unwrap();
                let ordered_temperatures = order_hash_set(temperatures);
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
