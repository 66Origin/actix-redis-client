use actix::MailboxError;
use redis::RedisError;

/// Error type
#[derive(Debug, Fail)]
pub enum ActixRedisClientError {
    #[fail(display = "Actor is dead | MailboxError {}", _0)]
    MailboxError(MailboxError),
    #[fail(display = "Redis error | RedisError {}", _0)]
    RedisError(RedisError),
    #[fail(display = "Unknown error | {}", _0)]
    Unknown(&'static str),
}

impl From<&'static str> for ActixRedisClientError {
    fn from(s: &'static str) -> Self {
        ActixRedisClientError::Unknown(s)
    }
}

impl From<MailboxError> for ActixRedisClientError {
    fn from(e: MailboxError) -> Self {
        ActixRedisClientError::MailboxError(e)
    }
}

impl From<RedisError> for ActixRedisClientError {
    fn from(e: RedisError) -> Self {
        ActixRedisClientError::RedisError(e)
    }
}
