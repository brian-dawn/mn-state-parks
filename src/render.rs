use std::collections::HashSet;

use anyhow::{Context, Error};
use askama::Template;
use chrono::{Datelike, NaiveDate};
use human_sort::compare;

use crate::api::ParsedPark;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    parks: &'a [ParsedPark],
    dates_to_show: &'a [NaiveDate],
}

mod filters {
    use chrono::{Datelike, NaiveDate};

    pub fn shortdate(s: &NaiveDate) -> ::askama::Result<String> {
        Ok(format!("{}", s.day()))
    }
}

pub fn render(parks: &[ParsedPark]) -> Result<String, Error> {
    let dates_to_show = parks
        .iter()
        .flat_map(|park| park.units.iter())
        .flat_map(|unit| unit.slices.iter())
        .map(|slice| slice.date.clone())
        .collect::<HashSet<_>>();

    let dates_to_show = fill_in_dates(&dates_to_show).context("No dates")?;

    let mut parks = parks.clone().to_vec();

    // Sort the parks by name.
    parks.sort_by(|a, b| a.name.cmp(&b.name));

    // Sort the units by name.
    for park in parks.iter_mut() {
        park.units
            .sort_by(|a, b| compare(&a.short_name, &b.short_name));
    }

    let index = IndexTemplate {
        parks: &parks,
        dates_to_show: &dates_to_show,
    };

    Ok(index.render()?)
}

#[test]
fn test_fill_in_dates() {
    let mut data = HashSet::new();
    data.insert(NaiveDate::from_ymd(2020, 1, 1));
    data.insert(NaiveDate::from_ymd(2020, 1, 3));

    let filled = fill_in_dates(&data).unwrap();
    assert_eq!(3, filled.len());
    assert_eq!(NaiveDate::from_ymd(2020, 1, 1), filled[0]);
    assert_eq!(NaiveDate::from_ymd(2020, 1, 2), filled[1]);
    assert_eq!(NaiveDate::from_ymd(2020, 1, 3), filled[2]);
}

fn fill_in_dates(dates_to_show: &HashSet<NaiveDate>) -> Option<Vec<NaiveDate>> {
    let mut out = Vec::new();

    let smallest = dates_to_show.iter().min()?;
    let largest = dates_to_show.iter().max()?;

    let mut current = smallest.clone();
    while current <= *largest {
        out.push(current.clone());

        current += chrono::Duration::days(1);
    }

    Some(out)
}
