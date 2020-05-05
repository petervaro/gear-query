use gear;


/*----------------------------------------------------------------------------*/
fn main()
{
    if let Err(error) = gear::main()
    {
        eprintln!("Error: {}", error);
    }
}
