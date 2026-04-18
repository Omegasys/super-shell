#[derive(Debug, Clone)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
}

pub fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.split_whitespace().collect();

    let program = parts.get(0).unwrap_or(&"").to_string();
    let args = parts.iter().skip(1).map(|s| s.to_string()).collect();

    Command { program, args }
}
