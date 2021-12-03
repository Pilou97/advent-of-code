use crate::prelude::utils::read_string;
use std::str::FromStr;

#[derive(PartialEq, Copy, Clone)]
enum Bit {
    Zero,
    One,
}

impl From<Bit> for i32 {
    fn from(value: Bit) -> i32 {
        match value {
            Bit::Zero => 0,
            Bit::One => 1,
        }
    }
}

impl Bit {
    fn opposite(&self) -> Bit {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }
}

impl From<bool> for Bit {
    fn from(value: bool) -> Bit {
        match value {
            false => Bit::Zero,
            true => Bit::One,
        }
    }
}

#[derive(Clone)]
struct BinaryNumber {
    inner: Vec<Bit>,
}

impl BinaryNumber {
    fn opposite(&self) -> Self {
        Self {
            inner: self
                .inner
                .iter()
                .map(|bit| bit.opposite())
                .collect::<Vec<Bit>>(),
        }
    }

    fn get(&self, n: usize) -> Option<Bit> {
        self.inner.get(n).cloned()
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl From<BinaryNumber> for i32 {
    fn from(value: BinaryNumber) -> i32 {
        value
            .inner
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (index, &value)| {
                acc + i32::from(value) * i32::pow(2, index as u32)
            })
    }
}

impl FromStr for BinaryNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .chars()
            .filter_map(|bit| match bit {
                '0' => Some(Bit::Zero),
                '1' => Some(Bit::One),
                _ => None,
            })
            .collect::<Vec<Bit>>();

        Ok(BinaryNumber { inner })
    }
}

struct Instructions {
    instruction_size: usize,
    inner: Vec<BinaryNumber>,
}

impl Instructions {
    fn new(instructions: Vec<BinaryNumber>) -> Self {
        let instruction_size = match instructions.get(0) {
            Some(instructions) => instructions.len(),
            None => 0,
        };

        Instructions {
            instruction_size,
            inner: instructions,
        }
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn most_common_bit_colomn(&self, nth: usize) -> Bit {
        let (zero_count, one_count) = self.inner.iter().fold((0, 0), |(n_zero, n_one), binary| {
            let bit = binary.inner.get(nth);
            if let Some(&bit) = bit {
                if bit == Bit::Zero {
                    (n_zero + 1, n_one)
                } else {
                    (n_zero, n_one + 1)
                }
            } else {
                (n_zero, n_one)
            }
        });

        if zero_count > one_count {
            Bit::Zero
        } else {
            Bit::One
        }
    }

    fn gamma(&self) -> BinaryNumber {
        let range = 0..self.instruction_size;
        BinaryNumber {
            inner: range
                .into_iter()
                .map(|nth| self.most_common_bit_colomn(nth))
                .collect::<Vec<Bit>>(),
        }
    }

    fn epsilon(&self) -> BinaryNumber {
        self.gamma().opposite()
    }

    fn find_specific_binary_number(&self, most_common: bool) -> BinaryNumber {
        let mut i = 0;
        let instructions = self.inner.clone();
        let mut instructions = Instructions::new(instructions);
        loop {
            let found = {
                let mut found = instructions.most_common_bit_colomn(i);
                if !most_common {
                    found = found.opposite()
                }
                found
            };

            let binary_numbers = instructions
                .inner
                .iter()
                .filter(|binary_number| match binary_number.get(i) {
                    Some(bit) => found == bit,
                    None => false,
                })
                .cloned()
                .collect::<Vec<BinaryNumber>>();
            i += 1;
            instructions = Instructions::new(binary_numbers);

            if instructions.len() <= 1 {
                break;
            }
        }
        instructions.inner.get(0).unwrap().clone() // I know it's bad, ok?
    }

    fn oxygen_generator_rating(&self) -> BinaryNumber {
        self.find_specific_binary_number(true)
    }

    fn co2_scrubber_rating(&self) -> BinaryNumber {
        self.find_specific_binary_number(false)
    }
}

pub fn day3() {
    let raw_data = read_string("data/day_3.data");
    let binary_numbers = raw_data
        .split('\n')
        .filter_map(|str| match BinaryNumber::from_str(str) {
            Ok(res) => Some(res),
            Err(_) => None,
        })
        .collect::<Vec<BinaryNumber>>();

    let instructions = Instructions::new(binary_numbers);

    println!(
        "Day 3, Puzzle 1 : {}",
        i32::from(instructions.epsilon()) * i32::from(instructions.gamma())
    );

    println!(
        "Day 3, Puzzle 2 : {}",
        i32::from(instructions.oxygen_generator_rating())
            * i32::from(instructions.co2_scrubber_rating()),
    );

    println!("------")
}
