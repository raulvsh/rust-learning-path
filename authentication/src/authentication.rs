pub struct User {
    username: String,
    password_hash: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            password_hash: hash_password(password).to_string(),
        }
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn set_password(&mut self, new_password: &str) {
        self.password_hash = hash_password(&new_password.to_owned()).to_string();
    }

    pub fn get_password(&self) -> &String {
        &self.password_hash
    }
}

fn hash_password(input: &str) -> &str {
    &input
}
