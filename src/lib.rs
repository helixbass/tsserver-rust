mod js_typing;
mod services;
mod tsserver;

use js_typing::shared::find_argument;

pub use tsserver::server::start;
