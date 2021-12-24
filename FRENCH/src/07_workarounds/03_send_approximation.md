<!--
# `Send` Approximation
-->

# L'approximation de `Send`

<!--
Some `async fn` state machines are safe to be sent across threads, while
others are not. Whether or not an `async fn` `Future` is `Send` is determined
by whether a non-`Send` type is held across an `.await` point. The compiler
does its best to approximate when values may be held across an `.await`
point, but this analysis is too conservative in a number of places today.
-->

Certaines machines à état de fonctions asynchrones sont sûres pour être
envoyées entre des processus, alors que d'autres ne le sont pas. Le fait que la
`Future` d'une fonction asynchrone est `Send` ou non est conditionné par le
fait qu'un type `Send` soit maintenu par un `.await`, mais cette approche est
aujourd'hui trop conservatrice sur certains points.

<!--
For example, consider a simple non-`Send` type, perhaps a type
which contains an `Rc`:
-->

Par exemple, imaginez un simple type qui n'est pas `Send`, comme un type qui
contient un `Rc` :

<!--
```rust
use std::rc::Rc;

#[derive(Default)]
struct NotSend(Rc<()>);
```
-->

```rust
use std::rc::Rc;

#[derive(Default)]
struct EstPasSend(Rc<()>);
```

<!--
Variables of type `NotSend` can briefly appear as temporaries in `async fn`s
even when the resulting `Future` type returned by the `async fn` must be `Send`:
-->

Les variables du type `EstPasSend` peuvent intervenir brièvement dans des
fonctions asynchrones même si le type résultant de la `Future` retournée par la
fonction asynchrone doit être `Send` :

<!--
```rust,edition2018
# use std::rc::Rc;
# #[derive(Default)]
# struct NotSend(Rc<()>);
async fn bar() {}
async fn foo() {
    NotSend::default();
    bar().await;
}

fn require_send(_: impl Send) {}

fn main() {
    require_send(foo());
}
```
-->

```rust,edition2018
# use std::rc::Rc;
# #[derive(Default)]
# struct EstPasSend(Rc<()>);
async fn beta() {}
async fn alpha() {
    EstPasSend::default();
    beta().await;
}

fn necessite_send(_: impl Send) {}

fn main() {
    necessite_send(alpha());
}
```

<!--
However, if we change `foo` to store `NotSend` in a variable, this example no
longer compiles:
-->

Cependant, si nous changeons `alpha` pour stocker le `EstPasSend` dans une
variable, cet exemple ne se compile plus :

<!--
```rust,edition2018
# use std::rc::Rc;
# #[derive(Default)]
# struct NotSend(Rc<()>);
# async fn bar() {}
async fn foo() {
    let x = NotSend::default();
    bar().await;
}
# fn require_send(_: impl Send) {}
# fn main() {
#    require_send(foo());
# }
```
-->

```rust,edition2018
# use std::rc::Rc;
# #[derive(Default)]
# struct EstPasSend(Rc<()>);
# async fn beta() {}
async fn alpha() {
    let x = EstPasSend::default();
    beta().await;
}
# fn necessite_send(_: impl Send) {}
# fn main() {
#    necessite_send(alpha());
# }
```

<!--
```
error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
  -- > src/main.rs:15:5
   |
15 |     require_send(foo());
   |     ^^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
   |
   = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
   = note: required because it appears within the type `NotSend`
   = note: required because it appears within the type `{NotSend, impl std::future::Future, ()}`
   = note: required because it appears within the type `[static generator@src/main.rs:7:16: 10:2 {NotSend, impl std::future::Future, ()}]`
   = note: required because it appears within the type `std::future::GenFuture<[static generator@src/main.rs:7:16: 10:2 {NotSend, impl std::future::Future, ()}]>`
   = note: required because it appears within the type `impl std::future::Future`
   = note: required because it appears within the type `impl std::future::Future`
note: required by `require_send`
  -- > src/main.rs:12:1
   |
12 | fn require_send(_: impl Send) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
```
-->

```
error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
  -- > src/main.rs:15:5
   |
15 |     necessite_send(foo());
   |     ^^^^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
   |
   = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
   = note: required because it appears within the type `EstPasSend`
   = note: required because it appears within the type `{EstPasSend, impl std::future::Future, ()}`
   = note: required because it appears within the type `[static generator@src/main.rs:7:16: 10:2 {EstPasSend, impl std::future::Future, ()}]`
   = note: required because it appears within the type `std::future::GenFuture<[static generator@src/main.rs:7:16: 10:2 {EstPasSend, impl std::future::Future, ()}]>`
   = note: required because it appears within the type `impl std::future::Future`
   = note: required because it appears within the type `impl std::future::Future`
note: required by `necessite_send`
  -- > src/main.rs:12:1
   |
12 | fn necessite_send(_: impl Send) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
```

<!--
This error is correct. If we store `x` into a variable, it won't be dropped
until after the `.await`, at which point the `async fn` may be running on
a different thread. Since `Rc` is not `Send`, allowing it to travel across
threads would be unsound. One simple solution to this would be to `drop`
the `Rc` before the `.await`, but unfortunately that does not work today.
-->

Cette erreur est justifiée. Si nous stockons `x` dans une variable, il ne sera
pas libéré avant d'arriver après le `.await`, moment où la fonction asynchrone
s'exécute peut-être sur un processus différent. Comme `Rc` n'est pas `Send`,
lui permettre de voyager entre les processus ne serait pas sain. Une solution
simple à cela serait de libérer le `Rc` avec `drop` avant le `.await`, mais
malheureusement cela ne fonctionne pas aujourd'hui.

<!--
In order to successfully work around this issue, you may have to introduce
a block scope encapsulating any non-`Send` variables. This makes it easier
for the compiler to tell that these variables do not live across an
`.await` point.
-->

Pour contourner ce problème, vous pouvez créer une portée de bloc qui englobe
chacune des variables qui ne sont pas `Send`. Cela permet de dire facilement au
compilateur que ces variables ne vivent plus en dehors de l'utilisation du
`.await`.

<!--
```rust,edition2018
# use std::rc::Rc;
# #[derive(Default)]
# struct NotSend(Rc<()>);
# async fn bar() {}
async fn foo() {
    {
        let x = NotSend::default();
    }
    bar().await;
}
# fn require_send(_: impl Send) {}
# fn main() {
#    require_send(foo());
# }
```
-->

```rust,edition2018
# use std::rc::Rc;
# #[derive(Default)]
# struct EstPasSend(Rc<()>);
# async fn beta() {}
async fn alpha() {
    {
        let x = EstPasSend::default();
    }
    beta().await;
}
# fn necessite_send(_: impl Send) {}
# fn main() {
#    necessite_send(alpha());
# }
```
