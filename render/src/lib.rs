mod color_key;
mod renderer;
mod sizes;
pub(crate) use color_key::ColorKey;
pub use renderer::Renderer;
use std::fs::{metadata, File};
use std::io::Read;

/// Read bytes from a file.
pub(crate) fn get_bytes(path: &str) -> Vec<u8> {
    let metadata = metadata(path).unwrap();
    let mut f = File::open(path).unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).unwrap();
    buffer
}