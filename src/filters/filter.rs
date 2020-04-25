use crate::{
    input::Item,
    filter::Filter as _Filter,
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
impl _Filter for Filter
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
