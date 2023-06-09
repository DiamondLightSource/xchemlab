use crate::{verify_access_token, TokenVerificationError};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    TypedHeader,
};
use openidconnect::{
    core::{CoreClient, CoreTokenIntrospectionResponse},
    AccessToken,
};

#[derive(Debug, thiserror::Error)]
pub enum AuthTokenExtractionError {
    #[error("Could not retrieve token from headers")]
    Unavailable,
    #[error("Could not verify token")]
    Unverifiable(#[from] TokenVerificationError),
}

pub struct ExtractAuthToken(Result<CoreTokenIntrospectionResponse, AuthTokenExtractionError>);

impl ExtractAuthToken {
    /// Unwraps the value to [`openidconnect::core::CoreTokenIntrospectionResponse`].
    pub fn into_inner(self) -> Result<CoreTokenIntrospectionResponse, AuthTokenExtractionError> {
        self.0
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ExtractAuthToken
where
    S: Send + Sync,
    CoreClient: FromRef<S>,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self(
            async {
                let TypedHeader(Authorization(token)) =
                    TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                        .await
                        .map_err(|_| AuthTokenExtractionError::Unavailable)?;
                let client = CoreClient::from_ref(state);
                Ok(
                    verify_access_token(&AccessToken::new(token.token().to_string()), &client)
                        .await?,
                )
            }
            .await,
        ))
    }
}