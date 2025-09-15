use std::fmt::Display;

use crate::structs::ruleset_parser::RulesetParser;

pub struct Row {
    row: Vec<bool>,
    dead: String,
    alive: String,
}

impl Row {
    pub fn compute_row(&mut self, ruleset_parser: &RulesetParser) {
        self.row = ruleset_parser.compute_row(&self.row);
    }
}

impl Default for Row {
    fn default() -> Self {
        Row {
            row: Vec::new(),
            dead: ".".to_owned(),
            alive: "x".to_owned(),
        }
    }
}

impl From<Vec<bool>> for Row {
    fn from(row: Vec<bool>) -> Self {
        Row {
            row,
            ..Row::default()
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted: String = self
            .row
            .iter()
            .map(|el| -> &str {
                if el.to_owned() {
                    &self.alive
                } else {
                    &self.dead
                }
            })
            .collect();

        write!(f, "{formatted}")
    }
}
