// manifest.rs

//! 📜 Manifest Module
//! Registry of EchoEngine modules, their purpose, and audit status.
//! Enables introspection, debugging, and legacy documentation.

use std::collections::HashMap;

/// Metadata for each registered module
#[derive(Clone, Debug)]
pub struct ModuleEntry {
    pub name: &'static str,
    pub path: &'static str,
    pub purpose: &'static str,
    pub invoked: bool,
}

/// The manifest ledger
#[derive(Default)]
pub struct Manifest {
    pub entries: HashMap<&'static str, ModuleEntry>,
}

impl Manifest {
    /// Register a module into the manifest
    pub fn register(&mut self, name: &'static str, path: &'static str, purpose: &'static str) {
        self.entries.insert(name, ModuleEntry {
            name,
            path,
            purpose,
            invoked: false,
        });
    }

    /// Mark a module as invoked
    pub fn invoke(&mut self, name: &'static str) {
        if let Some(entry) = self.entries.get_mut(name) {
            entry.invoked = true;
        }
    }

    /// Get audit summary
    pub fn audit(&self) -> Vec<&ModuleEntry> {
        self.entries.values().collect()
    }
}
