> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.

<!--
# Final Project: Building a Concurrent Web Server with Async Rust
In this chapter, we'll use asynchronous Rust to modify the Rust book's 
[single-threaded web server](https://doc.rust-lang.org/book/ch20-01-single-threaded.html) 
to serve requests concurrently.
## Recap
Here's what the code looked like at the end of the lesson.
-->

# Projet final : construire un serveur web concurrent avec le Rust asynchrone

Dans ce chapitre, nous allons utiliser le Rust asynchrone pour modifier le
[serveur web mono-processus](https://jimskapt.github.io/rust-book-fr/ch20-00-final-project-a-web-server.html)
du livre sur Rust, afin qu'il serve les requêtes de manière concurrente.

## Résumé

Voici ce à quoi ressemblera le code à la fin de cette leçon.

<!--
`src/main.rs`:
```rust
{{#include ../../examples-sources/09_01_sync_tcp_server/src/main.rs}}
```
-->

`src/main.rs` :

```rust
{{#include ../../examples/09_01_sync_tcp_server/src/main.rs}}
```

<!--
`hello.html`:
```html
{{#include ../../examples-sources/09_01_sync_tcp_server/hello.html}}
```
-->

`hello.html` :

```html
{{#include ../../examples/09_01_sync_tcp_server/hello.html}}
```

<!--
`404.html`:
```html
{{#include ../../examples-sources/09_01_sync_tcp_server/404.html}}
```
-->

`404.html` :

```html
{{#include ../../examples/09_01_sync_tcp_server/404.html}}
```

<!--
If you run the server with `cargo run` and visit `127.0.0.1:7878` in your browser,
you'll be greeted with a friendly message from Ferris!
-->

Si vous exécutez le serveur avec `cargo run` et visitez `127.0.0.1:7878` dans
votre navigateur, vous allez être accueilli par un message chaleureux de
Ferris !
