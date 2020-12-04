pub fn lines(puzzle_input: &str) -> Vec<String> {
    puzzle_input
        .trim()
        .lines()
        .map(|line| String::from(line))
        .collect()
}

pub trait PuzzleInput {
    fn to_strs<'a>(&'a self) -> Vec<&'a str>;
}

impl PuzzleInput for &[String] {
    fn to_strs<'a>(&'a self) -> Vec<&'a str> {
        self.iter().map(|x| x.as_str()).collect()
    }
}
impl PuzzleInput for Vec<String> {
    fn to_strs<'a>(&'a self) -> Vec<&'a str> {
        self.iter().map(|x| x.as_str()).collect()
    }
}
