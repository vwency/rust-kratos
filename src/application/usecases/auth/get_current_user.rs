use crate::domain::entities::user::UserProfile;
use crate::domain::ports::{IdentityError, IdentityPort};

pub struct GetCurrentUserUseCase {
    identity_port: Box<dyn IdentityPort>,
}

impl GetCurrentUserUseCase {
    pub fn new(identity_port: Box<dyn IdentityPort>) -> Self {
        Self { identity_port }
    }

    pub async fn execute(&self, cookie: Option<&str>) -> Result<UserProfile, IdentityError> {
        let cookie = cookie.ok_or(IdentityError::NotAuthenticated)?;

        let user = self.identity_port.get_current_user(cookie).await?;

        Ok(user)
    }
}
