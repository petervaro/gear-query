use std::fmt::{
    self,
    Display,
    Formatter,
};

use crate::input::{
    Meta,
    Item,
};


/*----------------------------------------------------------------------------*/
pub struct Sum(String);


/*----------------------------------------------------------------------------*/
impl Sum
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new(column: &str,
               meta: &Meta,
               items: &[&Item]) -> Self
    {
        let mut formatted = format!("Total {}: ", column);
        match column
        {
            "weight" =>
            {
                let unit = meta.weight();
                let sum = items.iter().map(|item| item.weight()).sum::<i32>();
                unit.format_to_string(sum, &mut formatted)
            },
            "price" =>
            {
                let unit = meta.price();
                let sum = items.iter().map(|item| item.price()).sum::<f32>();
                unit.format_to_string(sum, &mut formatted)
            },
            _ => panic!("Invalid column: {}", column),
        };

        Self(formatted)
    }
}


/*----------------------------------------------------------------------------*/
impl Display for Sum
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.0)
    }
}
