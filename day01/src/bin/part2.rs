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
    let mut index = 0;
    // The from_fn is an iterator, from the official doc:
    // Creates a new iterator where each iteration calls the provided closure
    // https://doc.rust-lang.org/std/iter/fn.from_fn.html
    let line_iter = std::iter::from_fn(move || {
        // For each iteration we take the string from index to its end
        let reduced_line = &line[index..];

        // Hard coded solution, verifies if the string we have starts
        // with a certain pattern, if so we return a char value to substitute
        // the first char of the substring we were analyzing
        let result = if reduced_line.starts_with("one") {
            Some('1')
        } else if reduced_line.starts_with("two") {
            Some('2')
        } else if reduced_line.starts_with("three") {
            Some('3')
        } else if reduced_line.starts_with("four") {
            Some('4')
        } else if reduced_line.starts_with("five") {
            Some('5')
        } else if reduced_line.starts_with("six") {
            Some('6')
        } else if reduced_line.starts_with("seven") {
            Some('7')
        } else if reduced_line.starts_with("eight") {
            Some('8')
        } else if reduced_line.starts_with("nine") {
            Some('9')
        } else {
            let result = reduced_line.chars().next();
            result
        };

        index += 1;
        result
    });

    // At this point we will have converted every possible spelled number's
    // first letter with the number it tells (for example 1ne for one).
    // We can now analyze the string the same way we were doing in part 1

    // Read every line char by char and returns only the digits converted to u32
    let mut it = line_iter.filter_map(|character| {
        // Convert every char to digit
        // Returns None if the char is not convertable given the radix
        // for example: assert_eq!('f'.to_digit(10), None);
        // but assert_eq!('f'.to_digit(16), Some(15));
        character.to_digit(10)
    });
    // .next returns the next item in the array, in this case the first one
    let first = it.next().expect("should be a number");
    // returns the last item in the array
    let last = it.last();

    match last {
        Some(num) => format!("{first}{num}").parse::<u32>(),
        // If there is only one number, we double it
        None => format!("{first}{first}").parse::<u32>(),
    }.expect("should be a valid number")
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
