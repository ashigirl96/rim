use std::cmp;
use std::str::FromStr;
use std::string::ParseError;
use unicode_segmentation::UnicodeSegmentation;

pub struct Row {
    string: String,
    len: usize,
}

impl FromStr for Row {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row = Self {
            string: String::from(s),
            len: 0,
        };
        row.update_length();
        Ok(row)
    }
}

impl Row {
    // 渡されたstartからendまでの行の文字列を返す。
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        // self.string.get(start..end).unwrap_or_default().to_string()
        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push_str("  ");
            } else {
                result.push_str(grapheme);
            }
        }
        result
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn update_length(&mut self) {
        self.len = self.string[..].graphemes(true).count();
    }
}
