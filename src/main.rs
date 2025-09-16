use std::{fmt::Display, process};

fn main() {
    let ruleset = RulesetParser::new(get_ruleset());

    println!("{:?}", binary_to_byte(&byte_to_binary(110)));

    let mut input = Row::from(vec![
        false, false, false, false, false, false, false, false, false, true, false,
    ]);

    println!("{input}");

    for _ in 0..20 {
        input.compute_row(&ruleset);
        println!("{input}");
    }
}

fn get_ruleset() -> u8 {
    use std::env;
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No ruleset provided!");
        process::exit(1);
    }

    let ruleset = &args[1];
    match ruleset.parse::<u8>() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error while parsing number: {e}");
            process::exit(1);
        }
    }
}

struct Row {
    row: Vec<bool>,
    dead: String,
    alive: String,
}

impl Row {
    fn compute_row(&mut self, ruleset_parser: &RulesetParser) {
        self.row = ruleset_parser.compute_row(&self.row);
    }
}

impl Default for Row {
    fn default() -> Self {
        Row {
            row: vec![],
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
        let stuff: String = self
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

        write!(f, "{stuff}")
    }
}

struct RulesetParser {
    ruleset: Vec<bool>,
}

impl RulesetParser {
    fn new(ruleset: u8) -> Self {
        RulesetParser {
            ruleset: byte_to_binary(ruleset),
        }
    }

    fn get_life(&self, seq: &[bool]) -> bool {
        let index = binary_to_byte(seq);

        self.ruleset[7 - index as usize]
    }

    fn compute_row(&self, input: &[bool]) -> Vec<bool> {
        // Respect first and last cell always being dead

        let mut out = input.to_owned();

        for i in 1..(input.len() - 1) {
            let col = &input[i - 1..i + 2];

            out[i] = self.get_life(col);
        }

        out
    }
}

impl Default for RulesetParser {
    fn default() -> Self {
        RulesetParser::new(104)
    }
}

fn binary_to_byte(input: &[bool]) -> u8 {
    input
        .iter()
        .rev()
        .enumerate()
        .map(|(i, el)| match el {
            true => 2_u8.pow(u32::try_from(i).unwrap()),
            false => 0,
        })
        .sum::<u8>()
}

fn byte_to_binary(number: u8) -> Vec<bool> {
    let mut result: Vec<bool> = vec![];
    let mut cache = number;
    for i in (0..8).rev() {
        let n: u8 = 2_u8.pow(i);

        if cache >= n {
            cache -= n;
            result.push(true);
        } else {
            result.push(false);
        };
    }

    result
}
