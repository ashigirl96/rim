use crate::Row;
use std::str::FromStr;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn open() -> Self {
        let mut rows = Vec::new();
        let row = Row::from_str("Hello, world!").unwrap();
        rows.push(row);
        Self { rows }
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
