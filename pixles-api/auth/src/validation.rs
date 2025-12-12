use crate::models::errors::BadRegisterUserRequestError;
use crate::models::requests::RegisterRequest;
use service::user as UserService;

pub struct RegistrationValidator;

impl RegistrationValidator {
    pub fn validate(request: &RegisterRequest) -> Result<(), BadRegisterUserRequestError> {
        Self::validate_username(&request.username)?;
        Self::validate_email(&request.email)?;
        Self::validate_password(&request.password)?;
        Ok(())
    }

    fn validate_username(username: &str) -> Result<(), BadRegisterUserRequestError> {
        if !UserService::is_valid_username(username) {
            return Err(BadRegisterUserRequestError::Username);
        }
        Ok(())
    }

    fn validate_email(email: &str) -> Result<(), BadRegisterUserRequestError> {
        if !UserService::is_valid_email(email) {
            return Err(BadRegisterUserRequestError::Email);
        }
        Ok(())
    }

    fn validate_password(password: &str) -> Result<(), BadRegisterUserRequestError> {
        if UserService::is_valid_password(password).is_err() {
            return Err(BadRegisterUserRequestError::Password);
        }
        Ok(())
    }
}
