// TODO: Phase 1 — Tokenize input into DslToken stream
// TODO: Phase 2 — Group tokens into DslCommand blocks
// TODO: Phase 3 — Validate command structure and arguments
// TODO: Phase 4 — Return structured DslScript or error


use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// DSL token types used in parsing or interpreting custom scripts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DslToken {
    Identifier(String),
    Number(i64),
    StringLiteral(String),
    Symbol(char),
    Keyword(String),
}

/// Represents a parsed DSL command or expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslCommand {
    pub name: String,
    pub args: Vec<DslToken>,
}

/// A parsed DSL script consisting of multiple commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslScript {
    pub commands: Vec<DslCommand>,
}

impl DslScript {
    /// Creates an empty script.
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// Adds a command to the script.
    pub fn push_command(&mut self, cmd: DslCommand) {
        self.commands.push(cmd);
    }

    /// Returns true if the script is empty.
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

/// Parses raw DSL text into a structured script.
/// This is a stub — expand with real parsing logic.
pub fn parse_dsl(source: &str) -> Result<DslScript, String> {
    let mut script = DslScript::new();

    // TODO: Tokenize and parse the source string into DslCommand entries.
    // Example: "spawn_tile x=10 y=20 layer=2" → DslCommand { name: "spawn_tile", args: [...] }

    Err("DSL parser not yet implemented".into())
}
