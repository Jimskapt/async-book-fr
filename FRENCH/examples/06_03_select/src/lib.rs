#![cfg(test)]
#![recursion_limit = "128"]

mod example {
    // ANCHOR: example
    use futures::{
        future::FutureExt, // pour utiliser `.fuse()`
        pin_mut,
        select,
    };

    async fn premiere_tache() { /* ... */ }
    async fn seconde_tache() { /* ... */ }

    async fn course_de_taches() {
        let t1 = premiere_tache().fuse();
        let t2 = seconde_tache().fuse();

        pin_mut!(t1, t2);

        select! {
            () = t1 => println!("la première tâche s'est terminée en premier"),
            () = t2 => println!("la seconde tâche s'est terminée en premier"),
        }
    }
    // ANCHOR_END: example
}

mod default_and_complete {
    // ANCHOR: default_and_complete
    use futures::{future, select};

    async fn compter() {
        let mut future_a = future::ready(4);
        let mut future_b = future::ready(6);
        let mut total = 0;

        loop {
            select! {
                a = future_a => total += a,
                b = future_b => total += b,
                complete => break,
                default => unreachable!(), // ne sera jamais exécuté (les futures
                                           // sont prêtes, puis ensuite terminées)
            };
        }
        assert_eq!(total, 10);
    }
    // ANCHOR_END: default_and_complete

    #[test]
    fn executer_compter() {
        futures::executor::block_on(compter());
    }
}

mod fused_stream {
    // ANCHOR: fused_stream
    use futures::{
        select,
        stream::{FusedStream, Stream, StreamExt},
    };

    async fn ajouter_deux_streams(
        mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
        mut s2: impl Stream<Item = u8> + FusedStream + Unpin,
    ) -> u8 {
        let mut total = 0;

        loop {
            let element = select! {
                x = s1.next() => x,
                x = s2.next() => x,
                complete => break,
            };
            if let Some(nombre_suivant) = element {
                total += nombre_suivant;
            }
        }

        total
    }
    // ANCHOR_END: fused_stream
}

mod fuse_terminated {
    // ANCHOR: fuse_terminated
    use futures::{
        future::{Fuse, FusedFuture, FutureExt},
        pin_mut, select,
        stream::{FusedStream, Stream, StreamExt},
    };

    async fn obtenir_nouveau_nombre() -> u8 { /* ... */ 5 }

    async fn executer_avec_nouveau_nombre(_: u8) { /* ... */ }

    async fn executer_boucle(
        mut temporisation: impl Stream<Item = ()> + FusedStream + Unpin,
        nombre_initial: u8,
    ) {
        let executer_avec_nouveau_nombre_future =
            executer_avec_nouveau_nombre(nombre_initial).fuse();
        let obtenir_nouveau_nombre_future = Fuse::terminated();
        pin_mut!(
            executer_avec_nouveau_nombre_future,
            obtenir_nouveau_nombre_future
        );
        loop {
            select! {
                () = temporisation.select_next_some() => {
                    // La temporisation s'est terminée. Démarre un nouveau
                    // `obtenir_nouveau_nombre_future` s'il n'y en a pas un qui est
                    // déjà en cours d'exécution.
                    if obtenir_nouveau_nombre_future.is_terminated() {
                        obtenir_nouveau_nombre_future.set(obtenir_nouveau_nombre().fuse());
                    }
                },
                new_num = obtenir_nouveau_nombre_future => {
                    // Un nouveau nombre est arrivé : cela démarrera un nouveau
                    // `executer_avec_nouveau_nombre_future`, ce qui libèrera
                    // l'ancien.
                    executer_avec_nouveau_nombre_future.set(executer_avec_nouveau_nombre(new_num).fuse());
                },
                // Execute le `executer_avec_nouveau_nombre_future`
                () = executer_avec_nouveau_nombre_future => {},
                // panique si tout est terminé, car la `temporisation` est censé
                // générer des valeurs à l'infini.
                complete => panic!("`temporisation` s'est terminé inopinément"),
            }
        }
    }
    // ANCHOR_END: fuse_terminated
}

mod futures_unordered {
    // ANCHOR: futures_unordered
    use futures::{
        future::{Fuse, FusedFuture, FutureExt},
        pin_mut, select,
        stream::{FusedStream, FuturesUnordered, Stream, StreamExt},
    };

    async fn obtenir_nouveau_nombre() -> u8 { /* ... */ 5 }

    async fn executer_avec_nouveau_nombre(_: u8) -> u8 { /* ... */ 5 }

    // Exécute `executer_avec_nouveau_nombre` avec le dernier nombre obtenu
    // auprès de `obtenir_nouveau_nombre`.
    //
    // `obtenir_nouveau_nombre` est exécuté à nouveau à chaque fois que la
    // temporisation se termine, ce qui annule immédiatement le
    // `executer_avec_nouveau_nombre` en cours et la remplace avec la nouvelle
    // valeur retournée.
    async fn executer_boucle(
        mut temporisation: impl Stream<Item = ()> + FusedStream + Unpin,
        nombre_initial: u8,
    ) {
        let mut executer_avec_nouveau_nombre_futures = FuturesUnordered::new();
        executer_avec_nouveau_nombre_futures.push(executer_avec_nouveau_nombre(nombre_initial));
        let obtenir_nouveau_nombre_future = Fuse::terminated();
        pin_mut!(obtenir_nouveau_nombre_future);
        loop {
            select! {
                () = temporisation.select_next_some() => {
                    // La temporisation s'est terminée. Démarre un nouveau
                    // `obtenir_nouveau_nombre_future` s'il n'y en a pas un qui est
                    // déjà en cours d'exécution.
                    if obtenir_nouveau_nombre_future.is_terminated() {
                        obtenir_nouveau_nombre_future.set(obtenir_nouveau_nombre().fuse());
                    }
                },
                new_num = obtenir_nouveau_nombre_future => {
                    // Un nouveau nombre est arrivé : cela démarrera un nouveau
                    // `executer_avec_nouveau_nombre_future`..
                    executer_avec_nouveau_nombre_futures.push(executer_avec_nouveau_nombre(new_num));
                },
                // Exécute le `executer_avec_nouveau_nombre_futures` et vérifie si certaines ont terminé.
                res = executer_avec_nouveau_nombre_futures.select_next_some() => {
                    println!("executer_avec_nouveau_nombre_future a retourné {:?}", res);
                },
                // panique si tout est terminé, car la `temporisation` est censé
                // générer des valeurs à l'infini.
                complete => panic!("`temporisation` s'est terminé inopinément"),
            }
        }
    }

    // ANCHOR_END: futures_unordered
}
