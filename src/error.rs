use actix::MailboxError;
use redis::RedisError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ActixRedisClientError(String);

unsafe impl Sync for ActixRedisClientError {}
unsafe impl Send for ActixRedisClientError {}

impl fmt::Display for ActixRedisClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl From<&'static str> for ActixRedisClientError {
    fn from(s: &'static str) -> Self {
        ActixRedisClientError(String::from(s))
    }
}

impl From<String> for ActixRedisClientError {
    fn from(s: String) -> Self {
        ActixRedisClientError(s.clone())
    }
}

impl From<MailboxError> for ActixRedisClientError {
    fn from(e: MailboxError) -> Self {
        ActixRedisClientError(format!("{}", e))
    }
}

impl From<RedisError> for ActixRedisClientError {
    fn from(e: RedisError) -> Self {
        ActixRedisClientError(format!("{}", e))
    }
}
