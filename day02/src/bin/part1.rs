use anyhow;

fn main() {
    // Open a text file in the same dir
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output.expect("not a u32 number"));
}

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<u32> {
    Ok(42)
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
        assert_eq!(result, 42);

        Ok(())
    }
}
