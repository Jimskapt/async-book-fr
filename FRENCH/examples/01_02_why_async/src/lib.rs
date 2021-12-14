#![cfg(test)]

fn telecharger(_url: &str) {
    // ...
}

#[test]
// ANCHOR: get_two_sites
fn recuperer_deux_sites() {
    // Crée deux tâches pour faire le travail.
    let premiere_tache = std::thread::spawn(|| telecharger("https://www.foo.com"));
    let seconde_tache = std::thread::spawn(|| telecharger("https://www.bar.com"));

    // Attente que les deux tâches se terminent.
    premiere_tache.join().expect("la première tâche a paniqué");
    seconde_tache.join().expect("la deuxième tâche a paniqué");
}
// ANCHOR_END: get_two_sites

async fn telecharger_asynchrone(_url: &str) {
    // ...
}

// ANCHOR: get_two_sites_async
async fn recuperer_deux_sites_asynchrone() {
    // Crée deux différentes "futures" qui, lorsqu'elles sont menée à terme,
    // va télécharger les pages web de manière asynchrone.
    let premier_future = telecharger_asynchrone("https://www.foo.com");
    let second_future = telecharger_asynchrone("https://www.bar.com");

    // Exécute les deux futures en même temps jusqu'à leur fin.
    futures::join!(premier_future, second_future);
}
// ANCHOR_END: get_two_sites_async

#[test]
fn recuperer_deux_sites_asynchrone_test() {
    futures::executor::block_on(recuperer_deux_sites_asynchrone());
}
