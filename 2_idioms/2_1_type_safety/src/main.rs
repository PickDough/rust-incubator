use std::{any::{Any, TypeId}, collections::BTreeMap, iter::Map, marker::PhantomData};

mod post {
    #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Id(pub u64);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(pub String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(pub String);
}

mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);
}

trait PostState {}
#[derive(Debug, Clone)]
struct New;

impl PostState for New {}
#[derive(Debug, Clone)]
struct UnModerated;

impl PostState for UnModerated {}
#[derive(Debug, Clone)]
struct Published;

impl PostState for Published {}
#[derive(Debug, Clone)]
struct Deleted;

impl PostState for Deleted {}

#[derive(Debug, Clone)]
struct Post<S> {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
    state: PhantomData<S>,
}

impl<S> Post<S> {
    fn from<T>(self) -> Post<T> {
        Post {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            state: Default::default(),
        }
    }

    fn id(&self) -> &post::Id {
        &self.id
    }

    fn body(&self) -> &post::Body {
        &self.body
    }

    fn title(&self) -> &post::Title {
        &self.title
    }
}

impl Post<New> {
    fn new(id: post::Id, user_id: user::Id, title: post::Title, body: post::Body) -> Post<New> {
        Post {
            id,
            user_id,
            title,
            body,
            state: Default::default(),
        }
    }
}

impl Post<New> {
    fn publish(self) -> Post<UnModerated> {
        Post::from(self)
    }
}

impl Post<UnModerated> {
    fn allow(self) -> Post<Published> {
        Post::from(self)
    }

    fn deny(self) -> Post<Deleted> {
        Post::from(self)
    }
}

impl Post<Published> {
    fn delete(self) -> Post<Deleted> {
        Post::from(self)
    }
}

enum PostStates {
    New(Post<New>),
    UnModerated(Post<UnModerated>),
    Published(Post<Published>),
    Deleted(Post<Deleted>),
}

impl PostStates {}

struct PostStore(BTreeMap<post::Id, (Post<Box<dyn PostState>>, TypeId)>);

impl PostStore {
    fn new() -> Self {
        PostStore { 0: BTreeMap::new() }
    }

    fn add_post<S: PostState + 'static>(&mut self, post: Post<S>) {
        self.0.insert(post.id().clone(), (post.from::<Box<dyn PostState>>(), TypeId::of::<S>()));
    }

    fn map_post_by_id<'a, F, U>(&'a self, post_id: &post::Id, f: F) -> Option<&'a U>
    where
        F: FnOnce(&Post<Box<dyn PostState>>) -> &U,
    {
        self.0
            .get(post_id)
            .and_then(|v| Some(f(&v.0)))
    }

    fn take<S: PostState + 'static>(&mut self, post_id: &post::Id) -> Option<Post<S>> {
        self.0
        .remove(post_id)
        .and_then(|v| {
            if v.1 != TypeId::of::<S>() {
                None
            } else {
                Some(v.0.from::<S>())
            }
        })
    }
}

fn main() {
    let post = Post::new(
        post::Id(1),
        user::Id(1),
        post::Title("Fresh Post".to_owned()),
        post::Body("Blah blah blah".to_owned()),
    );
    let id = post.id().clone();

    let mut store = PostStore::new();
    store.add_post(post);

    let map = store.map_post_by_id(&id, |post| post.title());

    println!("{:?}", map);

    println!("{:?}", store.take::<New>(&id))
}
