<!--
# `async`/`.await`
-->

# `async` et `await`

<!--
In [the first chapter], we took a brief look at `async`/`.await`.
This chapter will discuss `async`/`.await` in
greater detail, explaining how it works and how `async` code differs from
traditional Rust programs.
-->

Dans [le premier chapitre][the first chapter], nous avons introduit `async` et
`await`. Ce nouveau chapitre va aborder plus en détails `async` et `await`, en
expliquant comment il fonctionne et comment le code `async` se distingue des
programmes Rust traditionnels.

<!--
`async`/`.await` are special pieces of Rust syntax that make it possible to
yield control of the current thread rather than blocking, allowing other
code to make progress while waiting on an operation to complete.
-->

`async` et `await` sont des entités spécifiques de la syntaxe Rust qui rend
possible de transférer le contrôle du processus en cours plutôt que de bloquer,
ce qui permet à un autre code de progresser pendant que nous attendons qu'une
opération se termine.

<!--
There are two main ways to use `async`: `async fn` and `async` blocks.
Each returns a value that implements the `Future` trait:
-->

Il y a deux principales façons d'utiliser `async` : `async fn` et les blocs
`async`. Chacun retourne une valeur qui implémente le trait `Future` :

<!--
```rust,edition2018,ignore
{{#include ../../examples-sources/03_01_async_await/src/lib.rs:async_fn_and_block_examples}}
```
-->

```rust,edition2018,ignore
{{#include ../../examples/03_01_async_await/src/lib.rs:async_fn_and_block_examples}}
```

<!--
As we saw in the first chapter, `async` bodies and other futures are lazy:
they do nothing until they are run. The most common way to run a `Future`
is to `.await` it. When `.await` is called on a `Future`, it will attempt
to run it to completion. If the `Future` is blocked, it will yield control
of the current thread. When more progress can be made, the `Future` will be picked
up by the executor and will resume running, allowing the `.await` to resolve.
-->

Comme nous l'avons vu dans le premier chapitre, les corps des `async` et des
autres futures sont passives : ils ne font rien jusqu'à ce qu'ils soient
exécutés. La façon la plus courante d'exécuter une `Future` est d'utiliser
`await` sur elle. Lorsque `await` est utilisé sur une `Future`, il va tenter de
l'exécuter jusqu'à sa fin. Si la `Future` est bloquée, il va transférer le
contrôle du processus en cours. Lorsqu'une progression pourra être effectuée,
la `Future` va être récupéré par l'exécuteur et va continuer son exécution, ce
qui permettra à terme au `await` de se résoudre.

<!--
## `async` Lifetimes
-->

## Les durées de vie `async`

<!--
Unlike traditional functions, `async fn`s which take references or other
non-`'static` arguments return a `Future` which is bounded by the lifetime of
the arguments:
-->

Contrairement au fonctions traditionnelles, les `async fn` qui utilisent des
références, ou d'autres arguments non `static`, vont retourner une `Future` qui
est contrainte par la durée de vie des arguments :

<!--
```rust,edition2018,ignore
{{#include ../../examples-sources/03_01_async_await/src/lib.rs:lifetimes_expanded}}
```
-->

```rust,edition2018,ignore
{{#include ../../examples/03_01_async_await/src/lib.rs:lifetimes_expanded}}
```

<!--
This means that the future returned from an `async fn` must be `.await`ed
while its non-`'static` arguments are still valid. In the common
case of `.await`ing the future immediately after calling the function
(as in `foo(&x).await`) this is not an issue. However, if storing the future
or sending it over to another task or thread, this may be an issue.
-->

Cela signifie que l'on doit utiliser `await` sur la future retournée d'une
`async fn` uniquement pendant que ses arguments non `static` soient toujours en
vigueur. Dans le cas courant où on utilise `await` sur la future immédiatement
après avoir appelé la fonction (comme avec `alpha(&x).await`), ce n'est pas un
problème. Cependant, si on stocke la future ou si on l'envoie à une autre tâche
ou processus, cela peut devenir un problème.

