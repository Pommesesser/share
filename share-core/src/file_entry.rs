#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct FileEntry {
    pub id: String,
    pub name: String,
    pub size: i64,
}

