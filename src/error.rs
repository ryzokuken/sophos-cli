use std::fmt;

#[derive(Debug)]
pub struct SophosdError {
    details: String,
}

impl SophosdError {
    pub fn new_username() -> SophosdError {
        SophosdError { details: String::from("invalid username")}
    }

    pub fn new_password() -> SophosdError {
        SophosdError { details: String::from("invalid password")}
    }
}

impl fmt::Display for SophosdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl std::error::Error for SophosdError {
    fn description(&self) -> &str {
        &self.details
    }
}

