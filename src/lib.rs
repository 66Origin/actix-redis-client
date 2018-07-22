#[macro_use]
extern crate failure;
extern crate actix;
pub extern crate redis;

use actix::prelude::*;
use std::marker::PhantomData;

mod error;
pub use self::error::*;

/// Result type
pub type ActixRedisClientResult<T> = Result<T, ActixRedisClientError>;

/// Basic command that can be sent to Redis client
/// The redis crate is re-exposed to make use of `redis::cmd()` function to generate commands
pub struct Command<T> {
    cmd: redis::Cmd,
    _marker: PhantomData<T>,
}

impl<T> Command<T> {
    pub fn new(cmd: redis::Cmd) -> Self {
        Command {
            cmd,
            _marker: PhantomData::default(),
        }
    }
}

impl<T: 'static> Message for Command<T> {
    type Result = ActixRedisClientResult<T>;
}

/// Actor to give to Actix to do the background processing of Redis messages
pub struct RedisExecutorSync(redis::Client);
impl RedisExecutorSync {
    fn new(client: redis::Client) -> Self {
        RedisExecutorSync(client)
    }

    /// Starts the executor. Give it a number of threads and a factory `Fn() -> redis::Client` that handles client creation and you're good to go.
    pub fn start<F>(threads: usize, client_factory: F) -> Addr<Self>
    where
        F: Fn() -> redis::Client + Send + Sync + 'static,
    {
        SyncArbiter::start(threads, move || Self::new(client_factory()))
    }

    /// Accessor to retrieve current Redis connection
    pub fn get_connection(&self) -> Result<redis::Connection, ActixRedisClientError> {
        match self.0.get_connection() {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    /// Accessor to retrieve current PubSub Redis connection
    pub fn get_pubsub(&self) -> Result<redis::PubSub, ActixRedisClientError> {
        match self.0.get_pubsub() {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

impl Actor for RedisExecutorSync {
    type Context = SyncContext<Self>;
}

impl<T: redis::FromRedisValue + 'static> Handler<Command<T>> for RedisExecutorSync {
    type Result = ActixRedisClientResult<T>;

    fn handle(&mut self, cmd: Command<T>, _: &mut Self::Context) -> Self::Result {
        match cmd.cmd.query(&self.0) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}
