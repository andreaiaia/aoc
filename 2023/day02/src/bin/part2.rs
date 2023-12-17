use std::collections::BTreeMap;
use anyhow;
use nom::{
    bytes::complete::tag,
    character::complete::{
        self, alpha1, digit1, line_ending,
    },
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    amount: u32
}

#[derive(Debug)]
struct Game<'a> {
    id: &'a str,
    rounds: Vec<Vec<Cube<'a>>>
}

impl<'a> Game<'a> {
    fn minimum_cube_set(&self) -> u32 {
        let map: BTreeMap<&str, u32> = BTreeMap::new();
        self.rounds.iter().fold(map, |mut acc, round| {
            for cube in round.iter() {
                acc.entry(cube.color)
                    .and_modify(|v| {
                    *v = (*v).max(cube.amount);
                }).or_insert(cube.amount);
            }
            acc
        })
        .values()
        .product()
    }
}

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) =
        separated_pair(complete::u32, tag(" "), alpha1)(input)?;

    Ok((input, Cube{ color, amount }))
}

fn round(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;

    Ok((input, cubes))
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) =
        preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(
        tag(": "),
        separated_list1(tag("; "), round)
    )(input)?;

    Ok((input, Game {
        id,
        rounds
    }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<u32> {
    let games = parse_games(input).expect("should parse");

    Ok(games
        .1
        .iter()
        .map(|game| game.minimum_cube_set())
        .sum::<u32>()
    )
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
        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = process(test_input).expect("Test: not a u32 number");
        assert_eq!(result, 2286);

        Ok(())
    }
}
