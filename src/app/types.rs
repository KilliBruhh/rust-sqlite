use std::future::Future;
use std::pin::Pin;
use crate::app::session_context::SessionStatus;

pub type CommandHandler = fn(String, &mut SessionStatus) -> Pin<Box<dyn Future<Output = ()> + Send>>;