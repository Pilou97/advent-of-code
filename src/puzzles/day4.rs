use std::str::FromStr;

use crate::utils::read_string;

struct Input {
    inner: Vec<i32>,
}

impl From<String> for Input {
    fn from(value: String) -> Self {
        let inner = value
            .split(',')
            .filter_map(|str| match i32::from_str(str) {
                Ok(res) => Some(res),
                Err(_) => None,
            })
            .collect::<Vec<i32>>();

        Input { inner }
    }
}

#[derive(Clone)]
struct BingoSnapshot {
    wining_board: Vec<(i32, bool)>,
    won_on: i32,
}

#[derive(Clone)]
struct Grid {
    inner: Vec<(i32, bool)>,
    width: usize,
    height: usize,
    bingo_snapshot: Option<BingoSnapshot>,
}

impl From<Vec<i32>> for Grid {
    fn from(value: Vec<i32>) -> Self {
        let inner = value
            .iter()
            .map(|&value| (value, false))
            .collect::<Vec<(i32, bool)>>();
        Grid {
            inner,
            width: 5,
            height: 5,
            bingo_snapshot: None,
        }
    }
}

fn column<T>(vec: &[T], i: usize, width: usize) -> Vec<T>
where
    T: Copy,
{
    vec.iter()
        .enumerate()
        .filter(|(index, _)| {
            let x = index % width;
            x == i
        })
        .map(|(_, elt)| elt)
        .copied()
        .collect()
}

fn line<T>(vec: &[T], i: usize, width: usize) -> Vec<T>
where
    T: Copy,
{
    vec.iter()
        .enumerate()
        .filter(|(index, _)| {
            let y = index / width;
            y == i
        })
        .map(|(_, elt)| elt)
        .copied()
        .collect()
}

impl Grid {
    fn add_number(&self, n: i32) -> Grid {
        let inner = self
            .inner
            .iter()
            .map(|&(number, state)| {
                if number == n {
                    (number, true)
                } else {
                    (number, state)
                }
            })
            .collect::<Vec<(i32, bool)>>();

        // Check if the board ha won
        for i in 0..self.width {
            let is_bingo = column(&inner, i, self.width)
                .iter()
                .all(|value: &(i32, bool)| value.1);

            if is_bingo {
                return Grid {
                    inner: inner.clone(),
                    width: self.width,
                    height: self.height,
                    bingo_snapshot: Some(BingoSnapshot {
                        wining_board: inner,
                        won_on: n,
                    }),
                };
            }
        }

        for i in 0..self.height {
            let is_bingo = line(&inner, i, self.width)
                .iter()
                .all(|value: &(i32, bool)| value.1);
            if is_bingo {
                return Grid {
                    inner: inner.clone(),
                    width: self.width,
                    height: self.height,
                    bingo_snapshot: Some(BingoSnapshot {
                        wining_board: inner,
                        won_on: n,
                    }),
                };
            }
        }

        Grid {
            inner,
            width: self.width,
            height: self.height,
            bingo_snapshot: None,
        }
    }

    fn solve(&self) -> i32 {
        match self.bingo_snapshot.clone() {
            Some(bingo_snapshot) => {
                bingo_snapshot
                    .wining_board
                    .iter()
                    .filter_map(|(number, bool)| match bool {
                        true => None,
                        false => Some(number),
                    })
                    .sum::<i32>()
                    * bingo_snapshot.won_on
            }
            _ => 0,
        }
    }

    fn is_bingo(&self) -> bool {
        self.bingo_snapshot.is_some()
    }
}

pub fn day4() {
    let raw_data = read_string("data/day_4.data");
    let (input, grids) = raw_data
        .split('\n')
        .filter(|&str| !str.is_empty())
        .enumerate()
        .fold(
            ("", vec![]),
            |(input, mut grids): (&str, Vec<Vec<i32>>), (i, value)| {
                if i == 0 {
                    return (value, grids);
                }

                let mut line = value
                    .split(' ')
                    .filter_map(|str| match i32::from_str(str) {
                        Ok(n) => Some(n),
                        Err(_) => None,
                    })
                    .collect::<Vec<i32>>();

                let i = i as i32 - 1;
                let grid_index = i / 5;

                let grid = grids.get_mut(grid_index as usize);
                match grid {
                    Some(grid) => grid.append(&mut line),
                    None => grids.push(line),
                }

                (input, grids)
            },
        );

    let input = Input::from(input.to_string());
    let grids = grids.iter().cloned().map(Grid::from).collect::<Vec<Grid>>();

    let (bingo, _) = input
        .inner
        .iter()
        .fold((vec![], grids), |(bingo, grids), &n| {
            let mut bingo = bingo;
            let next_grids: Vec<Grid> = grids
                .iter()
                .map(|grid| {
                    let next_grid = grid.add_number(n);
                    if next_grid.is_bingo() && !grid.is_bingo() {
                        bingo.push(next_grid.solve());
                    }
                    next_grid
                })
                .collect();
            (bingo, next_grids)
        });

    println!("Day 4, Puzzle 1 : {:?}", bingo.get(0));
    println!("Day 4, Puzzle 2 : {:?}", bingo.iter().last());
}
