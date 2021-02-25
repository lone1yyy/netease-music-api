pub(crate) mod crypto;
pub(crate) mod forward;
pub(crate) mod request;

pub use self::crypto::crypto_test;
pub use self::request::{ForwordRequest, _json_test}; //export

pub trait CheckNull {
    fn check_null(&self, key: &str) -> bool;
}

pub trait CookieString {
    fn as_cookie_string(&self) -> String;
}
