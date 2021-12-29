<!--
# Testing the TCP Server
Let's move on to testing our `handle_connection` function.
-->

# Test du serveur TCP

Passons désormais au test de notre fonction `gestion_connexion`.

<!--
First, we need a `TcpStream` to work with.
In an end-to-end or integration test, we might want to make a real TCP connection
to test our code.
One strategy for doing this is to start a listener on `localhost` port 0.
Port 0 isn't a valid UNIX port, but it'll work for testing.
The operating system will pick an open TCP port for us.
-->

D'abord, nous avons besoin d'un `TcpStream` avec lequel travailler.
Dans un test d'intégration ou de bout-à-bout, nous serions tentés de faire une
vraie connexion TCP pour tester notre code.
Une des stratégies pour faire cela est de démarrer un écouteur sur le port 0 de
`localhost`.
Le port 0 n'est pas un port UNIX valide, mais il fonctionne pour faire des
tests.
Le système d'exploitation va obtenir un port TCP ouvert pour nous.

<!--
Instead, in this example we'll write a unit test for the connection handler,
to check that the correct responses are returned for the respective inputs.
To keep our unit test isolated and deterministic, we'll replace the `TcpStream` with a mock.
-->

Dans cet exemple, nous allons plutôt écrire un test unitaire pour le
gestionnaire de connexion, pour vérifier que les réponses correctes soient
retournées à leurs entrées respectives.
Pour faire en sorte que notre test unitaire soit isolé et déterminé, nous
allons remplacer le `TcpStream` par un mock.

<!--
First, we'll change the signature of `handle_connection` to make it easier to test.
`handle_connection` doesn't actually require an `async_std::net::TcpStream`;
it requires any struct that implements `async_std::io::Read`, `async_std::io::Write`, and `marker::Unpin`.
Changing the type signature to reflect this allows us to pass a mock for testing.
```rust,ignore
use std::marker::Unpin;
use async_std::io::{Read, Write};

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
```
-->

Pour commencer, nous allons changer la signature de `gestion_connexion` pour
faciliter ses tests.
En fait, `gestion_connexion` ne nécessite pas de `async_std::net::TcpStream`,
il a juste besoin d'une structure qui implémente `async_std::io::Read`,
`async_std::io::Write`, et `marker::Unpin`.
Changeons la signature du type dans ce sens nous permet de lui passer un mock
pour la tester.

```rust,ignore
use std::marker::Unpin;
use async_std::io::{Read, Write};

async fn gestion_connexion(mut stream: impl Read + Write + Unpin) {
```

<!--
Next, let's build a mock `TcpStream` that implements these traits.
First, let's implement the `Read` trait, with one method, `poll_read`.
Our mock `TcpStream` will contain some data that is copied into the read buffer,
and we'll return `Poll::Ready` to signify that the read is complete.
```rust,ignore
{{#include ../../examples-sources/09_05_final_tcp_server/src/main.rs:mock_read}}
```
-->

Ensuite, créons un mock de `TcpStream` qui implémente ces traits.
Implémentons d'abord le trait `Read`, qui a une méthode, `poll_read`.
Notre mock de `TcpStream` va contenir certaines données qui sont copiées dans
le tampon de lecture, et nous allons retourner `Poll::Ready` pour signaler que
la lecture est terminée.

```rust,ignore
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:mock_read}}
```

<!--
Our implementation of `Write` is very similar,
although we'll need to write three methods: `poll_write`, `poll_flush`, and `poll_close`.
`poll_write` will copy any input data into the mock `TcpStream`, and return `Poll::Ready` when complete.
No work needs to be done to flush or close the mock `TcpStream`, so `poll_flush` and `poll_close`
can just return `Poll::Ready`.
```rust,ignore
{{#include ../../examples-sources/09_05_final_tcp_server/src/main.rs:mock_write}}
```
-->

Notre implémentation de `Write` y ressemble beaucoup, même si nous avons besoin
d'écrire trois méthodes : `poll_write`, `poll_flush`, et `poll_close`.
`poll_write` va copier toutes les données d'entrée dans le mock de `TcpStream`,
et retourne `Poll::Ready` lorsqu'elle sera terminée.
Il n'y a pas besoin de purger et fermer le mock de `TcpStream`, donc
`poll_flush` et `poll_close` peuvent simplement retourner `Poll::Ready`.

```rust,ignore
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:mock_write}}
```

<!--
Lastly, our mock will need to implement `Unpin`, signifying that its location in memory can safely be moved.
For more information on pinning and the `Unpin` trait, see the [section on pinning](../04_pinning/01_chapter.md).
```rust,ignore
{{#include ../../examples-sources/09_05_final_tcp_server/src/main.rs:unpin}}
```
-->

Enfin, notre mock a besoin d'implémenter `Unpin`, ce qui veut dire que sa
position dans la mémoire peut être déplacée en toute sécurité. Pour plus
d'informations sur l'épinglage et le trait `Unpin`, rendez-vous à la [section
sur l'épinglage](../04_pinning/01_chapter.md).

```rust,ignore
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:unpin}}
```

<!--
Now we're ready to test the `handle_connection` function.
After setting up the `MockTcpStream` containing some initial data,
we can run `handle_connection` using the attribute `#[async_std::test]`, similarly to how we used `#[async_std::main]`.
To ensure that `handle_connection` works as intended, we'll check that the correct data
was written to the `MockTcpStream` based on its initial contents.
```rust,ignore
{{#include ../../examples-sources/09_05_final_tcp_server/src/main.rs:test}}
```
-->

Maintenant nous sommes prêts à tester la fonction `gestion_connexion`.
Après avoir réglé le `MockTcpStream` pour contenir les données initiales, nous
pouvons exécuter `gestion_connexion` en utilisant l'attribut
`#[async_std::test]`, de la même manière que nous avons utilisé
`#[async_std::main]`.
Pour nous assurer que `gestion_connexion` fonctionne comme attendu, nous
allons vérifier que les données ont été correctement écrites dans le
`MockTcpStream` en fonction de son contenu initial.

```rust,ignore
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:test}}
```
