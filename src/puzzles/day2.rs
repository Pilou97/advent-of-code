use crate::prelude::utils::read_string;
use std::str::FromStr;

struct Submarine {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

#[derive(Clone)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted = s.split(' ').collect::<Vec<&str>>();
        match (splitted.get(0), splitted.get(1)) {
            (Some(command), Some(command_value)) => {
                match (command, i32::from_str(command_value.to_owned())) {
                    (command, Ok(command_value)) => match command.to_string().as_str() {
                        "forward" => Ok(Command::Forward(command_value)),
                        "down" => Ok(Command::Down(command_value)),
                        "up" => Ok(Command::Up(command_value)),
                        _ => Err("Unknown command".to_string()),
                    },
                    _ => Err("Unknown command value".to_string()),
                }
            }
            _ => Err("Unknown".to_string()),
        }
    }
}

impl Submarine {
    fn new() -> Self {
        Submarine {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn command(&self, command: &Command) -> Submarine {
        match command {
            Command::Forward(forward) => Submarine {
                horizontal: self.horizontal + forward,
                depth: self.depth,
                aim: 0,
            },
            Command::Down(down) => Submarine {
                horizontal: self.horizontal,
                depth: self.depth + down,
                aim: 0,
            },
            Command::Up(up) => Submarine {
                horizontal: self.horizontal,
                depth: self.depth - up,
                aim: 0,
            },
        }
    }

    fn command_2(&self, command: &Command) -> Submarine {
        match command {
            Command::Forward(forward) => Submarine {
                horizontal: self.horizontal + forward,
                depth: self.depth + self.aim * forward,
                aim: self.aim,
            },
            Command::Down(down) => Submarine {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim + down,
            },
            Command::Up(up) => Submarine {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim - up,
            },
        }
    }

    fn commands(self, commands: Vec<Command>, new: bool) -> Submarine {
        commands.iter().fold(self, |submarine, cmd| {
            if new {
                submarine.command_2(cmd)
            } else {
                submarine.command(cmd)
            }
        })
    }

    fn solve(self) -> i32 {
        self.horizontal * self.depth
    }
}

pub fn day2() {
    let raw_data = read_string("data/day_2.data");
    let commands = raw_data
        .split('\n')
        .filter_map(|str| match Command::from_str(str) {
            Ok(res) => Some(res),
            Err(_) => None,
        })
        .collect::<Vec<Command>>();

    let submarine = Submarine::new();
    let submarine = submarine.commands(commands.clone(), false);
    println!("Day 2, Puzzle 1 : {}", submarine.solve());

    let submarine = Submarine::new();
    let submarine = submarine.commands(commands, true);
    println!("Day 2, Puzzle 2 : {}", submarine.solve());

    println!("------")
}
