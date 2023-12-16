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
    let output = input.lines().map(|line| {
        // Read every line char by char and returns only the digits converted to u32
        let mut it = line.chars().filter_map(|character| {
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
    }).sum::<u32>();

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let test_input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

        let result = process(test_input).expect("Test: not a u32 number");
        assert_eq!(result, 142);

        Ok(())
    }
}
