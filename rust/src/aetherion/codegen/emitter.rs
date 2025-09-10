use super::config::Config;

pub fn generate_code(input: &str, config: &Config) -> String {
    let input = input.trim();

    if input.is_empty() {
        return String::new();
    }

    if !input.starts_with("struct ") {
        panic!("Invalid input: must start with 'struct'");
    }

    let struct_name = input
        .strip_prefix("struct ")
        .unwrap()
        .trim_end_matches(';')
        .trim();

    let mut output = String::new();

    if config.derive_debug {
        output.push_str("#[derive(Debug)]\n");
    }

    output.push_str(&format!("impl {} {{}}\n", struct_name));
    output
}
