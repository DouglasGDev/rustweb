pub mod captcha;
pub mod login;
pub mod main_page;
pub mod not_found;
pub mod ping;
pub mod status;

pub use captcha::{captcha_page, validate_captcha};
pub use login::{login_page, login};
pub use main_page::main_page;
pub use not_found::not_found_page;
pub use ping::ping;
pub use status::status;
