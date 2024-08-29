use super::Instruction;
use std::fmt;

fn split_camel_case(s: &str) -> Vec<String> {
    s.chars()
        .fold(
            (Vec::new(), String::new()),
            |(mut words, mut current), c| {
                if c.is_uppercase() && !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
                current.push(c);
                (words, current)
            },
        )
        .0
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

        let filtered = ["imm16", "imm8", "r8", "r16"];

        let instruction_formatted = split_camel_case(&instruction_name)
            .into_iter()
            .map(|s| s.to_lowercase())
            .filter(|s| !filtered.contains(&s.as_str()))
            .collect::<Vec<String>>()
            .join(" ");

        let args = match buffer.find('(') {
            Some(index) => &buffer[index + 1..buffer.len() - 1]
                .split(',')
                .into_iter()
                .map(|s| s.trim())
                .map(|s| {
                    s.parse::<i32>()
                        .map(|n| format!("{:#x}", n))
                        .unwrap_or_else(|_| s.to_string())
                })
                .collect::<Vec<String>>()
                .join(", ")
                .to_lowercase(),
            None => "",
        };

        // Process the captured string (example: convert to uppercase)
        let processed = format!("{} {}", instruction_formatted, args);

        // Write the processed string to the actual formatter
        write!(f, "{}", processed)
    }
}
