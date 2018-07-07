#![warn(clippy)]
extern crate actix;
pub extern crate redis;

use actix::prelude::*;
use std::marker::PhantomData;

mod error;
pub use self::error::*;

pub type ActixRedisClientResult<T> = Result<T, ActixRedisClientError>;

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

pub struct RedisExecutorSync(redis::Client);
impl RedisExecutorSync {
    fn new(client: redis::Client) -> Self {
        RedisExecutorSync(client)
    }

    pub fn start<F>(threads: usize, client_factory: F) -> Addr<Syn, Self>
    where
        F: Fn() -> redis::Client + Send + Sync + 'static,
    {
        SyncArbiter::start(threads, move || Self::new(client_factory()))
    }

    pub fn get_connection(&self) -> Result<redis::Connection, ActixRedisClientError> {
        match self.0.get_connection() {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

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
        match cmd.cmd.query(&mut self.0) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}
