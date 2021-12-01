use crate::prelude::utils::*;
use std::str::FromStr;

#[derive(PartialEq)]
enum Measurement {
    Increased,
    Decreased,
    NotChanged,
}

struct Data {
    inner: Vec<(i32, Measurement)>,
}

impl Data {
    fn add_measurements(measures: Vec<i32>) -> Vec<(i32, Measurement)> {
        measures
            .iter()
            .enumerate()
            .filter_map(|(index, value)| {
                index
                    .checked_sub(1)
                    .and_then(|previous_index| measures.get(previous_index))
                    .map(|previous| match previous.cmp(value) {
                        std::cmp::Ordering::Less => (*value, Measurement::Increased),
                        std::cmp::Ordering::Equal => (*value, Measurement::NotChanged),
                        std::cmp::Ordering::Greater => (*value, Measurement::Decreased),
                    })
            })
            .collect::<Vec<(i32, Measurement)>>()
    }

    /// remove the first data
    fn new_data1(measures: Vec<i32>) -> Self {
        let inner = Data::add_measurements(measures);
        Data { inner }
    }

    /// Packs the data by three
    fn new_data2(measures: Vec<i32>) -> Self {
        let inner = measures
            .iter()
            .enumerate()
            .filter_map(|(index, _)| {
                match (
                    measures.get(index),
                    measures.get(index + 1),
                    measures.get(index + 2),
                ) {
                    (Some(a), Some(b), Some(c)) => Some(a + b + c),
                    _ => None,
                }
            })
            .collect::<Vec<i32>>();

        let inner = Data::add_measurements(inner);

        Data { inner }
    }

    /// Solve the puzzle
    fn puzzle(&self) -> i32 {
        self.inner
            .iter()
            .map(|(_, measurement)| match measurement {
                Measurement::Increased => 1,
                Measurement::Decreased => 0,
                Measurement::NotChanged => 0,
            })
            .sum()
    }
}

pub fn day1() {
    let raw_data = read_string("data/day_one.data");

    let parsed = raw_data
        .split('\n')
        .filter_map(|str| match i32::from_str(str) {
            Ok(res) => Some(res),
            Err(_) => None,
        })
        .collect::<Vec<i32>>();

    let data = Data::new_data1(parsed.clone());
    let result = data.puzzle();
    println!("Day 1, Puzzle 1 : {}", result);

    let data = Data::new_data2(parsed);
    let result = data.puzzle();
    println!("Day 1, Puzzle 2 : {}", result);

    println!("------")
}
