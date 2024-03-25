#![allow(dead_code, unused)]
use im::{HashMap, Vector};

fn main() {
    let mut map = HashMap::new();
    map.insert(
        1u64,
        User {
            id: 1,
            nickname: "user1".to_string(),
        },
    );
    map.insert(
        2,
        User {
            id: 2,
            nickname: "user2".to_string(),
        },
    );
    map.insert(
        3,
        User {
            id: 3,
            nickname: "user3".to_string(),
        },
    );

    let map = UserVec(map);

    println!("{:?}", map.find(1));
    println!("{:?}", map.find_many(&[1, 3]));
    println!("{:?}", map.find_by(|user| user.nickname == "user2"));
}

trait UsersRepository {
    fn find(&self, id: u64) -> Option<&User>;
    fn find_many(&self, ids: &[u64]) -> Vector<&User>;
    fn find_by<F>(&self, by: F) -> Vector<&User>
    where
        F: Fn(&User) -> bool;
}

#[derive(Debug, Clone)]
struct User {
    id: u64,
    nickname: String,
}

struct UserVec(HashMap<u64, User>);
impl UsersRepository for UserVec {
    fn find(&self, id: u64) -> Option<&User> {
        self.0.get(&id)
    }

    fn find_many(&self, ids: &[u64]) -> Vector<&User> {
        ids.iter().filter_map(|id| self.0.get(id)).collect()
    }

    fn find_by<F>(&self, by: F) -> Vector<&User>
    where
        F: Fn(&User) -> bool,
    {
        self.0.values().filter(|user| by(user)).collect()
    }
}
