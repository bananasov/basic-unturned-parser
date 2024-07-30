use std::path::Path;

pub mod assets;
pub mod language;

pub trait Parser<T> {
    fn parse<P: AsRef<Path> + ?Sized>(directory: &P, content: &str) -> anyhow::Result<T>;
}
