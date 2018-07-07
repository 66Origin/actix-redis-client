extern crate actix;
pub extern crate redis;

use actix::prelude::*;
pub use redis::Cmd;

use std::marker::PhantomData;

#[macro_export]
macro_rules! redis_cmd {
    ($cmd: expr, $($args:expr),* ) => ({
        use redis::cmd;
        cmd($cmd.into())
            $(.arg($args.into()))*
    })
}

pub struct Command<T> {
    cmd: redis::Cmd,
    _marker: PhantomData<T>,
}

impl<T> Command<T> {
    pub fn new(cmd: Cmd) -> Self {
        Command {
            cmd,
            _marker: PhantomData::default(),
        }
    }
}

impl<T> Message for Command<T> {
    type Result = Result<T, redis::RedisError>;
}

pub struct RedisExecutorSync(Box<redis::ConnectionLike>);
impl RedisExecutorSync {
    fn new(client: Box<redis::ConnectionLike>) -> Self {
        RedisExecutorSync(client)
    }

    pub fn start<F>(threads: usize, client_factory: F) -> Addr<Syn, Self>
    where
        F: Fn() -> redis::ConnectionLike + Send + Sync + 'static,
    {
        SyncArbiter::start(threads, move || Self::new(client_factory()))
    }
}

impl Actor for RedisExecutorSync {
    type Context = SyncContext<Self>;
}

impl<T> Handler<Command<T>> for RedisExecutorSync {
    type Result = Result<T, redis::RedisError>;

    fn handle(&mut self, cmd: Command<T>, _: &mut Self::Context) -> Self::Result {
        cmd.query(self.0)
    }
}

impl Handler<RequestWithReply> for NATSExecutorSync {
    type Result = Result<Vec<u8>, nats::NatsError>;

    fn handle(&mut self, mut msg: RequestWithReply, _: &mut Self::Context) -> Self::Result {
        msg.inbox = Some(self.0.make_request(&msg.subject, &msg.data)?);
        Ok(self.0.wait()?.msg.into())
    }
}
