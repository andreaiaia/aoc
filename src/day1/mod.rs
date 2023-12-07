use std::fs::read_to_string;

pub fn day1() {
    println!("day 1:");
    let lines = read_lines("./prova.txt");

    for line in lines {
        println!("{}", line);
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
