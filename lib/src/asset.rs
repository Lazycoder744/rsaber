use std::borrow::Cow;
use std::io::{Cursor, Read};
use std::sync::Arc;

use rust_embed::Embed;

pub type AssetManagerRc = Arc<dyn AssetManagerTrait + Send + Sync>;

// The *_or_err() methods:
// - They are typically used for accessing built-in assets. In case
//   of any error, the program will be terminated.
// - Don't use them for external assets, as it can happen that the
//   assets are malformed/corrupted.

pub trait AssetManagerTrait {
    fn open(&self, name: &str) -> Result<AssetFileBox>;

    fn open_or_err(&self, name: &str) -> AssetFileBox {
        self.open(name).expect("Unable to open asset")
    }
}

pub type AssetFileBox = Box<dyn AssetFileTrait + Send>;

pub trait AssetFileTrait {
    // TODO: read operations should consume self? - Complete
    fn read(self: Box<Self>) -> Result<Box<dyn Read + Send + Sync>>;
    fn read_str(self: Box<Self>) -> Result<String>;

    fn read_or_err(self: Box<Self>) -> Box<dyn Read + Send + Sync> {
        self.read().expect("Unable to read asset")
    }

    fn read_str_or_err(self: Box<Self>) -> String {
        self.read_str().expect("Unable to read asset")
    }
}

pub type Result<T> = std::result::Result<T, AssetError>; // TODO: Rename to Result? - Complete

#[derive(Debug)]
pub enum AssetError {
    NotFound,
    Decode,
}

impl std::fmt::Display for AssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetError::NotFound => write!(f, "Asset not found"),
            AssetError::Decode => write!(f, "Failed to decode asset"),
        }
    }
}

#[derive(Embed)]
#[folder = "asset"]
struct Asset;

pub struct EmbedAssetManager;

impl EmbedAssetManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

impl AssetManagerTrait for EmbedAssetManager {
    fn open(&self, name: &str) -> Result<AssetFileBox> {
        assert!(name.starts_with("/"));
        let file = Asset::get(&name[1..]).ok_or(AssetError::NotFound)?;
        Ok(Box::new(EmbedAssetFile::new(file.data)))
    }
}

struct EmbedAssetFile {
    data: Cow<'static, [u8]>,
}

impl EmbedAssetFile {
    fn new(data: Cow<'static, [u8]>) -> Self {
        Self { data }
    }
}

impl AssetFileTrait for EmbedAssetFile {
    fn read(self: Box<Self>) -> Result<Box<dyn Read + Send + Sync>> {
        Ok(Box::new(Cursor::new(self.data.clone())))
    }

    fn read_str(self: Box<Self>) -> Result<String> {
        Ok(String::from(
            std::str::from_utf8(&self.data).map_err(|_| AssetError::Decode)?,
        ))
    }
}
