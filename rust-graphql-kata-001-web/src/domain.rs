pub struct Created(pub time::OffsetDateTime);

impl Created {
    pub fn is8601(&self) -> String {
        self.0.format("%Y-%m-%dT%H:%M:%S.%NZ")
    }
}

pub struct Forum {
    pub id: ForumId,
    pub created: Created,
    pub created_by: UserId,
    pub name: ForumName,
}

pub struct ForumId(pub String);

pub struct ForumName(pub String);

pub struct Reply {
    pub id: ReplyId,
    pub created: Created,
    pub created_by: UserId,
    pub thread: ThreadId,
    pub text: ReplyText,
}

pub struct ReplyId(pub String);

pub struct ReplyText(pub String);

pub struct Session {
    pub id: SessionId,
    pub user_agent: UserAgent,
    pub created: Created,
}

pub struct SessionId(pub String);

pub struct Thread {
    pub id: ThreadId,
    pub created: Created,
    pub created_by: UserId,
    pub forum: ForumId,
    pub name: ThreadName,
}

pub struct ThreadId(pub String);

pub struct ThreadName(pub String);

pub struct User {
    pub id: UserId,
    pub username: Username,
}

pub struct UserAgent(pub String);

#[derive(Clone)]
pub struct UserId(pub String);

pub struct Username(pub String);
