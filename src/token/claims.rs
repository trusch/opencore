use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use super::error::Error;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: usize,       // Expiration time (as UTC timestamp)
    pub iat: usize,       // Issued at (as UTC timestamp)
    pub iss: String,      // Issuer
    pub nbf: usize,       // Not Before (as UTC timestamp)
    pub sub: String,      // Subject (whom token refers to)
    pub grp: Vec<String>, // List of groups
    pub adm: bool,        // IsAdmin
    pub rfs: bool,        // IsRefreshToken
}

impl Claims {
    pub fn admin() -> Self {
        Claims {
            adm: true,
            ..Claims::default()
        }
    }

    pub fn principals(&self) -> Result<Vec<Uuid>, Error> {
        let mut res = vec![];
        let mut items = self.grp.clone();
        items.push(self.sub.clone());
        for item in items {
            res.push(Uuid::parse_str(&item)?);
        }
        Ok(res)
    }

    pub fn has_group(&self, grp: &String) -> bool {
        self.grp.contains(grp)
    }
}
