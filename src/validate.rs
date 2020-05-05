use crate::input::Item;


/*----------------------------------------------------------------------------*/
pub trait Validate
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn validate(&self, item: &Item) -> crate::Result<()>;

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn attribute(&self) -> String;

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn expected(&self) -> String;

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn missing_message(&self, owner: &str) -> String
    {
        format!("Expected {} for `{}` on `{}`, but the value is missing",
                self.expected(),
                self.attribute(),
                owner)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn invalid_message(&self, owner: &str,
                              invalid: &str) -> String
    {
        format!("Expected {} for `{}` on `{}`, but found: `{}`",
                self.expected(),
                self.attribute(),
                owner,
                invalid)
    }
}
