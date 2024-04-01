//! Authentication and authorization.

mod access_key;
mod authentication;
mod authorization_provider;
mod client_credentials;
mod security_token;
mod session_id;
mod user_session;

pub(crate) use security_token::ParseSecurityTokenError;

pub use access_key::{AccessKeyId, SecretAccessKey};
pub use authentication::Authentication;
pub use authorization_provider::AuthorizationProvider;
pub use client_credentials::ClientCredentials;
pub use security_token::SecurityToken;
pub use session_id::SessionId;
pub use user_session::UserSession;

#[cfg(feature = "jwt")]
mod jwt_claims;

#[cfg(feature = "jwt")]
pub(crate) use jwt_claims::{default_time_tolerance, default_verification_options};

#[cfg(feature = "jwt")]
pub use jwt_claims::{JwtClaims, JwtHmacKey};
