#[derive(Clone)]
pub struct Created(pub time::OffsetDateTime);

impl Created {
    pub fn is8601(&self) -> String {
        self.0.format("%Y-%m-%dT%H:%M:%S.%NZ")
    }
}

#[derive(Clone)]
pub struct Forum {
    pub id: ForumId,
    pub created: Created,
    pub created_by: UserId,
    pub name: ForumName,
}

#[derive(Clone)]
pub struct ForumId(pub String);

#[derive(Clone)]
pub struct ForumName(pub String);

#[derive(Clone)]
pub struct Reply {
    pub id: ReplyId,
    pub created: Created,
    pub created_by: UserId,
    pub thread: ThreadId,
    pub text: ReplyText,
}

#[derive(Clone)]
pub struct ReplyId(pub String);

#[derive(Clone)]
pub struct ReplyText(pub String);

#[derive(Clone)]
pub struct Session {
    pub id: SessionId,
    pub user_agent: UserAgent,
    pub created: Created,
}

#[derive(Clone)]
pub struct SessionId(pub String);

#[derive(Clone)]
pub struct Thread {
    pub id: ThreadId,
    pub created: Created,
    pub created_by: UserId,
    pub forum: ForumId,
    pub name: ThreadName,
}

#[derive(Clone)]
pub struct ThreadId(pub String);

#[derive(Clone)]
pub struct ThreadName(pub String);

#[derive(Clone)]
pub struct User {
    pub id: UserId,
    pub username: Username,
}

#[derive(Clone)]
pub struct UserAgent(pub String);

#[derive(Clone)]
pub struct UserId(pub String);

#[derive(Clone)]
pub struct Username(pub String);
