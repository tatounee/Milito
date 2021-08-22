pub struct Cheat {
    password: Vec<char>,
    password_ptr: usize,
    active: bool,
}

impl Cheat {
    pub fn new(password: &str) -> Self {
        let no_password = password.is_empty();

        let mut cheat = Self {
            password: password.chars().collect(),
            password_ptr: 0,
            active: false,
        };

        cheat.active = no_password;

        cheat
    }

    pub fn type_key(&mut self, key: char) {
        if self.password_ptr < self.password.len() {
            if self.password[self.password_ptr] == key {
                self.password_ptr += 1
            } else {
                self.password_ptr = 0
            }
            self.active = self.password_ptr == self.password.len();
        }
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.active
    }
}
