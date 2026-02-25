pub struct User {
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(email: String, password: String) -> User {
        User {
            email,
            password,
        }
    }

    pub fn hash_password(&self) -> String {
        let pass = self.password.clone();
        bcrypt::hash(&pass, DEFAULT_COST).expect("error while hashing password")
    }

    pub fn verify_password(&self, password: String) -> bool {
        bcrypt::verify(password, &self.password).expect("error while verifying password")
    }
}