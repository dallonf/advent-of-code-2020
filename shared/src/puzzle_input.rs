pub fn lines(puzzle_input: &str) -> Vec<String> {
    puzzle_input
        .trim()
        .lines()
        .map(|line| String::from(line))
        .collect()
}
