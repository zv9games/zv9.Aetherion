// TODO:
// - Group tokens into meaningful DslCommand blocks
// - Support command arguments with named keys
// - Validate command structure (e.g. required args)
// - Add error handling for malformed input
// - Support multi-line scripts and comments


use crate::aetherion::codegen::dsl::{DslToken, DslCommand, DslScript};

/// A simple tokenizer that splits raw input into DSL tokens.
/// This is a stub — expand with real lexical analysis.
pub fn tokenize(source: &str) -> Vec<DslToken> {
    let mut tokens = Vec::new();

    for word in source.split_whitespace() {
        if let Ok(num) = word.parse::<i64>() {
            tokens.push(DslToken::Number(num));
        } else if word.starts_with('"') && word.ends_with('"') {
            tokens.push(DslToken::StringLiteral(word.trim_matches('"').to_string()));
        } else if word.chars().all(|c| c.is_alphabetic()) {
            tokens.push(DslToken::Identifier(word.to_string()));
        } else {
            for ch in word.chars() {
                tokens.push(DslToken::Symbol(ch));
            }
        }
    }

    tokens
}

/// Parses a sequence of tokens into a list of DSL commands.
/// This is a stub — expand with proper grammar rules.
pub fn parse_tokens(tokens: Vec<DslToken>) -> DslScript {
    let mut script = DslScript::new();

    // TODO: Implement real parsing logic
    if !tokens.is_empty() {
        script.push_command(DslCommand {
            name: "noop".into(),
            args: tokens,
        });
    }

    script
}

/// Parses raw DSL source text into a structured script.
/// Combines tokenization and parsing.
pub fn parse(source: &str) -> Result<DslScript, String> {
    let tokens = tokenize(source);
    let script = parse_tokens(tokens);
    Ok(script)
}
