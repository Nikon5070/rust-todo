#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub checked: bool,
}