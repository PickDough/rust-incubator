#![allow(dead_code, unused)]

use std::borrow::Cow;

use uuid::Uuid;

use dispatch::UserRepositoryDynamic;

use crate::dispatch::HashMapStorage;

mod dispatch;

trait Command {}

trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;
    fn handle_command(&self, cmd: &C, ctx: &Self::Context) -> Self::Result;
}

#[derive(Debug, Clone)]
struct User {
    id: Uuid,
    email: Cow<'static, str>,
    activated: bool,
}

impl User {
    fn new(email: Cow<'static, str>) -> Self {
        User {
            id: Uuid::new_v4(),
            email,
            activated: false,
        }
    }
}

trait UserRepository {
    fn set(&self, val: User) -> Result<(), &'static str>;

    fn update(&self, val: User) -> Result<(), &'static str>;

    fn get(&self, key: Uuid) -> Option<User>;

    fn remove(&self, key: Uuid) -> Option<User>;
}

#[derive(Debug)]
struct UserError(&'static str);

#[derive(Default, Copy, Clone)]
struct CreateUser {
    should_activate: bool,
}

impl Command for CreateUser {}

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = Result<(), UserError>;

    // Shouldn't we consume Command on execution?
    fn handle_command(&self, cmd: &CreateUser, user_repo: &Self::Context) -> Self::Result {
        user_repo
            .set(User {
                id: self.id,
                email: self.email.clone(),
                activated: cmd.should_activate,
            })
            .map_err(UserError)
    }
}

fn main() {
    let u = User::new("user@mail.com".into());
    let repo = UserRepositoryDynamic::new(Box::new(HashMapStorage::new()));

    println!(
        "{:?}",
        u.handle_command(
            &CreateUser {
                should_activate: true
            },
            &repo
        )
    );
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use uuid::Uuid;

    use crate::{CommandHandler, CreateUser, User, UserRepository};

    struct UserRepositoryMock {
        should_activate: bool,
    }

    impl UserRepository for UserRepositoryMock {
        fn set(&self, val: User) -> Result<(), &'static str> {
            assert_eq!(self.should_activate, val.activated);

            Ok(())
        }

        fn update(&self, _val: User) -> Result<(), &'static str> {
            unimplemented!()
        }

        fn get(&self, _key: Uuid) -> Option<User> {
            unimplemented!()
        }

        fn remove(&self, _key: Uuid) -> Option<User> {
            unimplemented!()
        }
    }

    #[test_case(true)]
    #[test_case(false)]
    fn should_activate_true(should_activate: bool) {
        let u = User::new("user@mail.com".into());
        let repo = UserRepositoryMock { should_activate };

        assert!(u
            .handle_command(&CreateUser { should_activate }, &repo)
            .is_ok())
    }
}
