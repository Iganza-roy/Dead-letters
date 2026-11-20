#[derive(Debug)]
pub struct Letter {
    pub id: u64,
    pub title: String,
    pub recipient: Option<String>,
    pub category: String,
    pub date_created: String,
    pub message: String,
}
