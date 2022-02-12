<!--
# Recursion
-->

# La récursivité

<!--
Internally, `async fn` creates a state machine type containing each
sub-`Future` being `.await`ed. This makes recursive `async fn`s a little
tricky, since the resulting state machine type has to contain itself:
-->

En interne, une fonction asynchrone génère une machine à états qui contient
chaque sous-future qui sont attendus avec `await`. Cela rend la récursivité des
fonctions asynchrones un peu compliqué, car la machine à état doit se contenir
elle-même :

<!--
```rust,edition2018
# async fn step_one() { /* ... */ }
# async fn step_two() { /* ... */ }
# struct StepOne;
# struct StepTwo;
// This function:
async fn foo() {
    step_one().await;
    step_two().await;
}
// generates a type like this:
enum Foo {
    First(StepOne),
    Second(StepTwo),
}

// So this function:
async fn recursive() {
    recursive().await;
    recursive().await;
}

// generates a type like this:
enum Recursive {
    First(Recursive),
    Second(Recursive),
}
```
-->

```rust,edition2018
# async fn etape_une() { /* ... */ }
# async fn etape_deux() { /* ... */ }
# struct EtapeUne;
# struct EtapeDeux;
// Cette fonction ...
async fn alpha() {
    etape_une().await;
    etape_deux().await;
}
// ... génère un type comme celui-ci :
enum Alpha {
    Premiere(EtapeUne),
    Seconde(EtapeDeux),
}

// Donc cette fonction ...
async fn recursif() {
    recursif().await;
    recursif().await;
}

// ... génère un type comme celui-ci :
enum Recursif {
    Premiere(Recursif),
    Seconde(Recursif),
}
```

<!--
This won't work—we've created an infinitely-sized type!
The compiler will complain:
-->

Cela ne fonctionne pas, nous avons créé un type de taille infinie !
Le compilateur va se plaindre :

<!--
```
error[E0733]: recursion in an `async fn` requires boxing
 -- > src/lib.rs:1:22
  |
1 | async fn recursive() {
  |                      ^ an `async fn` cannot invoke itself directly
  |
  = note: a recursive `async fn` must be rewritten to return a boxed future.
```
-->

```
error[E0733]: recursion in an `async fn` requires boxing
 -- > src/lib.rs:1:22
  |
1 | async fn recursif() {
  |                     ^ an `async fn` cannot invoke itself directly
  |
  = note: a recursive `async fn` must be rewritten to return a boxed future.
```

<!--
In order to allow this, we have to introduce an indirection using `Box`.
Unfortunately, compiler limitations mean that just wrapping the calls to
`recursive()` in `Box::pin` isn't enough. To make this work, we have
to make `recursive` into a non-`async` function which returns a `.boxed()`
`async` block:
-->

Pour nous permettre cela, nous devons faire une dérivation en utilisant
`Box`. Malheureusement, les limitations du compilateur font en sorte
qu'envelopper les appels à `recursif()` dans une `Box::pin` n'est pas
suffisant. Pour que cela fonctionne, nous devons transformer `recursif` en
fonction synchrone pour retourner un bloc `async` qui est dans une `Box` :

<!--
```rust,edition2018
{{#include ../../examples-sources/07_05_recursion/src/lib.rs:example}}
```
-->

```rust,edition2018
{{#include ../../examples/07_05_recursion/src/lib.rs:example}}
```
