use super::Instruction;
use colored::*;
use std::fmt;

fn split_camel_case(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_word = String::new();

    for (i, c) in s.char_indices() {
        if i > 0 && c.is_uppercase() {
            result.push(current_word);
            current_word = String::new();
        }
        current_word.push(c);
    }

    if !current_word.is_empty() {
        result.push(current_word);
    }

    result
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Create a temporary buffer to capture the default debug output
        let mut buffer = String::new();

        // Use a temporary formatter to write the default debug format
        let _ = fmt::write(&mut buffer, format_args!("{:?}", self));

        let instruction_name = match buffer[1..].find('(') {
            Some(index) => buffer[..index + 1].to_string(),
            None => buffer.clone(),
        };

        let args: &mut Vec<String> = match buffer.find('(') {
            Some(index) => &mut buffer[index + 1..buffer.len() - 1]
                .split(',')
                .into_iter()
                .map(|s| s.trim())
                .map(|s| {
                    s.parse::<i32>()
                        .map(|n| format!("{:#x}", n))
                        .unwrap_or_else(|_| s.to_string())
                })
                .rev()
                .collect::<Vec<String>>(),
            None => &mut Vec::new(),
        };

        let instruction_formatted = split_camel_case(&instruction_name)
            .into_iter()
            .map(|s| s.to_lowercase())
            .map(|s| match s.as_str() {
                "imm8" => args.pop().unwrap().green().to_string(),
                "imm16" => args.pop().unwrap().green().to_string(),

                // Registers
                "r8" => args.pop().unwrap().cyan().to_string(),
                "r16" => args.pop().unwrap().cyan().to_string(),
                "r16stk" => args.pop().unwrap().cyan().to_string(),
                "a" => "a".cyan().to_string(),

                // Memory address
                "r16mem" => format!("[{}]", args.pop().unwrap()).blue().to_string(),
                "imm8mem" => format!("[{}]", args.pop().unwrap()).blue().to_string(),
                "imm16mem" => format!("[{}]", args.pop().unwrap()).blue().to_string(),

                "b3" => args.pop().unwrap().blue().to_string(),
                "cond" => args.pop().unwrap().to_string(),
                _ => s,
            })
            .collect::<Vec<String>>()
            .join(" ");

        // Process the captured string (example: convert to uppercase)
        let processed = format!("{}", instruction_formatted);

        // Write the processed string to the actual formatter
        write!(f, "{}", processed.to_lowercase())
    }
}
