<!--
# Running Asynchronous Code
An HTTP server should be able to serve multiple clients concurrently;
that is, it should not wait for previous requests to complete before handling the current request.
The book
[solves this problem](https://doc.rust-lang.org/book/ch20-02-multithreaded.html#turning-our-single-threaded-server-into-a-multithreaded-server)
by creating a thread pool where each connection is handled on its own thread.
Here, instead of improving throughput by adding threads, we'll achieve the same effect using asynchronous code.
-->

# Exécuter du code asynchrone

Un serveur HTTP doit être capable de servir plusieurs clients en concurrence,
et par conséquent, il ne doit pas attendre que les requêtes précédentes soient
terminées pour s'occuper de la requête en cours.
Le livre Rust [résout ce
problème](https://jimskapt.github.io/rust-book-fr/ch20-02-multithreaded.html#transformer-notre-serveur-monot%C3%A2che-en-serveur-multit%C3%A2ches)
en créant un groupe de tâches où chaque connexion est gérée sur son propre
processus. Nous allons obtenir le même effet en utilisant du code asynchrone,
au lieu d'améliorer le débit en ajoutant des processus.

<!--
Let's modify `handle_connection` to return a future by declaring it an `async fn`:
```rust,ignore
{{#include ../../examples-sources/09_02_async_tcp_server/src/main.rs:handle_connection_async}}
```
-->

Modifions le `gestion_connexion` pour retourner une future en la déclarant
comme étant une fonction asynchrone :

```rust,ignore
{{#include ../../examples/09_02_async_tcp_server/src/main.rs:handle_connection_async}}
```

<!--
Adding `async` to the function declaration changes its return type
from the unit type `()` to a type that implements `Future<Output=()>`.
-->

L'ajout de `async` à la déclaration de la fonction change son type de retour de
`()` à un type qui implémente `Future<Output=()>`.

<!--
If we try to compile this, the compiler warns us that it will not work:
```console
$ cargo check
    Checking async-rust v0.1.0 (file:///projects/async-rust)
warning: unused implementer of `std::future::Future` that must be used
  -- > src/main.rs:12:9
   |
12 |         handle_connection(stream);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: futures do nothing unless you `.await` or poll them
```
-->

Si nous essayons de compiler cela, le compilateur va nous avertir que cela ne
fonctionnera pas :

```console
$ cargo check
    Checking async-rust v0.1.0 (file:///projects/async-rust)
warning: unused implementer of `std::future::Future` that must be used
  -- > src/main.rs:12:9
   |
12 |         gestion_connexion(flux);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: futures do nothing unless you `.await` or poll them
```

<!--
Because we haven't `await`ed or `poll`ed the result of `handle_connection`,
it'll never run. If you run the server and visit `127.0.0.1:7878` in a browser,
you'll see that the connection is refused; our server is not handling requests.
-->

Comme nous n'avons pas utilisé `await` ou `poll` sur le résultat de
`gestion_connexion`, cela ne va jamais s'exécuter. Si vous lancez le serveur et
visitez `127.0.0.1:7878` dans un navigateur web, vous constaterez que la
connexion est refusée, notre serveur ne prend pas en charge les requêtes.

<!--
We can't `await` or `poll` futures within synchronous code by itself.
We'll need an asynchronous runtime to handle scheduling and running futures to completion.
Please consult the [section on choosing a runtime](../08_ecosystem/00_chapter.md)
for more information on asynchronous runtimes, executors, and reactors.
Any of the runtimes listed will work for this project, but for these examples,
we've chosen to use the `async-std` crate.
-->

Nous ne pouvons pas utiliser `await` ou `poll` sur des futures dans du code
synchrone tout seul.
Nous allons avoir besoin d'un environnement d'exécution asynchrone pour gérer
la planification et l'exécution des futures jusqu'à ce qu'elles se terminent.
Vous pouvez consulter la [section pour choisir un environnement
d'exécution](../08_ecosystem/00_chapter.md) pour plus d'information sur les
environnements d'exécution, exécuteurs et réacteurs asynchrones.
Tous les environnements d'exécution listés vont fonctionner pour ce projet,
mais pour ces exemples, nous avons choisi d'utiliser la crate `async-std`.

<!--
## Adding an Async Runtime
The following example will demonstrate refactoring synchronous code to use an async runtime; here, `async-std`.
The `#[async_std::main]` attribute from `async-std` allows us to write an asynchronous main function.
To use it, enable the `attributes` feature of `async-std` in `Cargo.toml`:
```toml
[dependencies.async-std]
version = "1.6"
features = ["attributes"]
```
-->

## Ajouter un environnement d'exécution asynchrone

L'exemple suivant va monter le remaniement du code synchrone pour utiliser un
environnement d'exécution asynchrone, dans ce cas `async-std`.
L'attribut `#[async_std::main]` de `async-std` nous permet d'écrire une
fonction `main` asynchrone.
Pour l'utiliser, il faut activer la fonctionnalité `attributes` de `async-std`
dans `Cargo.toml` :

```toml
[dependencies.async-std]
version = "1.6"
features = ["attributes"]
```

<!--
As a first step, we'll switch to an asynchronous main function,
and `await` the future returned by the async version of `handle_connection`.
Then, we'll test how the server responds.
Here's what that would look like:
```rust
{{#include ../../examples-sources/09_02_async_tcp_server/src/main.rs:main_func}}
```
Now, let's test to see if our server can handle connections concurrently.
Simply making `handle_connection` asynchronous doesn't mean that the server
can handle multiple connections at the same time, and we'll soon see why.
-->

Pour commencer, nous allons changer pour une fonction `main` asynchrone, et
utiliser `await` sur la future retournée par la version asynchrone de
`gestion_connexion`. Ensuite, nous testerons comment le serveur répond. Voici à
quoi cela ressemblerait :

```rust
{{#include ../../examples/09_02_async_tcp_server/src/main.rs:main_func}}
```

Maintenant, testons pour voir si notre serveur peut gérer les connexions en
concurrence. Transformer simplement `gestion_connexion` en asynchrone ne
signifie pas que le serveur puisse gérer plusieurs connexions en même temps, et
nous allons bientôt voir pourquoi.

<!--
To illustrate this, let's simulate a slow request.
When a client makes a request to `127.0.0.1:7878/sleep`,
our server will sleep for 5 seconds:
-->

Pour illustrer cela, simulons une réponse lente.
Lorsqu'un client fait une requête vers `127.0.0.1:7878/pause`, notre serveur va
attendre 5 secondes :

<!--
```rust,ignore
{{#include ../../examples-sources/09_03_slow_request/src/main.rs:handle_connection}}
```
This is very similar to the 
[simulation of a slow request](https://doc.rust-lang.org/book/ch20-02-multithreaded.html#simulating-a-slow-request-in-the-current-server-implementation)
from the Book, but with one important difference:
we're using the non-blocking function `async_std::task::sleep` instead of the blocking function `std::thread::sleep`.
It's important to remember that even if a piece of code is run within an `async fn` and `await`ed, it may still block.
To test whether our server handles connections concurrently, we'll need to ensure that `handle_connection` is non-blocking.
-->

```rust,ignore
{{#include ../../examples/09_03_slow_request/src/main.rs:handle_connection}}
```

C'est très ressemblant à la [simulation d'une requête
lente](https://jimskapt.github.io/rust-book-fr/ch20-02-multithreaded.html#simuler-une-longue-requ%C3%AAte-%C3%A0-traiter-avec-limpl%C3%A9mentation-actuelle-du-serveur)
dans le livre Rust, mais avec une différence importante :
nous utilisons la fonction non bloquante `async_std::task::sleep` au lieu de la
fonction bloquante `std::thread::sleep`.
Il est important de se rappeler que même si un code est exécuté dans une
fonction asynchrone et qu'on utilise `await` sur elle, elle peut toujours
bloquer.
Pour tester si notre serveur puisse gérer les connexions en concurrence, nous
avons besoin de nous assurer que `gestion_connexion` n'est pas bloquante.

<!--
If you run the server, you'll see that a request to `127.0.0.1:7878/sleep`
will block any other incoming requests for 5 seconds!
This is because there are no other concurrent tasks that can make progress
while we are `await`ing the result of `handle_connection`.
In the next section, we'll see how to use async code to handle connections concurrently.
-->

Si vous exécutez le serveur, vous constaterez qu'une requête vers
`127.0.0.1:7878/pause` devrait bloquer toutes les autres requêtes entrantes
pendant 5 secondes !
C'est parce qu'il n'y a pas d'autres tâches concurrentes qui peuvent progresser
pendant que nous utilisons `await` sur le résultat de `gestion_connexion`.
Dans la prochaine section, nous allons voir comment utiliser du code asynchrone
pour gérer en concurrence les connexions.
