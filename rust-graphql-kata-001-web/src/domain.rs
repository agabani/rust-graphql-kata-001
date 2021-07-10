#[derive(Clone)]
pub struct Session {
    pub id: String,
    pub user_agent: UserAgent,
}

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub username: Username,
}

#[derive(Clone)]
pub struct UserAgent(pub String);

#[derive(Clone)]
pub struct Username(pub String);
