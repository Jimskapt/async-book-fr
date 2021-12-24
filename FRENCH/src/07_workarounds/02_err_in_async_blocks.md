<!--
# `?` in `async` Blocks
-->

# `?` dans les blocs `async`

<!--
Just as in `async fn`, it's common to use `?` inside `async` blocks.
However, the return type of `async` blocks isn't explicitly stated.
This can cause the compiler to fail to infer the error type of the
`async` block.
-->

Tout comme dans `async fn`, il est courant d'utiliser `?` dans des blocs
`async`. Cependant, le type de retour des blocs `async` n'a pas d'état
explicite. Cela peut faire échouer le compilateur à déduire le type d'erreur du
bloc `async`.

<!--
For example, this code:
-->

Par exemple, ce code ...

<!--
```rust,edition2018
# struct MyError;
# async fn foo() -> Result<(), MyError> { Ok(()) }
# async fn bar() -> Result<(), MyError> { Ok(()) }
let fut = async {
    foo().await?;
    bar().await?;
    Ok(())
};
```
-->

```rust,edition2018
# struct MonErreur;
# async fn alpha() -> Result<(), MonErreur> { Ok(()) }
# async fn beta() -> Result<(), MonErreur> { Ok(()) }
let future = async {
    alpha().await?;
    beta().await?;
    Ok(())
};
```

<!--
will trigger this error:
-->

... va déclencher cette erreur :

<!--
```
error[E0282]: type annotations needed
 -- > src/main.rs:5:9
  |
4 |     let fut = async {
  |         --- consider giving `fut` a type
5 |         foo().await?;
  |         ^^^^^^^^^^^^ cannot infer type
```
-->

```
error[E0282]: type annotations needed
 -- > src/main.rs:5:9
  |
4 |     let future = async {
  |         ------ consider giving `fut` a type
5 |         alpha().await?;
  |         ^^^^^^^^^^^^^^ cannot infer type
```

<!--
Unfortunately, there's currently no way to "give `fut` a type", nor a way
to explicitly specify the return type of an `async` block.
To work around this, use the "turbofish" operator to supply the success and
error types for the `async` block:
-->

Malheureusement, il n'existe pas pour l'instant de façon de "donner un type à
`future`", ni une manière pour préciser explicitement le type de retour d'un
bloc `async`.
Pour contourner cela, utilisez l'opérateur "turbofish" pour renseigner les
types de succès et d'erreur pour le bloc `async` :

<!--
```rust,edition2018
# struct MyError;
# async fn foo() -> Result<(), MyError> { Ok(()) }
# async fn bar() -> Result<(), MyError> { Ok(()) }
let fut = async {
    foo().await?;
    bar().await?;
    Ok::<(), MyError>(()) // <- note the explicit type annotation here
};
```
-->

```rust,edition2018
# struct MonErreur;
# async fn alpha() -> Result<(), MonErreur> { Ok(()) }
# async fn beta() -> Result<(), MonErreur> { Ok(()) }
let future = async {
    alpha().await?;
    beta().await?;
    Ok::<(), MonErreur>(()) // <- remarquez l'annotation de type explicite ici
};
```
