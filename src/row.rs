use std::cmp;
use std::str::FromStr;
use std::string::ParseError;

pub struct Row {
    string: String,
}

impl FromStr for Row {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            string: String::from(s),
        })
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        self.string.get(start..end).unwrap_or_default().to_string()
    }
}
