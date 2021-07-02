extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub static MB: usize = 1024*1024;

pub mod download;
pub mod upload;