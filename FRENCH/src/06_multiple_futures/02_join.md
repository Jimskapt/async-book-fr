<!--
# `join!`
-->

# `join!`

<!--
The `futures::join` macro makes it possible to wait for multiple different
futures to complete while executing them all concurrently.
-->

La macro `futures::join` permet d'attendre que plusieurs futures différentes se
terminent pendant qu'elles sont toutes exécutées en concurrence.

<!--
# `join!`
-->

## `join!`

<!--
When performing multiple asynchronous operations, it's tempting to simply
`.await` them in a series:
-->

Lorsque nous avons besoin de faire plusieurs opérations asynchrones, il peut
être tentant d'utiliser `.await` en série sur elles :

<!--
```rust,edition2018,ignore
{{#include ../../examples-sources/06_02_join/src/lib.rs:naiive}}
```
-->

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:naiive}}
```

<!--
However, this will be slower than necessary, since it won't start trying to
`get_music` until after `get_book` has completed. In some other languages,
futures are ambiently run to completion, so two operations can be
run concurrently by first calling each `async fn` to start the futures, and
then awaiting them both:
-->

En revanche, cela peut être plus lent que nécessaire, puisqu'il ne commence
qu'à `obtenir_musique` avant que `obtenir_livre` soit terminé. Dans d'autres
langages, les futures sont exécutées normalement jusqu'à leur fin, donc deux
opérations peuvent être exécutées en concurrence en appelant chacune des
`async fn` pour démarrer les futures, et ensuite attendre la fin des deux :

<!--
```rust,edition2018,ignore
{{#include ../../examples-sources/06_02_join/src/lib.rs:other_langs}}
```
-->

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:other_langs}}
```

<!--
However, Rust futures won't do any work until they're actively `.await`ed.
This means that the two code snippets above will both run
`book_future` and `music_future` in series rather than running them
concurrently. To correctly run the two futures concurrently, use
`futures::join!`:
-->

Malheureusement, les futures en Rust ne font rien tant qu'on n'utilise pas
`.await` sur elles. Cela signifie que les deux extraits de code ci-dessus vont
exécuter `future_livre` et `future_musique` en série au lieu de les exécuter en
concurrence. Pour exécuter correctement les deux futures en concurrence,
utilisons `futures::join!` :

<!--
```rust,edition2018,ignore
{{#include ../../examples-sources/06_02_join/src/lib.rs:join}}
```
-->

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:join}}
```

<!--
The value returned by `join!` is a tuple containing the output of each
`Future` passed in.
-->

La valeur retournée par `join!` est une tuple contenant le résultat de chacune
des `Future`s qu'on lui a donné.

<!--
## `try_join!`
-->

## `try_join!`

<!--
For futures which return `Result`, consider using `try_join!` rather than
`join!`. Since `join!` only completes once all subfutures have completed,
it'll continue processing other futures even after one of its subfutures
has returned an `Err`.
-->

Pour les futures qui retournent `Result`, il vaut mieux utiliser `try_join!`
plutôt que `join!`. Comme `join!` se termine uniquement lorsque toutes les
sous-futures se soient terminées, il va continuer à calculer les autres futures
même si une de ses sous-futures a retourné une `Err`.

<!--
Unlike `join!`, `try_join!` will complete immediately if one of the subfutures
returns an error.
-->

Contrairement à `join!`, `try_join!` va se terminer tout de suite si une des
sous-futures retourne une erreur.

<!--
```rust,edition2018,ignore
{{#include ../../examples-sources/06_02_join/src/lib.rs:try_join}}
```
-->

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:try_join}}
```

<!--
Note that the futures passed to `try_join!` must all have the same error type.
Consider using the `.map_err(|e| ...)` and `.err_into()` functions from
`futures::future::TryFutureExt` to consolidate the error types:
-->

Notez que les futures envoyées au `try_join!` doivent toutes avoir le même type
d'erreur. Vous pouvez utiliser les fonctions `.map_err(|e| ...)` et
`.err_into()` de `futures::future::TryFutureExt` pour regrouper les types
d'erreurs :

<!--
```rust,edition2018,ignore
{{#include ../../examples-sources/06_02_join/src/lib.rs:try_join_map_err}}
```
-->

```rust,edition2018,ignore
{{#include ../../examples/06_02_join/src/lib.rs:try_join_map_err}}
```
