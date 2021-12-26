#![cfg(test)]

use futures::executor::block_on;

mod first {
// ANCHOR: hello_world
// `block_on` bloque le processus en cours jusqu'à ce que la future qu'on lui
// donne ait terminé son exécution. Les autres exécuteurs ont un comportement
// plus complexe, comme par exemple ordonnancer plusieurs futures sur le même
// processus.
use futures::executor::block_on;

async fn salutations() {
    println!("salutations !");
}

fn main() {
    let future = salutations(); // rien n'est pas affiché
    block_on(future); // `future` est exécuté et "salutations !" est affiché
}
// ANCHOR_END: hello_world

#[test]
fn run_main() { main() }
}

struct Chanson;
async fn apprendre_chanson() -> Chanson { Chanson }
async fn chanter_chanson(_: Chanson) {}
async fn danser() {}

mod second {
use super::*;
// ANCHOR: block_on_each
fn main() {
    let chanson = block_on(apprendre_chanson());
    block_on(chanter_chanson(chanson));
    block_on(danser());
}
// ANCHOR_END: block_on_each

#[test]
fn run_main() { main() }
}

mod third {
use super::*;
// ANCHOR: block_on_main
async fn apprendre_et_chanter() {
    // Attends (await) que la chanson soit apprise avant de la chanter.
    // Nous utilisons ici `.await` plutôt que `block_on` pour éviter de bloquer
    // le processus, ce qui rend possible de `danser` en même temps.
    let chanson = apprendre_chanson().await;
    chanter_chanson(chanson).await;
}

async fn async_main() {
    let f1 = apprendre_et_chanter();
    let f2 = danser();

    // `join!` se comporte comme `.await` mais ne peut pas attendre plusieurs
    // futures en concurrence. Si nous avions bloqué temporairement dans la
    // future `apprendre_et_chanter`, la future `danser` continuera son
    // exécution dans le processus en cours. Si `danser` se bloque aussi,
    // `apprendre_et_chanter` pourra continuer dans le processus en cours. Si
    // les deux futures sont bloquées, et bien `async_main` est bloqué et va en
    // informer son exécuteur.
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
// ANCHOR_END: block_on_main

#[test]
fn run_main() { main() }
}
