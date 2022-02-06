> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/async-book-fr/pull/9).

<!--
# Iteration and Concurrency
-->

# L'itération et la concurrence

<!--
Similar to synchronous `Iterator`s, there are many different ways to iterate
over and process the values in a `Stream`. There are combinator-style methods
such as `map`, `filter`, and `fold`, and their early-exit-on-error cousins
`try_map`, `try_filter`, and `try_fold`.
-->

Comme pour les `Iterator`s synchrones, il existe de nombreuses façons pour
itérer sur les valeurs dans un `Stream` et pour les traiter. Il existe des
méthodes conçues pour se combiner, comme `map`, `filter` et `fold`, et leurs
cousines conçues pour s'arrêter dès qu'elles rencontrent une erreur, comme
`try_map`, `try_filter`, et `try_fold`.

<!--
Unfortunately, `for` loops are not usable with `Stream`s, but for
imperative-style code, `while let` and the `next`/`try_next` functions can
be used:
-->

Malheureusement, les boucles `for` ne sont pas utilisables avec les `Stream`,
mais du code plus impératif peut être utilisé, comme `while let` et les
fonctions `next` et `try_next` :

<!--
```rust,edition2018,ignore
{{#include ../../examples-sources/05_02_iteration_and_concurrency/src/lib.rs:nexts}}
```
-->

```rust,edition2018,ignore
{{#include ../../examples/05_02_iteration_and_concurrency/src/lib.rs:nexts}}
```

<!--
However, if we're just processing one element at a time, we're potentially
leaving behind opportunity for concurrency, which is, after all, why we're
writing async code in the first place. To process multiple items from a stream
concurrently, use the `for_each_concurrent` and `try_for_each_concurrent`
methods:
-->

Cependant, si nous ne traitions qu'un seul élément à la fois, nous aurions
probablement gaspillé des occasions de concurrence, ce qui, après tout, est
la raison principale pour laquelle nous écrivons du code asynchrone. Pour
traiter en concurrence plusieurs éléments d'un `Stream`, utilisez les méthodes
`for_each_concurrent` et `try_for_each_concurrent` :

<!--
```rust,edition2018,ignore
{{#include ../../examples-sources/05_02_iteration_and_concurrency/src/lib.rs:try_for_each_concurrent}}
```
-->

```rust,edition2018,ignore
{{#include ../../examples/05_02_iteration_and_concurrency/src/lib.rs:try_for_each_concurrent}}
```
