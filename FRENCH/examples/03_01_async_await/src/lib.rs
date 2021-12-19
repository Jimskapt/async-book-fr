#![allow(unused)]
#![cfg(test)]

mod async_fn_and_block_examples {
use std::future::Future;
// ANCHOR: async_fn_and_block_examples

// `alpha()` retourne un type qui implémente `Future<Output = u8>`.
// `alpha().await` va retourner une valeur de type `u8`.
async fn alpha() -> u8 { 5 }

fn beta() -> impl Future<Output = u8> {
    // Ce bloc `async` va retourner un type qui implémente
    // `Future<Output = u8>`.
    async {
        let x: u8 = alpha().await;
        x + 5
    }
}
// ANCHOR_END: async_fn_and_block_examples
}

mod async_lifetimes_examples {
use std::future::Future;
// ANCHOR: lifetimes_expanded
// Cette fonction :
async fn alpha(x: &u8) -> u8 { *x }

// ... est équivalente à cette fonction :
fn alpha_enrichi<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move { *x }
}
// ANCHOR_END: lifetimes_expanded

async fn emprunter_x(x: &u8) -> u8 { *x }

#[cfg(feature = "never_compiled")]
// ANCHOR: static_future_with_borrow
fn incorrect() -> impl Future<Output = u8> {
    let x = 5;
    emprunter_x(&x) // ERREUR : `x` ne vit pas suffisamment longtemps
}

fn correct() -> impl Future<Output = u8> {
    async {
        let x = 5;
        emprunter_x(&x).await
    }
}
// ANCHOR_END: static_future_with_borrow
}

mod async_move_examples {
use std::future::Future;
// ANCHOR: async_move_examples
/// blocs `async` :
///
/// Plusieurs blocs `async` différents peuvent accéder à la même variable
/// locale tant qu'elles sont exécutées dans la portée de la variable
async fn blocs() {
    let ma_chaine = "alpha".to_string();

    let premiere_future = async {
        // ...
        println!("{}", ma_chaine);
    };

    let seconde_future = async {
        // ...
        println!("{}", ma_chaine);
    };

    // Exécute les deux futures jusqu'à leur fin, ce qui affichera
    // deux fois "alpha" :
    let ((), ()) = futures::join!(premiere_future, seconde_future);
}

/// blocs `async move` :
///
/// Un seul bloc `async move` peut avoir accès à la même variable capturée,
/// puisque qu'elles sont déplacées dans la `Future` générée par le bloc
/// `async move`.
/// Cependant, cela permet d'étendre la portée de la `Future` en dehors de
/// celle de la variable :
fn bloc_avec_move() -> impl Future<Output = ()> {
    let ma_chaine = "alpha".to_string();
    async move {
        // ...
        println!("{}", ma_chaine);
    }
}
// ANCHOR_END: async_move_examples
}
