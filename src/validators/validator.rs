use crate::{
    input::Item,
    validate::Validate,
    validators::{
        GroupValues,
        DistancesValues,
        TemperaturesValues,
    },
};


/*----------------------------------------------------------------------------*/
pub enum Validator<'a>
{
    GroupValues(&'a GroupValues),
    DistancesValues(&'a DistancesValues),
    TemperaturesValues(&'a TemperaturesValues),
}


/*----------------------------------------------------------------------------*/
impl<'a> Validator<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn validate(&self, item: &Item) -> crate::Result<()>
    {
        use Validator::*;

        match self
        {
            GroupValues(groups) => groups.validate(item),
            DistancesValues(distances) => distances.validate(item),
            TemperaturesValues(temperatures) => temperatures.validate(item),
        }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> From<&'a GroupValues> for Validator<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(group_values: &'a GroupValues) -> Self
    {
        Self::GroupValues(group_values)
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> From<&'a DistancesValues> for Validator<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(distances_values: &'a DistancesValues) -> Self
    {
        Self::DistancesValues(distances_values)
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> From<&'a TemperaturesValues> for Validator<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(temperatures_values: &'a TemperaturesValues) -> Self
    {
        Self::TemperaturesValues(temperatures_values)
    }
}
