use std::hash::{
    Hash,
    Hasher,
};

use serde::Deserialize;


/*----------------------------------------------------------------------------*/
#[derive(Deserialize)]
pub struct Group
{
    name: String,
    price: Option<f64>,
    weight: Option<i32>,
}


/*----------------------------------------------------------------------------*/
impl Hash for Group
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.name.hash(state)
    }
}


/*----------------------------------------------------------------------------*/
impl PartialEq for Group
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn eq(&self, other: &Self) -> bool
    {
        self.name == other.name
    }
}


/*----------------------------------------------------------------------------*/
impl Eq for Group {}
