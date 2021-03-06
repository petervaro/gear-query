use std::collections::HashSet;

use serde::Deserialize;

use crate::{
    input::Item,
    validate::Validate,
    validators::Validity,
};


/*----------------------------------------------------------------------------*/
#[derive(Deserialize)]
pub struct TemperaturesValues
{
    values: HashSet<String>,
}


/*----------------------------------------------------------------------------*/
impl TemperaturesValues
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn is_empty(&self) -> bool
    {
        self.values.is_empty()
    }
}


/*----------------------------------------------------------------------------*/
impl Validate for TemperaturesValues
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn attribute(&self) -> String
    {
        "temperatures".into()
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn expected(&self) -> String
    {
        let mut values = self.values.iter()
                                    .map(String::as_str)
                                    .collect::<Vec<&str>>();
        values.sort_unstable();
        format!("`{}`", values.join("` or `"))
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn validate(&self, item: &Item) -> crate::Result<()>
    {
        use Validity::*;

        debug_assert!(!self.values.is_empty());

        match item.temperatures_validity(&self.values)
        {
            Valid => Ok(()),
            Invalid(temperature) =>
                Err(self.invalid_message(item.id(), temperature).into()),
            Missing => Err(self.missing_message(item.id()).into()),
        }
    }
}
