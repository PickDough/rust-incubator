use cucumber::{gherkin::Step, given, then, when, World};
use im_collections::{User, UserVec, UsersRepository};

#[derive(Debug, Default, World)]
pub struct ImWorld {
    user_vec: UserVec,
}

#[given("A List of followings users")]
fn a_user_vec_with_the_following_users(world: &mut ImWorld) {
    world.user_vec.insert(User::new(1, "Steve".to_string()));
    world.user_vec.insert(User::new(2, "Bill".to_string()));
    world.user_vec.insert(User::new(3, "Elon".to_string()));
}

#[when("The List is copied and modified")]
fn the_list_copied_and_modified(world: &mut ImWorld) {
    let mut copy = world.user_vec.clone();
    copy.insert(User::new(4, "Jeff".to_string()));
}

#[then("The original List remains the same")]
fn the_original_list_remains_the_same(world: &mut ImWorld) {
    assert!(world.user_vec.find(4).is_none());
}

#[given(expr = "A List with {string}")]
fn a_list_with_some_users(world: &mut ImWorld, users: String) {
    for (i, u) in users.split(", ").enumerate() {
        //println!("{u}");
        world.user_vec.insert(User::new(i as u64, u.to_string()));
    }
}

#[then(expr = "The {word} is in the List")]
fn the_user_is_in_the_list(world: &mut ImWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) {
            let user = row[0].as_str();

            assert!(world.user_vec.find_by(|u| u.nickname == user).len() == 1);
        }
    }
}

fn main() {
    futures::executor::block_on(ImWorld::run("tests/features/collections.feature"));
}
