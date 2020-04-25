use serde::Deserialize;

use crate::input::formatters::Unit;


/*----------------------------------------------------------------------------*/
#[derive(Deserialize)]
pub struct Meta
{
    weight: Unit,
    price: Unit,
}


/*----------------------------------------------------------------------------*/
impl Meta
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn weight(&self) -> &Unit
    {
        &self.weight
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn price(&self) -> &Unit
    {
        &self.price
    }
}
