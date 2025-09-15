use std::{io, process};

pub fn get_ruleset(args: &[String]) -> u8 {
    if args.len() < 2 {
        eprintln!("Please enter a ruleset!");
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

pub fn get_start(args: &[String]) -> Vec<bool> {
    let input_string: String = if args.len() > 2 {
        args[2].clone()
    } else {
        println!(
            "Please enter x for alive and . for dead. Also, please start and end with a dead cell."
        );

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line!");

        let user_input = user_input.trim().to_string();

        let invalid_characters: Vec<_> = user_input
            .chars()
            .filter(|c| *c != 'x' && *c != '.')
            .collect();

        if !invalid_characters.is_empty() {
            eprintln!(
                "Invalid characters found: {}",
                invalid_characters
                    .iter()
                    .enumerate()
                    .map(|(i, c)| format!(
                        "{c}{} ",
                        if i == invalid_characters.len() - 1 {
                            ""
                        } else {
                            ","
                        }
                    ))
                    .collect::<String>()
            );
            eprintln!("Exiting...");
            process::exit(1);
        }

        user_input
    };

    if !input_string.starts_with(".") {
        eprintln!("The input doesn't start with \".\"!");
        process::exit(1);
    }

    if !input_string.ends_with(".") {
        eprintln!("The input doesn't end with \".\"!");
        process::exit(1);
    }

    input_string.chars().map(|c| c == 'x').collect() // map x to true, rest to false
}
