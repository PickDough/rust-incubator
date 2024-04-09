#![allow(dead_code, unused)]
use im::{HashMap, Vector};

pub trait UsersRepository {
    fn find(&self, id: u64) -> Option<&User>;
    fn find_many(&self, ids: &[u64]) -> Vector<&User>;
    fn find_by<F>(&self, by: F) -> Vector<&User>
    where
        F: Fn(&User) -> bool;
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub nickname: String,
}

impl User {
    pub fn new(id: u64, nickname: String) -> Self {
        Self { id, nickname }
    }
}

#[derive(Debug, Default, Clone)]
pub struct UserVec(HashMap<u64, User>);
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

impl UserVec {
    pub fn insert(&mut self, user: User) {
        self.0.insert(user.id, user);
    }
}
