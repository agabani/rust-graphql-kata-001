#[derive(Clone)]
pub struct Created(pub time::OffsetDateTime);

#[derive(Clone)]
pub struct Forum {
    id: ForumId,
    created: Created,
    created_by: UserId,
    name: ForumName,
}

#[derive(Clone)]
pub struct ForumId(pub String);

#[derive(Clone)]
pub struct ForumName(pub String);

#[derive(Clone)]
pub struct Reply {
    id: ReplyId,
    created: Created,
    created_by: UserId,
    thread: ThreadId,
    text: ReplyText,
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
    id: ThreadId,
    created: Created,
    created_by: UserId,
    forum: ForumId,
    name: ThreadName,
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
