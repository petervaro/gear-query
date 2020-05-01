use std::fmt::{
    self,
    Display,
    Formatter,
};

use crate::{
    input::Item,
    column::{
        Column,
        Alignment,
    },
};


/*----------------------------------------------------------------------------*/
pub struct Table<'a>
{
    columns: Vec<Column>,
    headers: Vec<&'a str>,
}


/*----------------------------------------------------------------------------*/
impl<'a> Table<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    const PADDING: usize = 2;

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new<'f>(headers: Vec<&'a str>,
                   items: impl Iterator<Item = &'f Item>) -> Self
    {
        let mut columns = Vec::new();

        /* Add headers as columns */
        columns.extend(headers.iter().map(
            |header| Column::from_string(Alignment::Centre,
                                         header.to_uppercase())));

        /* Add items as columns */
        for item in items
        {
            columns.extend(item.columns(&headers));
        }

        Self
        {
            columns,
            headers,
        }
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn column_widths(&self) -> impl Iterator<Item = usize> + '_
    {
        let columns_count_in_row = self.headers.len();
        (0..columns_count_in_row).map(
            move |i| self.columns.iter()
                                 .skip(i)
                                 .step_by(columns_count_in_row)
                                 .map(|column| column.width() + Self::PADDING)
                                 .max()
                                 .unwrap_or(0))
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn rows(&self) -> impl Iterator<Item = &'_ [Column]> + '_
    {
        let columns_count_in_row = self.headers.len();
        (0..self.columns.len())
            .step_by(columns_count_in_row)
            .map(move |i| &self.columns[i..i + columns_count_in_row])
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn decorator(column_widths: &[usize]) -> String
    {
        let mut decorator = String::new();
        decorator.push('+');
        for column_width in column_widths
        {
            for _ in 0..*column_width
            {
                decorator.push('-');
            }
            decorator.push('+');
        }

        decorator
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> Display for Table<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        let column_widths =
            {
                let mut column_widths = Vec::with_capacity(self.headers.len());
                column_widths.extend(self.column_widths());
                column_widths
            };
        let decorator = Self::decorator(&column_widths);

        writeln!(f, "{}", decorator)?;

        for row in self.rows()
        {
            write!(f, "|")?;
            for (column, &available_width) in row.iter().zip(&column_widths)
            {
                write!(f, " ")?;
                write!(f, "{}", column.as_fitted(available_width - Self::PADDING))?;
                write!(f, " |")?;
            }

            writeln!(f, "\n{}", decorator)?;
        }

        Ok(())
    }
}
