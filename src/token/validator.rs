use jsonwebtoken::{decode, DecodingKey, Validation};

use super::claims::Claims;
use super::error::Error;

#[derive(Debug)]
pub struct Validator {
    decoding_key: DecodingKey<'static>,
}

impl Validator {
    pub fn new(secret: &'static str) -> Validator {
        Validator {
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
        }
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
