pub mod forum;
pub mod reply;
pub mod session;
pub mod thread;
pub mod user;

pub struct Database {
    pub postgres: sqlx::Pool<sqlx::Postgres>,
}

pub struct Identity<I, T> {
    pub id: I,
    pub value: T,
}

impl Database {
    pub fn new(postgres: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { postgres }
    }
}
