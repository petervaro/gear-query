use crate::{
    filter,
    input::Item,
    filters::{
        is_in_groups::IsInGroups,
        is_in_distances::IsInDistances,
        is_in_temperatures::IsInTemperatures,
    },
};



/*----------------------------------------------------------------------------*/
pub enum Filter
{
    IsInGroups(IsInGroups),
    IsInDistances(IsInDistances),
    IsInTemperatures(IsInTemperatures),
}


/*----------------------------------------------------------------------------*/
impl filter::Filter for Filter
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn filter(&self, item: &Item) -> bool
    {
        use Filter::*;
        match self
        {
            IsInGroups(filter) => filter.filter(item),
            IsInDistances(filter) => filter.filter(item),
            IsInTemperatures(filter) => filter.filter(item),
        }
    }
}
