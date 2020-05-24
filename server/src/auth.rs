use serde::{Deserialize, Serialize};

// taken from keats/jsonwebtoken (with gratitude)
mod jwt_numeric_date {
    //! Custom serialization of DateTime<Utc> to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    /// Serializes a DateTime<Utc> to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
            .single() // If there are multiple or no valid DateTimes from timestamp, return None
            .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum ClaimsType {
    Login,
    EmailVerify,
    PwdReset,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct Claims {
    /// The user id.
    pub id: i32,
    /// When the token expires.
    #[serde(with = "jwt_numeric_date")]
    pub exp: chrono::DateTime<chrono::Utc>,
    /// The type of token represented (a login token, an email verification token or a password reset token)
    pub claims_type: ClaimsType,
}

impl actix_web::FromRequest for Claims {
    type Error = actix_web::error::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match req.headers().get("x-api-token") {
            Some(token) => {
                match jwt::decode::<Claims>(
                    token.to_str().unwrap(),
                    &jwt::DecodingKey::from_secret(
                        &std::env::var("SECRET_KEY").unwrap().as_bytes(),
                    ),
                    &jwt::Validation::default(),
                ) {
                    Ok(claims) => futures::future::ok(claims.claims),
                    Err(e) => futures::future::err(actix_web::error::ErrorBadRequest(
                        "The token supplied is invalid.",
                    )),
                }
            }
            None => futures::future::err(actix_web::error::ErrorBadRequest(
                "A token was not supplied.",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_authorised_works() {
        use actix_web::FromRequest;
        use actix_web::HttpMessage;
        let key = jwt::encode(
            &jwt::Header::default(),
            &Claims {
                id: 0,
                exp: chrono::Utc::now()
                    .checked_add_signed(chrono::Duration::minutes(15))
                    .unwrap(),
                claims_type: ClaimsType::Login,
            },
            &jwt::EncodingKey::from_secret("secret".as_ref()),
        )
        .unwrap();
        let test_request =
            actix_web::test::TestRequest::with_header("x-api-token", key).to_http_request();

        let claims_result: Claims = Claims::extract(&test_request).await.unwrap();
        assert_eq!(claims_result.id, 0);
    }
    #[actix_rt::test]
    async fn test_unauthorised_doesnt_work() {}
}
