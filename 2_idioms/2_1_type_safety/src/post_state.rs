
pub trait PostState: Default + Into<PostStateEnum> + TryFrom<PostStateEnum> {}

#[derive(Debug, Default, Clone)]
pub struct New;
impl PostState for New {}
impl Into<PostStateEnum> for New {
    fn into(self) -> PostStateEnum {
        PostStateEnum::New
    }
}
impl TryFrom<PostStateEnum> for New {
    type Error = ();

    fn try_from(value: PostStateEnum) -> Result<Self, Self::Error> {
        match value {
            PostStateEnum::New => Ok(Default::default()),
            PostStateEnum::UnModerated => Err(()),
            PostStateEnum::Published => Err(()),
            PostStateEnum::Deleted => Err(()),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct UnModerated;
impl PostState for UnModerated {}
impl Into<PostStateEnum> for UnModerated {
    fn into(self) -> PostStateEnum {
        PostStateEnum::UnModerated
    }
}
impl TryFrom<PostStateEnum> for UnModerated {
    type Error = ();

    fn try_from(value: PostStateEnum) -> Result<Self, Self::Error> {
        match value {
            PostStateEnum::New => Err(()),
            PostStateEnum::UnModerated => Ok(Default::default()),
            PostStateEnum::Published => Err(()),
            PostStateEnum::Deleted => Err(()),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Published;
impl PostState for Published {}
impl Into<PostStateEnum> for Published {
    fn into(self) -> PostStateEnum {
        PostStateEnum::Published
    }
}
impl TryFrom<PostStateEnum> for Published {
    type Error = ();

    fn try_from(value: PostStateEnum) -> Result<Self, Self::Error> {
        match value {
            PostStateEnum::New => Err(()),
            PostStateEnum::UnModerated => Err(()),
            PostStateEnum::Published => Ok(Default::default()),
            PostStateEnum::Deleted => Err(()),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Deleted;
impl PostState for Deleted {}
impl Into<PostStateEnum> for Deleted {
    fn into(self) -> PostStateEnum {
        PostStateEnum::Deleted
    }
}
impl TryFrom<PostStateEnum> for Deleted {
    type Error = ();

    fn try_from(value: PostStateEnum) -> Result<Self, Self::Error> {
        match value {
            PostStateEnum::New => Err(()),
            PostStateEnum::UnModerated => Err(()),
            PostStateEnum::Published => Err(()),
            PostStateEnum::Deleted => Ok(Default::default()),
        }
    }
}

#[derive(Clone)]
pub enum PostStateEnum {
    New,
    UnModerated,
    Published,
    Deleted,
}
