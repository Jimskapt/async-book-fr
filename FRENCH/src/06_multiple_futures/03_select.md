<!--
# `select!`
-->

# `select!`

<!--
The `futures::select` macro runs multiple futures simultaneously, allowing
the user to respond as soon as any future completes.
-->

La macro `futures::select` exécute plusieurs futures en même temps, permettant
à son utilisateur de répondre dès qu'une future est terminée.

<!--
```rust,edition2018
{{#include ../../examples-sources/06_03_select/src/lib.rs:example}}
```
-->

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:example}}
```

<!--
The function above will run both `t1` and `t2` concurrently. When either
`t1` or `t2` finishes, the corresponding handler will call `println!`, and
the function will end without completing the remaining task.
-->

La fonction ci-dessus va exécuter `t1` et `t2` en concurrence. Lorsque `t1` ou
`t2` se termine, la branche correspondante va appeler `println!` et la fonction
va se terminer sans terminer la tâche restante.

<!--
The basic syntax for `select` is `<pattern> = <expression> => <code>,`,
repeated for as many futures as you would like to `select` over.
-->

La syntaxe classique pour `select` est `<motif> = <expression> => <code>,`,
répétée par autant de futures que vous voulez gérer avec le `select`.

<!--
## `default => ...` and `complete => ...`
-->

## `default => ...` et `complete => ...`

<!--
`select` also supports `default` and `complete` branches.
-->

`select` autorise aussi l'utilisation des branches `default` et `complete`.

<!--
A `default` branch will run if none of the futures being `select`ed
over are yet complete. A `select` with a `default` branch will
therefore always return immediately, since `default` will be run
if none of the other futures are ready.
-->

La branche `default` va s'exécuter si aucune des futures dans le `select` n'est
terminée. Un `select` avec une branche `default` toutefois retourner sa valeur
immédiatement, puisque `default` sera exécuté si aucune des futures n'est
terminée.

<!--
`complete` branches can be used to handle the case where all futures
being `select`ed over have completed and will no longer make progress.
This is often handy when looping over a `select!`.
-->

La branche `complete` peut être utilisée pour gérer le cas où toutes les
futures présentes dans le `select` se sont terminées et ne vont pas plus
progresser. C'est parfois utile lorsqu'on boucle sur un `select!`.

<!--
```rust,edition2018
{{#include ../../examples-sources/06_03_select/src/lib.rs:default_and_complete}}
```
-->

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:default_and_complete}}
```

<!--
## Interaction with `Unpin` and `FusedFuture`
-->

## Utilisation avec `Unpin` et `FusedFuture`

<!--
One thing you may have noticed in the first example above is that we
had to call `.fuse()` on the futures returned by the two `async fn`s,
as well as pinning them with `pin_mut`. Both of these calls are necessary
because the futures used in `select` must implement both the `Unpin`
trait and the `FusedFuture` trait.
-->

Vous avez peut-être remarqué dans le premier exemple ci-dessus que nous avons
dû appeller `.fuse()` sur les futures retournées par les deux fonctions
asynchrones, ainsi que les épingler avec `pin_mut`. Chacun de ces appels sont
nécessaires car les futures utilisées dans `select` doivent implémenter les
traits `Unpin` et `FusedFuture`.

<!--
`Unpin` is necessary because the futures used by `select` are not
taken by value, but by mutable reference. By not taking ownership
of the future, uncompleted futures can be used again after the
call to `select`.
-->

`Unpin` est nécessaire car les futures utilisées par `select` ne sont pas des
valeurs, mais des références mutables. En évitant de prendre possession de la
future, les futures non terminées peuvent toujours être utilisées après l'appel
à `select`.

<!--
Similarly, the `FusedFuture` trait is required because `select` must
not poll a future after it has completed. `FusedFuture` is implemented
by futures which track whether or not they have completed. This makes
it possible to use `select` in a loop, only polling the futures which
still have yet to complete. This can be seen in the example above,
where `a_fut` or `b_fut` will have completed the second time through
the loop. Because the future returned by `future::ready` implements
`FusedFuture`, it's able to tell `select` not to poll it again.
-->

De la même manière, le trait `FusedFuture` est nécessaire car `select` ne doit
pas appeler une future après qu'elle soit complétée. `FusedFuture` est
implémentée par les futures qui ont besoin de savoir si oui ou non elles se
sont terminées. Cela permet d'utiliser `select` dans une boucle, pour appeler
uniquement les futures qui n'ont pas encore terminé. Nous pouvons voir cela
dans l'exemple ci-dessus, où `future_a` ou `future_b` sont terminés dans le
deuxième tour de boucle. Comme la future retournée par `future::ready`
implémente `FusedFuture`, c'est possible d'indiquer au `select` de ne pas les
appeler à nouveau.

<!--
Note that streams have a corresponding `FusedStream` trait. Streams
which implement this trait or have been wrapped using `.fuse()`
will yield `FusedFuture` futures from their
`.next()` / `.try_next()` combinators.
-->

Remarquez que les `Stream`s ont un trait `FusedStream` correspondant. Les
`Stream`s qui implémentent ce trait ou qui ont été enveloppés en utilisant
`.fuse()` vont produire des futures `FusedFutures` à partir de leurs
combinateurs `.next()` ou `try_next()`.

<!--
```rust,edition2018
{{#include ../../examples-sources/06_03_select/src/lib.rs:fused_stream}}
```
-->

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:fused_stream}}
```

<!--
## Concurrent tasks in a `select` loop with `Fuse` and `FuturesUnordered`
-->

## Des tâches concurrentes dans une boucle `select` avec `Fuse` et `FuturesUnordered`

<!--
One somewhat hard-to-discover but handy function is `Fuse::terminated()`,
which allows constructing an empty future which is already terminated,
and can later be filled in with a future that needs to be run.
-->

Une fonction difficile à aborder, mais qui est pratique, est
`Fuse::terminated()`, ce qui permet de construire une future vide qui est déjà
terminée, et qui peut être rempli plus tard avec une future qui a besoin d'être
exécutée.

<!--
This can be handy when there's a task that needs to be run during a `select`
loop but which is created inside the `select` loop itself.
-->

Cela s'avère utile lorsqu'une tâche nécessite d'être exécuté dans une boucle
`select` qui est elle-même créée dans la boucle `select`.

<!--
Note the use of the `.select_next_some()` function. This can be
used with `select` to only run the branch for `Some(_)` values
returned from the stream, ignoring `None`s.
-->

Remarquez l'utilisation de la fonction `.select_next_some()`. Elle peut être
utilisée avec `select` pour exécuter uniquement la branche pour les valeurs
`Some(_)` retournées par le `Stream`, en ignorant les `None`s.

<!--
```rust,edition2018
{{#include ../../examples-sources/06_03_select/src/lib.rs:fuse_terminated}}
```
-->

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:fuse_terminated}}
```

<!--
When many copies of the same future need to be run simultaneously,
use the `FuturesUnordered` type. The following example is similar
to the one above, but will run each copy of `run_on_new_num_fut`
to completion, rather than aborting them when a new one is created.
It will also print out a value returned by `run_on_new_num_fut`.
-->

Lorsque de nombreuses copies d'une même future a besoin d'être exécuté en même
temps, utilisez le type `FuturesUnordered`. L'exemple suivant ressemble à celui
ci-dessus, mais va exécuter chaque copie de `obtenir_nouveau_nombre_future`
jusqu'à ce qu'elles soient terminées, plutôt que de les arrêter lorsqu'une
nouvelle est générée. Cela va aussi afficher la valeur retournée par
`obtenir_nouveau_nombre_future`.

<!--
```rust,edition2018
{{#include ../../examples-sources/06_03_select/src/lib.rs:futures_unordered}}
```
-->

```rust,edition2018
{{#include ../../examples/06_03_select/src/lib.rs:futures_unordered}}
```
