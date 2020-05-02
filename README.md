# Gear Query

Gear query is an easy to use command line tool to keep track and query outdoor /
backpacking equipment.

## Dependencies

To install it a Rust compiler and Cargo are needed, which could be installed by
following [this guide][rust].

## Install

```bash
$ git clone https://gitlab.com/petervaro/gear.git
$ cd gear
$ cargo install --path .
$ printf 'export PATH=$PATH:$HOME/.cargo/bin\n' > ~/.bashrc
```

## Gear List

The gear list file uses the [TOML][toml] format.

There's a single entity type, the item, which could be used to describe the gear
list.  But before it could be used, the `meta` section's values needs to be
specified.

### Meta

These values are used to properly represent the data in the items.  Currently
the `weight` and `price` _formatters_ could be specified, each with a unit
symbol and the position of the symbol (either `'PREFIX'` or `'SUFFIX'`).

#### Example

```toml
# Here we describe that all weight values are in grams and
# the 'g' unit should be put after the value, e.g. 12g
[meta.weight]
symbol = "g"
position = "SUFFIX"

# Here we describe that all price values are in dollars and
# the '$' symbol should be put before the value, e.g. $34.55
[meta.price]
symbol = "$"
position = "PREFIX"
```

### Item

An item is a piece of gear that must have a `kind` attribute, a string.  All the
other attributes are optional, but they could be the followings:

- `name`: string
- `group`: string
- `weight`: integer
- `price`: float
- `distances`: list of strings
- `temperatures`: list of strings

There are no predefined values, `distances` or `temperatures` could be arbitrary
as well the `group` an item _belongs to_.

There are 2 main sections of the gear list where items could be specified:

- `base`: According to the following definition:
  > Base weight (the weight of a backpack plus the gear inside & outside it,
  > excluding consumables such as food, water, and fuel, which vary depending on
  > the duration and style of trip)
  > &mdash; [Wikipedia][base]
- `consumables`: Everything else

#### Example

```toml
[[base]]
kind = "Rucksack"
weight = 1201
price = 149.99
distances = ["Short"]
temperatures = ["Warm", "Cold"]

[[base]]
kind = "Water Filter"
name = "MSR TrailShot Micro"
group = "Kitchen"
weight = 146
price = 39.95
distances = ["Long"]
temperatures = ["Warm", "Cold"]

# The following specifies two main scenarios:
# - short distance: 1L water in bottle and 1L water in reservoir, and
# - long distance: 2L water in bottles and 3L water in reservoir

[[consumables]]
kind = "Water"
name = "In Bottle"
weight = 1000
distances = ["Short", "Long"]
temperatures = ["Warm", "Cold"]

[[consumables]]
kind = "Water"
name = "In Bottle"
weight = 1000
distances = ["Long"]
temperatures = ["Warm", "Cold"]

[[consumables]]
kind = "Water"
name = "In Reservoir"
weight = 1000
distances = ["Short", "Long"]
temperatures = ["Warm", "Cold"]

[[consumables]]
kind = "Water"
name = "In Reservoir"
weight = 2000
distances = ["Long"]
temperatures = ["Warm", "Cold"]
```

## Usage

### Simple Usage

```bash
# Either change the directory where the `gear.toml` file is
$ cd path/to/gear.toml
$ gear --all

# Or specify the path as a positional argument in which
# case the name of the TOML file doesn't matter
$ gear path/to/gear/list/toml --all
```

### Advanced Features

For all the available options read the output of help:

```bash
$ gear --help
```


<!-- LINKS -->
[rust]: https://www.rust-lang.org/tools/install
[toml]: https://github.com/toml-lang/toml
[base]: https://en.wikipedia.org/wiki/Ultralight_backpacking
