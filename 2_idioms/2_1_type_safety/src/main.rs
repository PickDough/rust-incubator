use std::marker::PhantomData;

mod post {
    #[derive(Clone, Debug, PartialEq)]
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

struct New;

impl PostState for New {}

struct UnModerated;

impl PostState for UnModerated {}

struct Published;

impl PostState for Published {}

struct Deleted;

impl PostState for Deleted {}

#[derive(Clone)]
struct Post<S: PostState> {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
    state: PhantomData<S>,
}

impl<S: PostState> Post<S> {
    fn from<T: PostState>(self) -> Post<T> {
        Post { id: self.id, user_id: self.user_id, title: self.title, body: self.body, state: Default::default() }
    }
}

impl Post<New> {
    fn new(user_id: user::Id, title: post::Title, body: post::Body) -> Post<New> {
        Post { id: post::Id(0), user_id, title, body, state: Default::default() }
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

fn main() {
    let post = Post::new(user::Id(1), post::Title("Fresh Post".to_owned()), post::Body("Blah blah blah".to_owned()));

    let post = post.publish();

    // let post =  post.delete();

    let post = post.allow();

    post.delete();
   
    //let post = post.publish();
}
