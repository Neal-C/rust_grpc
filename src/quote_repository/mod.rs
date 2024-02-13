#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::needless_return)]
mod repository;
pub use repository::Repository;
