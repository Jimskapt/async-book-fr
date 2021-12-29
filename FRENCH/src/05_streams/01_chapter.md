> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/async-book-fr/pull/8).

<!--
# The `Stream` Trait
-->

# Le trait `Stream`

<!--
The `Stream` trait is similar to `Future` but can yield multiple values before
completing, similar to the `Iterator` trait from the standard library:
-->

Le trait `Stream` ressemble à `Future`, mais peut retourner plusieurs valeurs
avant de se terminer, un peu comme le trait `Iterator` de la bibliothèque
standard :

<!--
```rust,ignore
{{#include ../../examples-sources/05_01_streams/src/lib.rs:stream_trait}}
```
-->

```rust,ignore
{{#include ../../examples/05_01_streams/src/lib.rs:stream_trait}}
```

<!--
One common example of a `Stream` is the `Receiver` for the channel type from
the `futures` crate. It will yield `Some(val)` every time a value is sent
from the `Sender` end, and will yield `None` once the `Sender` has been
dropped and all pending messages have been received:
-->

Un exemple courant d'un `Stream` est le `Receiver` pour le type `channel` de la
crate `futures`. Cela va retourner `Some(val)` à chaque fois qu'une valeur est
envoyée par l'extrémité `Sender`, et va retourner `None` une fois que `Sender`
a été libéré de la mémoire et que tous les messages en cours ont été reçus :

<!--
```rust,edition2018,ignore
{{#include ../../examples-sources/05_01_streams/src/lib.rs:channels}}
```
-->

```rust,edition2018,ignore
{{#include ../../examples/05_01_streams/src/lib.rs:channels}}
```
