use clap::{
    App,
    Arg,
    ArgMatches,
};

use crate::input::Item;


/*----------------------------------------------------------------------------*/
pub fn arguments<'a>() -> ArgMatches<'a>
{
    let all =
        Arg::with_name("all").short("a")
                             .long("all")
                             .takes_value(false)
                             .conflicts_with("base")
                             .conflicts_with("consumables")
                             .conflicts_with("groups")
                             .help("Show all items of all groups both \
                                    'base' and 'consumables'");

    let base =
        Arg::with_name("base").short("b")
                              .long("base")
                              .takes_value(false)
                              .help("Show only 'base' items");

    let consumables =
        Arg::with_name("consumables").short("c")
                                     .long("consumables")
                                     .takes_value(false)
                                     .help("Show only 'consumables' items");

    let groups =
        Arg::with_name("groups").short("g")
                                .long("group")
                                .takes_value(true)
                                .value_name("GROUP")
                                .multiple(true)
                                .help("Show specified group.  Can be defined \
                                       multiple times, or can take multiple \
                                       space separated values");

    let distances =
        Arg::with_name("distances").short("d")
                                   .long("distance")
                                   .takes_value(true)
                                   .value_name("DISTANCE")
                                   .multiple(true)
                                   .help("Show specified distance.  Can be \
                                          defined multiple times, or can take \
                                          multiple space separated values");

    let temperatures =
        Arg::with_name("temperatures").short("t")
                                      .long("temperature")
                                      .takes_value(true)
                                      .value_name("TEMPERATURE")
                                      .multiple(true)
                                      .help("Show specified temperature.  Can \
                                             be defined multiple times, or can \
                                             take multiple space separated \
                                             values");

    let sum =
        {
            let values = ["weight", "price"];
            Arg::with_name("sum").short("s")
                                 .long("sum")
                                 .takes_value(true)
                                 .value_name("COLUMN")
                                 .multiple(false)
                                 .possible_values(&values)
                                 .help("Sum up the specified column's values")
        };

    let sort =
        Arg::with_name("sort").short("S")
                              .long("sort-by")
                              .takes_value(true)
                              .value_name("COLUMN")
                              .multiple(false)
                              .possible_values(&Item::FIELDS)
                              .default_value(Item::default_field())
                              .help("Sort rows by a specific column");

    let order =
        {
            let values = ["ascending", "descending"];
            Arg::with_name("order").short("o")
                                   .long("order")
                                   .takes_value(true)
                                   .value_name("ORDER")
                                   .multiple(false)
                                   .possible_values(&values)
                                   .default_value(values[0])
                                   .help("Order rows")
        };

    let columns =
        Arg::with_name("columns").short("C")
                                 .long("column")
                                 .takes_value(true)
                                 .value_name("COLUMN")
                                 .multiple(true)
                                 .possible_values(&Item::FIELDS)
                                 .help("Show specified columns only.  Can be \
                                        defined multiple times, or can take \
                                        multiple space separated values");

    let path =
        Arg::with_name("path").takes_value(true)
                              .value_name("PATH")
                              .multiple(false)
                              .default_value("gear.toml")
                              .index(1)
                              .help("Specify path to the TOML file contains \
                                     gear items");

    // TODO: use CARGO_PKG_DESCRIPTION to populer `.about`
    App::new("Gear Query").version(env!("CARGO_PKG_VERSION"))
                          .author(env!("CARGO_PKG_AUTHORS"))
                          .arg(path)
                          .arg(all)
                          .arg(base)
                          .arg(consumables)
                          .arg(groups)
                          .arg(distances)
                          .arg(temperatures)
                          .arg(sum)
                          .arg(sort)
                          .arg(order)
                          .arg(columns)
                          .get_matches()
}
