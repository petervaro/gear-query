use crate::{
    filter,
    input::Item,
    filters::{
        IsInGroups,
        IsInDistances,
        IsInTemperatures,
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
            IsInGroups(f) => f.filter(item),
            IsInDistances(f) => f.filter(item),
            IsInTemperatures(f) => f.filter(item),
        }
    }
}
