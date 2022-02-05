#[derive(Debug)]
pub struct User {
    pub name: String,
    pub guess_count: u32
}

impl User{
    pub fn build_user(name:&str) -> User {
        User{
            name:  name.to_string(),
            guess_count: 0
        }
    }
}
impl User {
    //increment guess count , while guess are being made!
    pub fn guess(&mut self) -> () {
        self.guess_count=self.guess_count+1
    }
}
