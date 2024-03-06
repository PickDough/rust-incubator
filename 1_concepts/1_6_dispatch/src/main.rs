use std::{borrow::Cow, collections::HashMap, hash::Hash};

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

struct HashMapStorage<K, V> {
    storage: HashMap<K, V>,
}
impl<K, V> HashMapStorage<K, V> {
    fn new() -> Self {
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

#[derive(Debug, Default)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

struct UserRepositoryStatic<S> {
    storage: S,
}

impl<S: Storage<u64, User>> UserRepositoryStatic<S> {
    fn new(storage: S) -> Self {
        UserRepositoryStatic { storage }
    }

    fn set(&mut self, key: u64, val: User) -> Result<(), &'static str> {
        if let Some(_) = self.get(key) {
            Err("user with such key already exists")
        } else {
            self.storage.set(key, val);
            Ok(())
        }
    }

    fn update(&mut self, key: u64, val: User) -> Result<(), &'static str> {
        if let Some(_) = self.get(key) {
            self.storage.set(key, val);
            Ok(())
        } else {
            Err("user with such key not found")
        }
    }

    fn get(&self, key: u64) -> Option<&User> {
        self.storage.get(&key)
    }

    fn remove(&mut self, key: u64) -> Option<User> {
        self.storage.remove(&key)
    }
}

struct UserRepositoryDynamic {
    storage: Box<dyn Storage<u64, User>>,
}

impl UserRepositoryDynamic {
    fn new(storage: Box<dyn Storage<u64, User>>) -> Self {
        UserRepositoryDynamic { storage }
    }

    fn set(&mut self, key: u64, val: User) -> Result<(), &'static str> {
        if let Some(_) = self.get(key) {
            Err("user with such key already exists")
        } else {
            self.storage.set(key, val);
            Ok(())
        }
    }

    fn update(&mut self, key: u64, val: User) -> Result<(), &'static str> {
        if let Some(_) = self.get(key) {
            self.storage.set(key, val);
            Ok(())
        } else {
            Err("user with such key not found")
        }
    }

    fn get(&self, key: u64) -> Option<&User> {
        self.storage.get(&key)
    }

    fn remove(&mut self, key: u64) -> Option<User> {
        self.storage.remove(&key)
    }
}

fn main() {
    let mut static_repo = UserRepositoryStatic::new(HashMapStorage::new());

    println!("Insert: {:?}", static_repo.set(1, User::default()));
    println!("Insert fail: {:?}", static_repo.set(1, User::default()));
    println!("Get: {:?}", static_repo.get(1));
    println!(
        "Update + Get: {:?} + {:?}",
        static_repo.update(
            1,
            User {
                email: "user@mail.com".into(),
                ..Default::default()
            }
        ),
        static_repo.get(1)
    );
    println!("Remove: {:?}", static_repo.remove(1));
    println!("Get Empty: {:?}", static_repo.get(1));

    print!("\n\n\n");

    let mut dynamic_repo = UserRepositoryDynamic::new(Box::new(HashMapStorage::new()));
    println!("Insert: {:?}", dynamic_repo.set(1, User::default()));
    println!("Insert fail: {:?}", dynamic_repo.set(1, User::default()));
    println!("Get: {:?}", dynamic_repo.get(1));
    println!(
        "Update + Get: {:?} + {:?}",
        dynamic_repo.update(
            1,
            User {
                email: "user@mail.com".into(),
                ..Default::default()
            }
        ),
        dynamic_repo.get(1)
    );
    println!("Remove: {:?}", dynamic_repo.remove(1));
    println!("Get Empty: {:?}", dynamic_repo.get(1));
}
