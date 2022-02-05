#[derive(Debug)]
pub struct User {
    pub name: String,
    pub guess_count: u32
}

pub fn build_user(name:String) -> User {
    User{
        name:  String::from(name),
        guess_count: 0
    }
}
pub fn update_count(count:u32,user:&User) -> User {
    User{
        guess_count: count,
        name: user.name.clone()
    }
}