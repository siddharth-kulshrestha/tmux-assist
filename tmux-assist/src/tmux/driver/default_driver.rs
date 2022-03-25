use std::process::Command

pub struct DefaultDriver {
    path: String,
    env: String,
}

impl Driver for DefaultDriver{
    pub fn new_session(&self, NewSessionRequest) -> Result<String, Err> {
        let args = "new-session"
        
    }
}
