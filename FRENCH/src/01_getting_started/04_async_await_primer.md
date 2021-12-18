<!--
# `async`/`.await` Primer
-->

# Introduction à `async` et `await`

<!--
`async`/`.await` is Rust's built-in tool for writing asynchronous functions
that look like synchronous code. `async` transforms a block of code into a
state machine that implements a trait called `Future`. Whereas calling a
blocking function in a synchronous method would block the whole thread,
blocked `Future`s will yield control of the thread, allowing other
`Future`s to run.
-->

Le `async` et `await` sont les outils intégrés dans Rust pour écrire des
fonctions asynchrones qui ressemblent à du code synchrone. `async` transforme
un bloc de code en une machine à états qui implémente le trait `Future`. Alors
que l'appel à une fonction bloquante dans une méthode synchrone va bloquer tout
le processus, les `Future`s bloquées céderont le contrôle du processus,
permettant aux autres `Future`s de s'exécuter.

<!--
Let's add some dependencies to the `Cargo.toml` file:
-->

Ajoutons quelques dépendances au fichier `Cargo.toml` :

<!--
```toml
{{#include ../../examples-sources/01_04_async_await_primer/Cargo.toml:9:10}}
```
-->

```toml
{{#include ../../examples/01_04_async_await_primer/Cargo.toml:9:10}}
```

<!--
To create an asynchronous function, you can use the `async fn` syntax:
-->

Pour créer une fonction asynchrone, vous pouvez utiliser la syntaxe
`async fn` :

<!--
```rust,edition2018
async fn do_something() { /* ... */ }
```
-->

```rust,edition2018
async fn faire_quelquechose() { /* ... */ }
```

<!--
The value returned by `async fn` is a `Future`. For anything to happen,
the `Future` needs to be run on an executor.
-->

La valeur retournée par `async fn` est une `Future`. Pour que quelque chose se
produise, la `Future` a besoin d'être exécutée avec un exécuteur.

<!--
```rust,edition2018
{{#include ../../examples-sources/01_04_async_await_primer/src/lib.rs:hello_world}}
```
-->

```rust,edition2018
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:hello_world}}
```

<!--
Inside an `async fn`, you can use `.await` to wait for the completion of
another type that implements the `Future` trait, such as the output of
another `async fn`. Unlike `block_on`, `.await` doesn't block the current
thread, but instead asynchronously waits for the future to complete, allowing
other tasks to run if the future is currently unable to make progress.
-->

Dans une `async fn`, vous pouvez utiliser `.await` pour attendre la fin d'un
autre type qui implémente le trait `Future`, comme le résultat d'une autre
`async fn`. Contrairement à `block_on`, `.await` ne bloque pas le processus en
cours, mais attends plutôt de manière asynchrone que la future se termine, pour
permettre aux autres tâches de s'exécuter si la future n'est pas en mesure de
progresser actuellement.

<!--
For example, imagine that we have three `async fn`: `learn_song`, `sing_song`,
and `dance`:
-->

Par exemple, imaginons que nous ayons trois `async fn` : `apprendre_chanson`,
`chanter_chanson`, et `danser` :

<!--
```rust,ignore
async fn learn_song() -> Song { /* ... */ }
async fn sing_song(song: Song) { /* ... */ }
async fn dance() { /* ... */ }
```
-->

```rust,ignore
async fn apprendre_chanson() -> Chanson { /* ... */ }
async fn chanter_chanson(chanson: Chanson) { /* ... */ }
async fn danser() { /* ... */ }
```

<!--
One way to do learn, sing, and dance would be to block on each of these
individually:
-->

Une façon d'apprendre, chanter, et danser serait de bloquer sur chacun :

<!--
```rust,ignore
{{#include ../../examples-sources/01_04_async_await_primer/src/lib.rs:block_on_each}}
```
-->

```rust,ignore
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:block_on_each}}
```

<!--
However, we're not giving the best performance possible this way—we're
only ever doing one thing at once! Clearly we have to learn the song before
we can sing it, but it's possible to dance at the same time as learning and
singing the song. To do this, we can create two separate `async fn` which
can be run concurrently:
-->

Cependant, nous ne profitons pas de performances optimales de cette manière —
nous ne faisons qu'une seule chose à fois ! Il faut que nous apprenions la
chanson avant de pouvoir la chanter, mais il reste possible de danser en même
temps qu'on apprends et qu'on chante la chanson. Pour pouvoir faire cela, nous
pouvons créer deux `async fn` qui peuvent être exécutés en concurrence :

<!--
```rust,ignore
{{#include ../../examples-sources/01_04_async_await_primer/src/lib.rs:block_on_main}}
```
-->

```rust,ignore
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:block_on_main}}
```

<!--
In this example, learning the song must happen before singing the song, but
both learning and singing can happen at the same time as dancing. If we used
`block_on(learn_song())` rather than `learn_song().await` in `learn_and_sing`,
the thread wouldn't be able to do anything else while `learn_song` was running.
This would make it impossible to dance at the same time. By `.await`-ing
the `learn_song` future, we allow other tasks to take over the current thread
if `learn_song` is blocked. This makes it possible to run multiple futures
to completion concurrently on the same thread.
-->

Dans cet exemple, l'apprentissage de la chanson doit être effectué avant de
chanter la chanson, mais l'apprentissage et le chant peuvent se dérouler en
même temps qu'on danse. Si nous avions utilisé `block_on(apprendre_chanson())`
plutôt que `apprendre_chanson().await` dans `apprendre_et_chanter`, le
processus n'aurait rien pu faire tant que `apprendre_chanson` s'exécutait. Cela
aurait rendu impossible de danser en même temps. En attendant la future
`apprendre_chanson`, grâce à `await`, nous permettons aux autres tâches de
reprendre la main sur le processus en cours lorsque `apprendre_chanson` est
bloqué. Cela permet d'exécuter plusieurs futures jusqu'à leur fin de manière
concurrente sur le même processus.
