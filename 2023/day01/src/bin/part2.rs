use anyhow;

fn main() {
    // Open a text file in the same dir
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output.expect("not a u32 number"));
}

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<u32> {
    // Read file line by line
    let output = input.lines().map(process_line).sum::<u32>();

    Ok(output)
}

fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        // For each iteration we take the substring from index to the end
        let reduced_line = &line[index..];

        // Hard coded solution, verifies if the substring we have starts
        // with a certain pattern, if so then we return a char value to substitute
        // the first char of the substring we were analyzing
        let result = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };

        result.to_digit(10)
    });

    // At this point we have converted every possible spelled number's
    // first letter with the number it tells (for example 1ne for one).
    // We can now analyze the string the same way we were doing in part 1

    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("should be a valid number")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // This library allows to run the same test with multiple inputs
    // Defining the different inputs each time with these test cases
    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn line_test(#[case] line: &str, #[case] expected: u32) -> anyhow::Result<()>{
        assert_eq!(process_line(line), expected);
        Ok(())
    }

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let test_input = "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";

        let result = process(test_input).expect("Test: not a u32 number");
        assert_eq!(result, 281);

        Ok(())
    }
}
