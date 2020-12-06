pub fn lines(puzzle_input: &str) -> Vec<String> {
    puzzle_input
        .trim()
        .lines()
        .map(|line| String::from(line))
        .collect()
}

pub fn lines_strs(puzzle_input: &str) -> Vec<&str> {
    puzzle_input.trim().lines().collect()
}

pub trait StringList {
    fn to_strs<'a>(&'a self) -> Vec<&'a str>;
}

impl StringList for &[String] {
    fn to_strs<'a>(&'a self) -> Vec<&'a str> {
        self.iter().map(|x| x.as_str()).collect()
    }
}
impl StringList for Vec<String> {
    fn to_strs<'a>(&'a self) -> Vec<&'a str> {
        self.iter().map(|x| x.as_str()).collect()
    }
}

impl StringList for &[&str] {
    fn to_strs<'a>(&'a self) -> Vec<&'a str> {
        self.iter().copied().collect()
    }
}
impl StringList for Vec<&str> {
    fn to_strs<'a>(&'a self) -> Vec<&'a str> {
        self.clone()
    }
}
