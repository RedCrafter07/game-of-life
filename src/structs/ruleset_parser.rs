use crate::util::byte_decimal::{binary_to_byte, byte_to_binary};

pub struct RulesetParser {
    ruleset: Vec<bool>,
}

impl RulesetParser {
    pub fn new(ruleset: u8) -> Self {
        RulesetParser {
            ruleset: byte_to_binary(ruleset),
        }
    }

    fn get_life(&self, seq: &[bool]) -> bool {
        let index = binary_to_byte(seq);

        self.ruleset[7 - index as usize]
    }

    pub fn compute_row(&self, input: &[bool]) -> Vec<bool> {
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
        RulesetParser::new(104) // Just a default value
    }
}
