use std::fmt::Debug;

pub mod clap;
pub mod roles;
pub mod users;

pub trait Command {}

pub trait CommandHandler<C: Command> {
    type Output: Debug;

    async fn handle(&self, command: C) -> Self::Output;
}
