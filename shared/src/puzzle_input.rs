pub fn lines(puzzle_input: &str) -> Vec<String> {
    puzzle_input
        .trim()
        .lines()
        .map(|line| String::from(line))
        .collect()
}

pub fn to_strs<'a>(input: &'a Vec<String>) -> Vec<&'a str> {
    return input.as_slice().iter().map(|x| x.as_str()).collect();
}
