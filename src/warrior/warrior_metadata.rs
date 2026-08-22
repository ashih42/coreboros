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
