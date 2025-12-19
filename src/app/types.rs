use std::future::Future;
use std::pin::Pin;

pub type CommandHandler = fn(String) -> Pin<Box<dyn Future<Output = ()> + Send>>;