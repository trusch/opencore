use crate::token::claims::Claims;

#[derive(Debug, Clone)]
pub struct Context {
    pub claims: Claims,
    pub fencing_token: Option<(String, i64)>,
}
