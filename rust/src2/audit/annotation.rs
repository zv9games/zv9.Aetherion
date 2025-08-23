// annotation.rs

//! 📓 Annotation Module
//! Encodes ceremonial metadata for audit, overlay, and legacy introspection.
//! Used to embed purpose, analogy, and lineage into engine modules.

use std::collections::HashMap;

/// Annotation for a module, function, or ritual
#[derive(Clone, Debug)]
pub struct Annotation {
    pub name: String,         // e.g. "Bot Flipper"
    pub purpose: String,      // e.g. "Anchors spatial logic across dimensions"
    pub analogy: Option<String>, // e.g. "Like a Rubik's Cube pivot"
    pub lineage: Vec<String>, // e.g. ["dimension.rs", "flip.rs"]
}

/// Registry of annotations
#[derive(Default)]
pub struct AnnotationLedger {
    pub entries: HashMap<String, Annotation>,
}

impl AnnotationLedger {
    /// Add or update an annotation
    pub fn annotate(&mut self, key: &str, annotation: Annotation) {
        self.entries.insert(key.to_string(), annotation);
    }

    /// Retrieve annotation summary
    pub fn summarize(&self, key: &str) -> Option<String> {
        self.entries.get(key).map(|a| {
            let analogy = a.analogy.as_ref().map(|s| format!(" — {}", s)).unwrap_or_default();
            format!("📌 {}: {}{}", a.name, a.purpose, analogy)
        })
    }

    /// Export all annotations for overlay or manifest
    pub fn export_all(&self) -> Vec<String> {
        self.entries
            .values()
            .map(|a| {
                let analogy = a.analogy.as_ref().map(|s| format!(" — {}", s)).unwrap_or_default();
                format!("📌 {}: {}{}", a.name, a.purpose, analogy)
            })
            .collect()
    }
}
