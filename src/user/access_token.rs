use jsonwebtoken::{DecodingKey, EncodingKey, Header};

const ACCESS_TOKEN_TTL: chrono::Duration = chrono::Duration::hours(12);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    pub(crate) fn sub(&self) -> &str {
        &self.sub
    }

    pub(crate) fn exp(&self) -> usize {
        self.exp
    }
}

/// Encode a claim into an access token with JWT format.
pub(crate) fn encode(sub: crate::user::Id, exp: Option<usize>, secret: &str) -> String {
    let exp = exp.unwrap_or((chrono::Utc::now() + ACCESS_TOKEN_TTL).timestamp() as usize);

    let claims = Claims {
        sub: sub.value().into(),
        exp,
    };

    let value = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    value
}

/// Decode an access token with JWT format into a claim.
pub(crate) fn decode(token: &str, secret: &str) -> Result<Claims, DecodeError> {
    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Default::default(),
    )
    .map(|data| data.claims)
    .map_err(|err| match err.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => DecodeError::Expired,
        _ => DecodeError::Tempered,
    })
}

#[derive(Debug, PartialEq)]
pub(crate) enum DecodeError {
    Expired,
    Tempered,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_and_decode_ok() {
        let sub = crate::user::Id::new();
        let secret = "secret";

        let token = encode(sub.clone(), None, secret);

        assert!(!token.is_empty());

        let claims = decode(&token, secret).unwrap();
        assert_eq!(claims.sub(), sub.value());
        // token must be valid for 12 hours
        assert!(
            (claims.exp()
                // About `- chrono::Duration::minutes(1)`: To offset the elapsed time since the token was generated.
                > (chrono::Utc::now() - chrono::Duration::minutes(1) + ACCESS_TOKEN_TTL).timestamp()
                    as usize)
                && (claims.exp() <= (chrono::Utc::now() + ACCESS_TOKEN_TTL).timestamp() as usize)
        );
    }

    #[test]
    fn test_decode_err_expired() {
        let sub = crate::user::Id::new();
        let secret = "secret";
        let token = encode(sub.clone(), Some(1_000), secret);

        let claims = decode(&token, secret);

        assert_eq!(claims.unwrap_err(), DecodeError::Expired);
    }

    #[test]
    fn test_decode_err_token_tampered() {
        let sub = crate::user::Id::new();
        let secret = "secret";
        let token = encode(sub.clone(), None, secret);

        let mut parts = token.split('.').collect::<Vec<_>>();
        parts[1] = "tampered";
        let token = parts.join(".");

        let claims = decode(&token, secret);

        assert_eq!(claims.unwrap_err(), DecodeError::Tempered);
    }

    #[test]
    fn test_decode_err_wrong_secret() {
        let sub = crate::user::Id::new();
        let secret = "secret";
        let token = encode(sub.clone(), None, secret);

        let claims = decode(&token, "wrong_secret");

        assert_eq!(claims.unwrap_err(), DecodeError::Tempered);
    }
}
