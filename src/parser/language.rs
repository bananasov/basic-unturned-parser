use std::path::PathBuf;

use anyhow::Context;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Language {
    pub name: String,
    pub description: String,
}

impl Language {
    pub fn parse_language(path: PathBuf) -> anyhow::Result<Language> {
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read content for: {}", path.display()))?;
        let mut language = Language::default();

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let remainder = split.remainder().unwrap_or("");

            match field {
                "Name" => language.name = remainder.into(),
                "Description" => language.description = remainder.into(),
                _ => {}
            }
        }

        Ok(language)
    }
}
