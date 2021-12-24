#![cfg(test)]

struct Livre;
struct Musique;
async fn obtenir_livre() -> Livre { Livre }
async fn obtenir_musique() -> Musique { Musique }

mod naiive {
use super::*;
// ANCHOR: naiive
async fn obtenir_livre_et_musique() -> (Livre, Musique) {
    let livre = obtenir_livre().await;
    let musique = obtenir_musique().await;
    (livre, musique)
}
// ANCHOR_END: naiive
}

mod other_langs {
use super::*;
// ANCHOR: other_langs
// MAUVAISE FAÇON -- ne faites pas cela
async fn obtenir_livre_et_musique() -> (Livre, Musique) {
    let future_livre = obtenir_livre();
    let future_musique = obtenir_musique();
    (future_livre.await, future_musique.await)
}
// ANCHOR_END: other_langs
}

mod join {
use super::*;
// ANCHOR: join
use futures::join;

async fn obtenir_livre_et_musique() -> (Livre, Musique) {
    let future_livre = obtenir_livre();
    let future_musique = obtenir_musique();
    join!(future_livre, future_musique)
}
// ANCHOR_END: join
}

mod try_join {
use super::{Livre, Musique};
// ANCHOR: try_join
use futures::try_join;

async fn obtenir_livre() -> Result<Livre, String> { /* ... */ Ok(Livre) }
async fn obtenir_musique() -> Result<Musique, String> { /* ... */ Ok(Musique) }

async fn obtenir_livre_et_musique() -> Result<(Livre, Musique), String> {
    let future_livre = obtenir_livre();
    let future_musique = obtenir_musique();
    try_join!(future_livre, future_musique)
}
// ANCHOR_END: try_join
}

mod mismatched_err {
use super::{Livre, Musique};
// ANCHOR: try_join_map_err
use futures::{
    future::TryFutureExt,
    try_join,
};

async fn obtenir_livre() -> Result<Livre, ()> { /* ... */ Ok(Livre) }
async fn obtenir_musique() -> Result<Musique, String> { /* ... */ Ok(Musique) }

async fn obtenir_livre_et_musique() -> Result<(Livre, Musique), String> {
    let future_livre = obtenir_livre().map_err(|()| "Impossible d'obtenir le livre".to_string());
    let future_musique = obtenir_musique();
    try_join!(future_livre, future_musique)
}
// ANCHOR_END: try_join_map_err
}
