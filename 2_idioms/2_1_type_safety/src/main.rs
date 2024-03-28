#![allow(dead_code, unused)]
use std::collections::BTreeMap;

use post_state::{Deleted, New, PostState, PostStateEnum, Published, UnModerated};
mod post_state;

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

#[derive(Debug, Clone)]
struct Post<S: Clone> {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
    state: S,
}

impl<S: Clone> Post<S> {
    fn from<T: Default + Clone>(self) -> Post<T> {
        Post {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            state: T::default(),
        }
    }
}

impl<S: Clone> Post<S> {
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

struct PostStore(BTreeMap<post::Id, Post<PostStateEnum>>);

impl PostStore {
    fn new() -> Self {
        PostStore { 0: BTreeMap::new() }
    }

    fn add_post<S: PostState + 'static + Clone>(&mut self, post: Post<S>) {
        self.0.insert(
            post.id().clone(),
            Post {
                id: post.id,
                user_id: post.user_id,
                title: post.title,
                body: post.body,
                state: post.state.into(),
            },
        );
    }

    fn map_post_by_id<'a, F, U>(&'a self, post_id: &post::Id, f: F) -> Option<&'a U>
    where
        F: FnOnce(&Post<PostStateEnum>) -> &U,
    {
        self.0.get(post_id).and_then(|v| Some(f(&v)))
    }

    fn take<S: PostState + 'static + Clone>(&mut self, post_id: &post::Id) -> Option<Post<S>> {
        self.0
            .remove(post_id)
            .and_then(|v| match S::try_from(v.state) {
                Ok(s) => Some(Post {
                    id: v.id,
                    user_id: v.user_id,
                    title: v.title,
                    body: v.body,
                    state: s,
                }),
                Err(_) => None,
            })
    }
}

fn main() {
    let a = Post::new(
        post::Id(1),
        user::Id(1),
        post::Title("Fresh Post".to_owned()),
        post::Body("Blah blah blah".to_owned()),
    );

    let mut b = a.clone();
    b.id = post::Id(2);
    let b = b.publish();

    let mut post_store = PostStore::new();
    post_store.add_post(a);
    post_store.add_post(b);

    let x = post_store.map_post_by_id(&post::Id(1), |p| p.title());

    println!("{:?}", post_store.take::<New>(&post::Id(1)));
    println!("{:?}", post_store.take::<New>(&post::Id(2)));
}
