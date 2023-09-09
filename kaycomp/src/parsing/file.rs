use std::path::PathBuf;

#[salsa::input]
pub struct File {
    pub path: PathBuf,
    #[return_ref]
    pub contents: String,
}