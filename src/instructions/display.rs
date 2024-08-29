use super::Instruction;
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

        let filtered = ["imm16", "imm8", "r8", "r16", "b3", "r16mem"];

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
            .map(|s| {
                if filtered.contains(&s.as_str()) {
                    args.pop().unwrap_or(s).to_lowercase()
                } else {
                    s.to_lowercase()
                }
            })
            .collect::<Vec<String>>()
            .join(" ");

        // Process the captured string (example: convert to uppercase)
        let processed = format!("{}", instruction_formatted);

        // Write the processed string to the actual formatter
        write!(f, "{}", processed)
    }
}
