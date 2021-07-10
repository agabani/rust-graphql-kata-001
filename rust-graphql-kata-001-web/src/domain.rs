#[derive(Clone)]
pub struct User {
    pub id: String,
    pub username: String,
}

#[derive(Clone)]
pub struct Session {
    pub id: String,
    pub user_agent: String,
}
