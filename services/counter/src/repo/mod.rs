mod mem;
mod pg;
use futures::Future;

pub trait Repository {
    type Error;

    fn count(&self) -> impl Future<Output = Result<i32, Self::Error>>;
    fn increment(&self, amount: i32) -> impl Future<Output = Result<i32, Self::Error>>;
    fn decrement(&self, amount: i32) -> impl Future<Output = Result<i32, Self::Error>>;
}

pub use {mem::*, pg::*};
