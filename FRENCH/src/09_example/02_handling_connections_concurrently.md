<!--
# Handling Connections Concurrently
The problem with our code so far is that `listener.incoming()` is a blocking iterator.
The executor can't run other futures while `listener` waits on incoming connections,
and we can't handle a new connection until we're done with the previous one.
-->

# Gérer les connexions en concurrence

Le problème avec notre code précédent est que `ecouteur.incoming()` est un
itérateur bloquant. L'exécuteur ne peut pas exécuter d'autres futures pendant
que `ecouteur` attends les connexions entrantes, et nous ne pouvons pas gérer
une nouvelle connexion jusqu'à ce que nous ayons terminé avec la précédente.

<!--
In order to fix this, we'll transform `listener.incoming()` from a blocking Iterator
to a non-blocking Stream. Streams are similar to Iterators, but can be consumed asynchronously.
For more information, see the [chapter on Streams](../05_streams/01_chapter.md).
-->

Pour corriger cela, nous allons transformer l'itérateur bloquant
`ecouteur.incoming()` en `Stream` non bloquant. Les `Stream`s ressemblent aux
itérateurs, mais peuvent être consommés de manière asynchrone. Pour plus
d'informations, vous pouvez consulter [le chapitre sur les
`Stream`s](../05_streams/01_chapter.md).

<!--
Let's replace our blocking `std::net::TcpListener` with the non-blocking `async_std::net::TcpListener`,
and update our connection handler to accept an `async_std::net::TcpStream`:
```rust,ignore
{{#include ../../examples-sources/09_04_concurrent_tcp_server/src/main.rs:handle_connection}}
```
-->

Remplaçons notre `std::net::TcpListener` bloquant par le
`async_std::net::TcpListener` non bloquant, et mettons à jour notre gestion de
connexion pour accepter un `async_std::net::TcpStream` :

```rust,ignore
{{#include ../../examples/09_04_concurrent_tcp_server/src/main.rs:handle_connection}}
```

<!--
The asynchronous version of `TcpListener` implements the `Stream` trait for `listener.incoming()`,
a change which provides two benefits.
The first is that `listener.incoming()` no longer blocks the executor.
The executor can now yield to other pending futures 
while there are no incoming TCP connections to be processed.
-->

La version asynchrone de `TcpListener` implémente le trait `Stream` sur
`ecouteur.incoming()`, ce qui apporte deux avantages.
Le premier est que `ecouteur.incoming()` ne bloque plus l'exécuteur.
L'exécuteur peut maintenant transférer l'exécution à d'autres futures en
attente lorsqu'il n'y a plus de connexions TCP entrantes à traiter.

<!--
The second benefit is that elements from the Stream can optionally be processed concurrently,
using a Stream's `for_each_concurrent` method.
Here, we'll take advantage of this method to handle each incoming request concurrently.
We'll need to import the `Stream` trait from the `futures` crate, so our Cargo.toml now looks like this:
```diff
+[dependencies]
+futures = "0.3"

 [dependencies.async-std]
 version = "1.6"
 features = ["attributes"]
```
-->

Le second bienfait est que les éléments du `Stream` peuvent optionnellement
être traités en concurrence, en utilisant la méthode `for_each_concurrent` des
`Stream`s.
Ici, nous allons profiter de cette méthode pour traiter chaque requête entrante
de manière concurrente.
Nous avons besoin d'importer le trait `Stream` de la crate `futures`, donc
notre Cargo.toml ressemble maintenant à ceci :

```diff
+[dependencies]
+futures = "0.3"

 [dependencies.async-std]
 version = "1.6"
 features = ["attributes"]
```

<!--
Now, we can handle each connection concurrently by passing `handle_connection` in through a closure function.
The closure function takes ownership of each `TcpStream`, and is run as soon as a new `TcpStream` becomes available.
As long as `handle_connection` does not block, a slow request will no longer prevent other requests from completing.
```rust,ignore
{{#include ../../examples-sources/09_04_concurrent_tcp_server/src/main.rs:main_func}}
```
# Serving Requests in Parallel
Our example so far has largely presented concurrency (using async code)
as an alternative to parallelism (using threads).
However, async code and threads are not mutually exclusive.
In our example, `for_each_concurrent` processes each connection concurrently, but on the same thread.
The `async-std` crate allows us to spawn tasks onto separate threads as well.
Because `handle_connection` is both `Send` and non-blocking, it's safe to use with `async_std::task::spawn`.
Here's what that would look like:
```rust
{{#include ../../examples-sources/09_05_final_tcp_server/src/main.rs:main_func}}
```
Now we are using both concurrency and parallelism to handle multiple requests at the same time!
See the [section on multithreaded executors](../08_ecosystem/00_chapter.md#single-threading-vs-multithreading)
for more information.
-->

Maintenant, nous pouvons traiter chaque connexion en concurrence en passant
`gestion_connexion` dans une fermeture. La fermeture prend possession de chaque
`TcpStream`, et est exécuté dès qu'un nouveau `TcpStream` est disponible. Tant
que `gestion_connexion` ne bloque pas, une réponse lente ne va plus empêcher
les autres requêtes de se compléter.

```rust,ignore
{{#include ../../examples/09_04_concurrent_tcp_server/src/main.rs:main_func}}
```

# Servir les requêtes en parallèle

Notre exemple jusqu'à présent a largement présenté la concurrence (en utilisant
du code asynchrone) comme étant une alternative au parallélisme (en utilisant
des processus).
Cependant, le code asynchrone et les processus ne s'excluent pas mutuellement.
Dans notre exemple, `for_each_concurrent` traite chaque connexion en
concurrence, mais sur le même processus.
La crate `async-std` nous permet également de créer des tâches sur des
processus séparés.
Comme `gestion_connexion` est à la fois `Send` et non bloquant, il est sûr à
utiliser avec `async_std::task::spawn`.
Voici à quoi cela devrait ressembler :

```rust
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:main_func}}
```

Maintenant nous utilisons à la fois la concurrence et le parallélisme pour
traiter plusieurs requêtes en même temps ! Lisez la [section sur les exécuteurs
multi-processus](../08_ecosystem/00_chapter.md) pour en savoir plus.
