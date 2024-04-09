use super::Command;

pub struct Create {
    pub name: String,

    pub permissions: Vec<String>,
}
impl Command for Create {}

pub struct Get {
    pub role: String,
}
impl Command for Get {}

pub struct GetAll {}
impl Command for GetAll {}

pub struct Assign {
    pub user_id: i32,

    pub role: String,
}
impl Command for Assign {}

pub struct Update {
    pub role: String,

    pub name: Option<String>,

    pub permissions: Option<Vec<String>>,
}
impl Command for Update {}

pub struct Delete {
    pub role: String,
}
impl Command for Delete {}
