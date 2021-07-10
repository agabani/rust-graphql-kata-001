#[derive(Clone)]
pub struct Session {
    pub id: String,
    pub user_agent: UserAgent,
}

#[derive(Clone)]
pub struct User {
    pub id: UserId,
    pub username: Username,
}

#[derive(Clone)]
pub struct UserAgent(pub String);

#[derive(Clone)]
pub struct UserId(pub String);

impl UserId {
    pub fn new(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Clone)]
pub struct Username(pub String);

impl Username {
    pub fn new(value: &str) -> Self {
        Self(value.to_string())
    }
}
