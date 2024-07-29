use std::path::Path;

pub mod assets;
pub mod language;

pub trait Parser<T> {
    fn parse<P: AsRef<Path>>(directory: P, content: String) -> anyhow::Result<T>;
}
