
pub struct User {
    pub(crate) active: bool,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) sign_in_count: u64,
}