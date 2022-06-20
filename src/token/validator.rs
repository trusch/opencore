use jsonwebtoken::{decode, DecodingKey, Validation};

use super::claims::Claims;
use super::error::Error;
use crate::token::context::Context;


pub struct Validator {
    decoding_key: DecodingKey,
}

impl std::fmt::Debug for Validator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validator()")
    }
}

impl Validator {
    pub fn new(secret: &'static str) -> Validator {
        Validator {
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
        }
    }

    pub fn get_context<T>(&self, req: &tonic::Request<T>) -> Result<Context, Error> {
        let claims = self.get_access_token_claims(req)?;
        let fencing_token = match req.metadata().get("X-Fencing-Token") {
            Some(data) => {
                let token = std::str::from_utf8(data.as_bytes())?;
                tracing::debug!("X-Fencing-Token: {}", &token);

                let parts = token.split('#').collect::<Vec<&str>>();
                if parts.len() != 2 {
                    return Err(Error::Parse(
                        "failed to parse X-Fencing-Token header".into(),
                    ));
                }
                let lock_id = parts[0];
                let fence_id = parts[1].parse::<i64>()?;
                Some((lock_id.to_string(), fence_id))
            }
            None => None,
        };
        Ok(Context {
            claims,
            fencing_token,
        })
    }

    pub fn get_access_token_claims<T>(&self, req: &tonic::Request<T>) -> Result<Claims, Error> {
        let token = Validator::get_token(req)?;
        let claims = self.validate(&token)?;
        match claims.rfs {
            false => Ok(claims),
            true => Err(Error::Validate(
                "expected access_token found refresh_token".to_string(),
            )),
        }
    }

    fn get_token<T>(req: &tonic::Request<T>) -> Result<String, Error> {
        let value = match req.metadata().get("Authorization") {
            Some(data) => data,
            None => return Err(Error::NotFound),
        };

        let bs = value.as_bytes();

        const PREFIX_LENGTH: usize = "Bearer ".len();
        if bs.len() < PREFIX_LENGTH + 1 {
            return Err(Error::NotFound);
        }

        let token = std::str::from_utf8(&bs[PREFIX_LENGTH..])?;
        Ok(token.to_string())
    }

    pub fn validate(&self, token: &str) -> Result<Claims, Error> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::default())?;
        Ok(token_data.claims)
    }
}
