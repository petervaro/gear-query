use std::collections::HashSet;

use serde::Deserialize;

use crate::{
    input::Item,
    validate::Validate,
    validators::Validity,
};


/*----------------------------------------------------------------------------*/
#[derive(Deserialize)]
pub struct GroupValues
{
    values: HashSet<String>,
}


/*----------------------------------------------------------------------------*/
impl GroupValues
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn is_empty(&self) -> bool
    {
        self.values.is_empty()
    }
}


/*----------------------------------------------------------------------------*/
impl Validate for GroupValues
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn attribute(&self) -> String
    {
        "group".into()
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

        match item.group_validity(&self.values)
        {
            Valid => Ok(()),
            Invalid(group) => Err(self.invalid_message(item.id(), group).into()),
            Missing => Err(self.missing_message(item.id()).into()),
        }
    }
}
