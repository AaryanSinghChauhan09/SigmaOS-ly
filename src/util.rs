//! ly model — lightweight display manager.

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub uid: u32,
    pub session: String,
    pub logged_in: bool,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub automatic_login: Option<String>,
    pub users: Vec<User>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub config: Config,
}

impl Default for Model {
    fn default() -> Self { Self { config: Config { automatic_login: None, users: vec![] } } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_user(&mut self, name: &str, uid: u32, session: &str) {
        self.config.users.push(User { name: name.into(), uid, session: session.into(), logged_in: false });
    }

    pub fn login(&mut self, name: &str) -> bool {
        if let Some(u) = self.config.users.iter_mut().find(|u| u.name == name) {
            u.logged_in = true;
            true
        } else { false }
    }

    pub fn logout(&mut self, name: &str) -> bool {
        if let Some(u) = self.config.users.iter_mut().find(|u| u.name == name) {
            u.logged_in = false;
            true
        } else { false }
    }

    pub fn list_users(&self) -> &[User] { &self.config.users }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn login_logout() {
        let mut l = Model::new();
        l.add_user("eve", 1000, "i3");
        assert!(l.login("eve"));
        assert!(l.logout("eve"));
    }
}
