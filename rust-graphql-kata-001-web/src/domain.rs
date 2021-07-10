#[derive(Clone)]
pub struct Created(pub time::OffsetDateTime);

#[derive(Clone)]
pub struct Session {
    pub id: SessionId,
    pub user_agent: UserAgent,
    pub created: Created,
}

#[derive(Clone)]
pub struct SessionId(pub String);

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
