#![cfg(test)]
#![allow(dead_code)]

// ANCHOR: example
use futures::future::{BoxFuture, FutureExt};

fn recursif() -> BoxFuture<'static, ()> {
    async move {
        recursif().await;
        recursif().await;
    }.boxed()
}
// ANCHOR_END: example
