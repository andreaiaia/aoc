use std::collections::BTreeMap;
use itertools::Itertools;
use anyhow;

#[derive(Debug)]
enum Value {
    Symbol(char),
    Empty,
    Number(u32),
}

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<u32> {
// sum part numbers
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(
                move |(x, character)| {
                    (
                        (y as i32,x as i32),
                        match character {
                            '.' => Value::Empty,
                            c if c.is_ascii_digit() => {
                                Value::Number(
                                    c.to_digit(10).expect(
                                        "should be a number",
                                    ),
                                )
                            }
                            c => Value::Symbol(c),
                        },
                    )
                },
            )
        })
        .collect::<BTreeMap<(i32, i32), Value>>();

    let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
    for ((y, x), value) in map.iter() {
        if let Value::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(((last_num_x, _), _)) => {
                            if last_num_x + 1 == *x {
                                let last = numbers
                                    .iter_mut()
                                    .last()
                                    .expect("should exist");
                                last.push(((*x, *y), *num));
                            } else {
                                numbers.push(vec![(
                                    (*x, *y),
                                    *num,
                                )]);
                            }
                        }
                        None => unimplemented!(
                            "shouldn't happen"
                        ),
                    }
                }
                None => {
                    numbers.push(vec![((*x, *y), *num)]);
                }
            }
        }
    }

    // map: entire grid
    // numbers: sequential numbers
    let mut total = 0;
    for symbol in map.iter().filter(|(_key, value)| {
        matches!(value, Value::Symbol('*'))
    }) {
        // (x,y)
        let positions = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let pos_to_check: Vec<(i32, i32)> = positions
            .iter()
            .map(|outer_pos| {
                // outer_pos.x + pos.x, .y + .y
                (
                    outer_pos.0 + symbol.0 .1,
                    outer_pos.1 + symbol.0 .0,
                )
            })
            .collect();

        // dbg!(pos_to_check.len(), pos_to_check);
        let mut indexes_of_numbers = vec![];

        for pos in pos_to_check {
            for (i, num_list) in numbers.iter().enumerate()
            {
                if num_list
                    .iter()
                    .any(|(num_pos, _)| num_pos == &pos)
                {
                    indexes_of_numbers.push(i);
                }
            }
        }

        let is_gear =
            indexes_of_numbers.iter().unique().count() == 2;

        if is_gear {
            total += indexes_of_numbers
                .iter()
                .unique()
                .map(|index| {
                    numbers[*index]
                        .iter()
                        .map(|(_, num)| num.to_string())
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap()
                })
                .product::<u32>();
        }
    }

    Ok(total)
}

fn main() {
    // Open a text file in the same dir
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output.expect("not a u32 number"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        // Attention, if you indent the lines from Game 2 to Game 5, the
        // test doesn't work anymore because the line doesn't start with "Game x:"
        // but with tabs, so only the first game would be evaluated.
        let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = process(test_input).expect("Test: not a u32 number");
        assert_eq!(result, 467835);

        Ok(())
    }
}
