use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use uuid::Uuid;

use crate::{User, UserRepository};

pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

pub struct HashMapStorage<K, V> {
    storage: HashMap<K, V>,
}
impl<K, V> HashMapStorage<K, V> {
    pub fn new() -> Self {
        HashMapStorage {
            storage: HashMap::new(),
        }
    }
}

impl<K, V> Storage<K, V> for HashMapStorage<K, V>
where
    K: Eq + Hash,
{
    fn set(&mut self, key: K, val: V) {
        self.storage.insert(key, val);
    }

    fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        self.storage.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.storage.remove(key)
    }
}

#[derive(Clone)]
pub struct UserRepositoryDynamic {
    storage: Rc<RefCell<Box<dyn Storage<Uuid, User>>>>,
}

impl UserRepositoryDynamic {
    pub fn new(storage: Box<dyn Storage<Uuid, User>>) -> Self {
        UserRepositoryDynamic {
            storage: Rc::new(RefCell::from(storage)),
        }
    }

    fn set(&self, key: Uuid, val: User) -> Result<(), &'static str> {
        if let Some(_) = self.get(key) {
            Err("user with such key already exists")
        } else {
            self.storage.borrow_mut().set(key, val);
            Ok(())
        }
    }

    fn update(&self, key: Uuid, val: User) -> Result<(), &'static str> {
        if let Some(_) = self.get(key) {
            self.storage.borrow_mut().set(key, val);
            Ok(())
        } else {
            Err("user with such key not found")
        }
    }

    fn get(&self, key: Uuid) -> Option<User> {
        self.storage.borrow().get(&key).map(|f| f.clone())
    }

    fn remove(&self, key: Uuid) -> Option<User> {
        self.storage.borrow_mut().remove(&key)
    }
}

impl UserRepository for UserRepositoryDynamic {
    fn set(&self, val: User) -> Result<(), &'static str> {
        self.set(val.id, val)
    }

    fn update(&self, val: User) -> Result<(), &'static str> {
        self.update(val.id, val)
    }

    fn get(&self, key: Uuid) -> Option<User> {
        self.get(key)
    }

    fn remove(&self, key: Uuid) -> Option<User> {
        self.remove(key)
    }
}