<!--
One common workaround for turning an `async fn` with references-as-arguments
into a `'static` future is to bundle the arguments with the call to the
`async fn` inside an `async` block:
-->

Un contournement courant pour utiliser une `async fn` avec des références en
argument afin qu'elle retourne une future `'static` est d'envelopper les
arguments utilisés pour l'appel à la `async fn` à l'intérieur d'un bloc
`async` :

<!--
```rust,edition2018,ignore
{{#include ../../examples-sources/03_01_async_await/src/lib.rs:static_future_with_borrow}}
```
-->

```rust,edition2018,ignore
{{#include ../../examples/03_01_async_await/src/lib.rs:static_future_with_borrow}}
```

<!--
By moving the argument into the `async` block, we extend its lifetime to match
that of the `Future` returned from the call to `good`.
-->

En déplaçant l'argument dans le bloc `async`, nous avons étendu sa durée de vie
à celle de cette `Future` qui est retournée suite à l'appel à `correct`.

<!--
## `async move`
-->

## `async move`

<!--
`async` blocks and closures allow the `move` keyword, much like normal
closures. An `async move` block will take ownership of the variables it
references, allowing it to outlive the current scope, but giving up the ability
to share those variables with other code:
-->

Les blocs et fermetures `async`autorisent l'utilisation du mot-clé `move`,
comme les fermetures synchrones. Un bloc `async move` va prendre possession
des variables qu'il utilise, leur permettant de survivre à l'extérieur de la
portée actuelle, mais par conséquent qui empêche de partager ces variables avec
un autre code :

<!--
```rust,edition2018,ignore
{{#include ../../examples-sources/03_01_async_await/src/lib.rs:async_move_examples}}
```
-->

```rust,edition2018,ignore
{{#include ../../examples/03_01_async_await/src/lib.rs:async_move_examples}}
```

<!--
## `.await`ing on a Multithreaded Executor
-->

## Utiliser `await` avec un exécuteur multi-processus

<!--
Note that, when using a multithreaded `Future` executor, a `Future` may move
between threads, so any variables used in `async` bodies must be able to travel
between threads, as any `.await` can potentially result in a switch to a new
thread.
-->

Remarquez que lorsque vous utilisez un exécuteur de `Future` multi-processus,
une `Future` peut être déplacée entre les processus, donc toutes les variables
utilisées dans les corps des `async` doivent pouvoir aussi être déplacés entre
des processus, car n'importe quel `await` peut potentiellement basculer sur un
autre processus.

<!--
This means that it is not safe to use `Rc`, `&RefCell` or any other types
that don't implement the `Send` trait, including references to types that don't
implement the `Sync` trait.
-->

Cela signifie que ce n'est sûr d'utiliser `Rc`, `&RefCell` ou tout autre type
qui n'implémente pas le trait `Send`, y compris les références à des types qui
n'implémente pas le trait `Sync`.

<!--
(Caveat: it is possible to use these types as long as they aren't in scope
during a call to `.await`.)
-->

(Remarque : il reste possible d'utiliser ces types du moment qu'ils ne sont pas
dans la portée d'un appel à `await`)

<!--
Similarly, it isn't a good idea to hold a traditional non-futures-aware lock
across an `.await`, as it can cause the threadpool to lock up: one task could
take out a lock, `.await` and yield to the executor, allowing another task to
attempt to take the lock and cause a deadlock. To avoid this, use the `Mutex`
in `futures::lock` rather than the one from `std::sync`.
-->

Pour la même raison, ce n'est pas une bonne idée de maintenir un verrou
traditionnel, qui ne se préoccupe pas des futures, dans un `await`, car cela
peut provoquer le blocage du groupe de processus : une tâche peut poser le
verrou, attendre grâce à `await` et transférer le contrôle à l'exécuteur, qui
va permettre à une autre tâche de vouloir poser le verrou et cela va causer un
interblocage. Pour éviter cela, utilisez le `Mutex` dans `futures::lock` plutôt
que celui dans `std::sync`.

<!--
[the first chapter]: ../01_getting_started/04_async_await_primer.md
-->

[the first chapter]: ../01_getting_started/04_async_await_primer.md
