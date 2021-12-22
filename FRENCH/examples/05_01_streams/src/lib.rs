#![cfg(test)]

mod stream_trait {
use {
    futures::stream::{Stream as RealStream},
    std::{
        pin::Pin,
        task::{Context, Poll},
    },
};

// ANCHOR: stream_trait
trait Stream {
    /// Le type de la valeur retournée par le flux.
    type Item;

    /// Tente de résoudre l'élément suivant dans le flux.
    /// Retourne :
    /// `Poll::Pending` s'il n'est pas encore prêt,
    /// `Poll::Ready(Some(x))` si une valeur est prête,
    /// `Poll::Ready(None)` si le flux est terminé.
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<Option<Self::Item>>;
}
// ANCHOR_END: stream_trait

// assert that `Stream` matches `RealStream`:
impl<I> Stream for dyn RealStream<Item = I> {
    type Item = I;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<Option<Self::Item>>
    {
        RealStream::poll_next(self, cx)
    }
}
}

mod channels {
use {
    futures::{
        channel::mpsc,
        prelude::*,
    },
};

// ANCHOR: channels
async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);

    // `StreamExt::next` ressemble à `Iterator::next`, mais retourne un type
    // qui implémente `Future<Output = Option<T>>`.
    assert_eq!(Some(1), rx.next().await);
    assert_eq!(Some(2), rx.next().await);
    assert_eq!(None, rx.next().await);
}
// ANCHOR_END: channels

#[test]
fn run_send_recv() { futures::executor::block_on(send_recv()) }
}
