use std::{fmt, path::PathBuf};

#[derive(Debug, Clone)]
pub struct WarriorMetadata {
    pub name: String,
    pub author: Option<String>,
    pub strategy: Option<String>,
}

impl Default for WarriorMetadata {
    fn default() -> Self {
        Self {
            name: "_default_name".to_owned(),
            author: None,
            strategy: None,
        }
    }
}

impl WarriorMetadata {
    /// Construct `WarriorMetadata` with `name` derived from `filepath`.
    /// Example: filepath: "warriors/doge.red" -> name: "doge"
    #[allow(
        clippy::missing_panics_doc,
        clippy::unwrap_used,
        reason = "This operation is guaranteed to succeed because an earlier operation to open and read this file has succeeded."
    )]
    #[must_use]
    pub fn from_file(filepath: &str) -> Self {
        assert!(!filepath.is_empty());

        let pathbuf = PathBuf::from(filepath);

        let name = pathbuf
            .file_prefix()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        Self {
            name,
            author: None,
            strategy: None,
        }
    }
}

impl fmt::Display for WarriorMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const WIDTH: usize = 15;

        let author = self.author.as_deref().unwrap_or_default();
        let strategy = self.strategy.as_deref().unwrap_or_default();

        writeln!(f, ";redcode\n")?;
        writeln!(f, "{:<WIDTH$}{}", ";name", self.name)?;
        writeln!(f, "{:<WIDTH$}{}", ";author", author)?;
        writeln!(f, "{:<WIDTH$}{}", ";strategy", strategy)?;

        Ok(())
    }
}
