pub mod transfers;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;