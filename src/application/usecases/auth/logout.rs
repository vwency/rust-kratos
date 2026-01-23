use crate::domain::ports::{SessionError, SessionPort};

pub struct LogoutUseCase {
    session_port: Box<dyn SessionPort>,
}

#[allow(unused)]
impl LogoutUseCase {
    pub fn new(session_port: Box<dyn SessionPort>) -> Self {
        Self { session_port }
    }

    pub async fn execute(&self, cookie: Option<&str>) -> Result<(), SessionError> {
        let cookie = cookie.ok_or(SessionError::NotAuthenticated)?;

        self.session_port.logout(cookie).await
    }
}
