use core::str;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::{User, UserRole};

#[derive(Debug, Serialize, Deserialize, Validate, Clone, Default)]
pub struct RegisterDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Invalid email")
    )]
    pub email: String,
    #[validate(
        length(min = 8, message = "Password is required and must be at least 8 characters")
    )]
    pub password: String,
    #[validate(
        length(min = 1, message = " confirm password is required"),
        must_match(other = password, message = "Passwords do not match")
    )]
    pub confirm_password: String,
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone, Default)]
pub struct LoginDto {
    #[validate(length(min = 1, message = "Email is required"), email(message = "Invalid email"))]
    pub email: String,
    #[validate(
        length(min = 8, message = "Password is required and  must be at least 8 characters")
    )]
    pub password: String
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDto{
    #[validate(range(min = 1))]
    pub page:Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit:Option<usize>
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterUserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl FilterUserDto{
    pub fn filter_user(user: &User) -> Self{
        FilterUserDto{
            id: user.id.to_string(),
            name: user.name.clone(),
            email: user.email.to_owned(),
            role: user.role.to_str().to_string(),
            verified: user.verified,
            created_at: user.created_at.unwrap().to_string(),
            updated_at: user.updated_at.unwrap().to_string(),
        }
    }
    pub fn filter_users(user: &[User]) -> Vec<FilterUserDto>{
        user.iter().map(FilterUserDto::filter_user).collect()
    }
}
#[derive(Debug,Serialize, Deserialize, Validate)]
pub struct UserData{
    pub user: FilterUserDto,
}
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserResponseDto{
    pub data: UserData,
    pub status: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponceDto{
    pub status: String,
    pub users: Vec<FilterUserDto>,
    pub results: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponceDto{
    pub status: String,
    pub token: String
}
#[derive(Serialize, Deserialize)]
pub struct Responce{
    pub status: &'static str,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone, Default)]
pub struct NameUpdateDto{
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RoleUpdateDto {
    pub role: UserRole,
}

// fn validate_user_role(role: &UserRole) -> Result<(),validator::ValidationError>{
//     match role { 
//         UserRole::Admin | UserRole::User => Ok(()),
//         _=> Err(validator::ValidationError::new("Invalid role")),
//     }
// }
#[derive(Debug, Serialize, Deserialize, Validate, Default, Clone)]
pub struct UserPasswordUpdateDto {
    #[validate(
        length(min = 6, message = " New password is required must be at least 6 characters"),
        must_match(other = new_password, message = "Passwords do not match")
    )]
    pub new_password: String,

    #[validate(
        length(min = 6, message = "New password confirm is required and must be at least 6 characters"),
        must_match(other = "new_password", message="new passwords do not match")
    )]
    pub new_password_confirm: String,

    #[validate(
        length(min = 6, message = "Old password is required and must be at least 6 characters")
    )]
    pub old_password: String,

}

#[derive(Serialize, Deserialize, Validate)]
pub struct VerifyEmailQueryDto {
    #[validate(length(min = 1, message = "Token is required."),)]
    pub token: String,
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone)]
pub struct ForgotPasswordRequestDto {
    #[validate(length(min = 1, message = "Email is required"), email(message = "Email is invalid"))]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ResetPasswordRequestDto {
    #[validate(length(min = 1, message = "Token is required."),)]
    pub token: String,

    #[validate(
        length(min = 6, message = "New password is required and must be at least 6 characters")
    )]
    pub new_password: String,

    #[validate(
        length(min = 6, message = "New password confirm is required and confirm must be at least 6 characters"),
        must_match(other = "new_password", message="new passwords do not match")
    )]
    pub new_password_confirm: String,
}
    