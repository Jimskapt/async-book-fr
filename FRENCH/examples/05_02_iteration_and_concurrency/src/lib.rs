#![cfg(test)]

use {
    futures::{
        executor::block_on,
        stream::{self, Stream},
    },
    std::{
        io,
        pin::Pin,
    },
};

// ANCHOR: nexts
async fn somme_avec_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
    use futures::stream::StreamExt; // pour utiliser `next`
    let mut somme = 0;
    while let Some(valeur) = stream.next().await {
        somme += valeur;
    }
    somme
}

async fn somme_avec_try_next(
    mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
) -> Result<i32, io::Error> {
    use futures::stream::TryStreamExt; // pour utiliser `try_next`
    let mut somme = 0;
    while let Some(valeur) = stream.try_next().await? {
        somme += valeur;
    }
    Ok(somme)
}
// ANCHOR_END: nexts

#[test]
fn executer_somme_avec_next() {
    let mut stream = stream::iter(vec![2, 3]);
    let pin: Pin<&mut stream::Iter<_>> = Pin::new(&mut stream);
    assert_eq!(5, block_on(somme_avec_next(pin)));
}

#[test]
fn executer_sum_with_try_next() {
    let mut stream = stream::iter(vec![Ok(2), Ok(3)]);
    let pin: Pin<&mut stream::Iter<_>> = Pin::new(&mut stream);
    assert_eq!(5, block_on(somme_avec_try_next(pin)).unwrap());
}

#[allow(unused)]
// ANCHOR: try_for_each_concurrent
async fn sauter_partout(
    mut stream: Pin<&mut dyn Stream<Item = Result<u8, io::Error>>>,
) -> Result<(), io::Error> {
    use futures::stream::TryStreamExt; // pour utiliser `try_for_each_concurrent`
    const SAUTS_CONCURRENTS_MAXI: usize = 100;

    stream.try_for_each_concurrent(SAUTS_CONCURRENTS_MAXI, |nombre| async move {
        saute_x_fois(nombre).await?;
        reporter_x_sauts(nombre).await?;
        Ok(())
    }).await?;

    Ok(())
}
// ANCHOR_END: try_for_each_concurrent

async fn saute_x_fois(_: u8) -> Result<(), io::Error> { Ok(()) }
async fn reporter_x_sauts(_: u8) -> Result<(), io::Error> { Ok(()) }
