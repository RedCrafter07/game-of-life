use std::{fmt::Display, process};

use crate::{structs::ruleset_parser::RulesetParser, util::get_arg_or_default::get_arg_or_default};

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
        let dead = get_arg_or_default("DEAD_CELL", ".".into()).to_owned();
        let alive = get_arg_or_default("LIVING_CELL", "x".into()).to_owned();

        if dead.len() != 1 {
            eprintln!("Value for dead cell is not one character!");
            process::exit(1);
        }

        if alive.len() != 1 {
            eprintln!("Value for living cell is not one character!");
            process::exit(1);
        }

        Row {
            row: Vec::new(),
            dead,
            alive,
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
