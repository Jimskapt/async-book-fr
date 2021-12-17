<!--
# The State of Asynchronous Rust
-->

# L'état de l'art de l'asynchrone en Rust

<!--
Parts of async Rust are supported with the same stability guarantees as
synchronous Rust. Other parts are still maturing and will change
over time. With async Rust, you can expect:
-->

Certaines parties du Rust asynchrone sont pris en charge avec les mêmes
garanties de stabilité que le Rust synchrone. Les autres parties sont en cours
de perfectionnement et évolueront avec le temps. Voici ce que vous pouvez
attendre du Rust asynchrone :

<!--
- Outstanding runtime performance for typical concurrent workloads.
- More frequent interaction with advanced language features, such as lifetimes
  and pinning.
- Some compatibility constraints, both between sync and async code, and between
  different async runtimes.
- Higher maintenance burden, due to the ongoing evolution of async runtimes
  and language support.
-->

- D'excellentes performances à l'exécution des charges de travail en concurrence
  classiques.
- Une interaction plus régulière avec les fonctionnalités avancées du langage,
  comme les durées de vie et l'épinglage.
- Des contraintes de compatibilité, à la fois entre le code synchrone et
  asynchrone, et entre les différents environnements d'exécution.
- Une plus grande exigence de maintenance, à cause de l'évolution continue des
  environnements d'exécution asynchrones et du langage.

<!--
In short, async Rust is more difficult to use and can result in a higher
maintenance burden than synchronous Rust,
but gives you best-in-class performance in return.
All areas of async Rust are constantly improving,
so the impact of these issues will wear off over time.
-->

En résumé, le Rust asynchrone est plus difficile à utiliser et peut demander
plus de maintenance que le Rust synchrone, mais il vous procure en retour les
meilleures performances dans le domaine. Tous les éléments du Rust asynchrone
dont en constante amélioration, donc les effets de ces contre-parties
s'estomperont avec le temps.

<!--
## Language and library support
-->

## La prise en charge des bibliothèques et du langage

<!--
While asynchronous programming is supported by Rust itself,
most async applications depend on functionality provided
by community crates.
As such, you need to rely on a mixture of
language features and library support:
-->

Bien que la programmation asynchrone soit fournie par le coeur de Rust, la
plupart des applications asynchrones dépendent des fonctionnalités offertes
par les crates de la communauté.
Par conséquent, vous devez avoir recours à un mélange de fonctionnalités
offertes par le langage et les bibliothèques :

<!--
- The most fundamental traits, types and functions, such as the
  [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html) trait
  are provided by the standard library.
- The `async/await` syntax is supported directly by the Rust compiler.
- Many utility types, macros and functions are provided by the
  [`futures`](https://docs.rs/futures/) crate. They can be used in any async
  Rust application.
- Execution of async code, IO and task spawning are provided by "async
  runtimes", such as Tokio and async-std. Most async applications, and some
  async crates, depend on a specific runtime. See
  ["The Async Ecosystem"](../08_ecosystem/00_chapter.md) section for more
  details.
-->

- Les traits, types et fonctions les plus fondamentaux, comme le trait
  [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html), sont
  fournis par la bibliothèque standard.
- La syntaxe `async/await` est pris en charge directement par le compilateur
  Rust.
- De nombreux types, macros et fonctions utilitaires sont fournis par la crate
  [`futures`](https://docs.rs/futures/). Ils peuvent être utilisés dans de
  nombreuses applications asynchrones en Rust.
- L'exécution du code asynchrone, les entrées/sorties, et la création de tâches
  sont prises en charge par les "environnements d'exécution asynchrone", comme
  Tokio et async-std. La plupart des applications asynchrones, et certaines
  crates asynchrones, dépendent d'un environnement d'exécution précis. Vous
  pouvez consulter la section
  ["L'écosystème asynchrone"](../08_ecosystem/00_chapter.md) pour en savoir
  plus.

<!--
Some language features you may be used to from synchronous Rust are not yet
available in async Rust. Notably, Rust does not let you declare async
functions in traits. Instead, you need to use workarounds to achieve the same
result, which can be more verbose.
-->

Certaines fonctionnalités du langage auquel vous êtes habitué en Rust synchrone
peuvent ne pas être encore disponible en Rust asynchrone. Par exemple, Rust ne
vous permet pas encore de déclarer des fonctions asynchrones dans des traits.
Par conséquent, vous avez besoin de mettre en place des solutions de
substitution pour arriver au même résultat, ce qui peut rendre les choses un
peu plus verbeux.

<!--
## Compiling and debugging
-->

## La compilation et le débogage

<!--
For the most part, compiler- and runtime errors in async Rust work
the same way as they have always done in Rust. There are a few
noteworthy differences:
-->

Dans la plupart des cas, les erreurs de compilateur et d'exécution de Rust
asynchrone fonctionnent de la même manière qu'elles l'ont toujours fait en
Rust. Voici quelques différences intéressantes :

<!--
### Compilation errors
-->

### Les erreurs de compilation

<!--
Compilation errors in async Rust conform to the same high standards as
synchronous Rust, but since async Rust often depends on more complex language
features, such as lifetimes and pinning, you may encounter these types of
errors more frequently.
-->

Les erreurs de compilateur en Rust asynchrone suivent les mêmes règles strictes
que le Rust synchrone, mais comme le Rust asynchrone dépend souvent dépend
souvent de fonctionnalités du langage plus élaborées, comme les durées de vie
et l'épinglage, vous pourriez rencontrer plus régulièrement ces types
d'erreurs.

<!--
### Runtime errors
-->

### Les erreurs à l'exécution

<!--
Whenever the compiler encounters an async function, it generates a state
machine under the hood. Stack traces in async Rust typically contain details
from these state machines, as well as function calls from
the runtime. As such, interpreting stack traces can be a bit more involved than
it would be in synchronous Rust.
-->

A chaque fois que le compilateur va rencontrer une fonction asynchrone, il va
générer une machine à états en arrière-plan. Les traces de la pile en Rust
asynchrone contiennent généralement des informations sur ces machines à états,
ainsi que les appels de fonctions de l'environnement d'exécution. Par
conséquent, l'interprétation des traces de la pile peut être un peu plus ardue
qu'elle le serait en Rust synchrone.

<!--
### New failure modes
-->

### Les nouveaux types d'erreurs

<!--
A few novel failure modes are possible in async Rust, for instance
if you call a blocking function from an async context or if you implement
the `Future` trait incorrectly. Such errors can silently pass both the
compiler and sometimes even unit tests. Having a firm understanding
of the underlying concepts, which this book aims to give you, can help you
avoid these pitfalls.
-->

Quelques nouveaux types d'erreurs sont possibles avec Rust asynchrone, par
exemple si vous appelez une fonction bloquante à partir d'un contexte
asynchrone ou si vous n'implémentez pas correctement le trait `Future`. Ces
erreurs peuvent ne pas être signalées par le compilateur et parfois même par
les tests unitaires. Le but de ce livre est de vous apprendre les principes
fondamentaux pour vous aider à éviter ces pièges.

<!--
## Compatibility considerations
-->

## Remarques à propos de la compatibilité

<!--
Asynchronous and synchronous code cannot always be combined freely.
For instance, you can't directly call an async function from a sync function.
Sync and async code also tend to promote different design patterns, which can
make it difficult to compose code intended for the different environments.
-->

Le code asynchrone et synchrone ne peuvent pas tout le temps être combinés
librement. Par exemple, vous ne pouvez pas appeler directement une fonction
asynchrone à partir d'une fonction synchrone. Le code synchrone et asynchrone
ont aussi tendance à encourager des motifs de conception différents, ce qui
peut rendre difficile de combiner du code destiné aux différents
environnements.

<!--
Even async code cannot always be combined freely. Some crates depend on a
specific async runtime to function. If so, it is usually specified in the
crate's dependency list.
-->

Et même le code asynchrone ne peut pas être combiné librement. Certaines crates
dépendent d'un environnement d'exécution asynchrone pour fonctionner. Si c'est
le cas, c'est souvent précisé dans la liste des dépendances de la crate.

<!--
These compatibility issues can limit your options, so make sure to
research which async runtime and what crates you may need early.
Once you have settled in with a runtime, you won't have to worry
much about compatibility.
-->

Ces problèmes de compatibilité peuvent réduire vos options, donc il vaut mieux
faire assez tôt vos recherches sur les environnements d'exécution asynchrone et
de leurs crates associées. Une fois que vous vous êtes installé dans un
environnement d'exécution, vous n'aurez plus à vous soucier de la
compatibilité.

<!--
## Performance characteristics
-->

## Les performances

<!--
The performance of async Rust depends on the implementation of the
async runtime you're using.
Even though the runtimes that power async Rust applications are relatively new,
they perform exceptionally well for most practical workloads.
-->

La performance du Rust asynchrone dépend de l'implémentation de l'environnement
d'exécution asynchrone que vous choisissez.
Même si les environnements d'exécution qui propulsent les applications
asynchrones en Rust sont relativement récents, ils sont remarquablement
performants pour la plupart des charges de travail.

<!--
That said, most of the async ecosystem assumes a _multi-threaded_ runtime.
This makes it difficult to enjoy the theoretical performance benefits
of single-threaded async applications, namely cheaper synchronization.
Another overlooked use-case is _latency sensitive tasks_, which are
important for drivers, GUI applications and so on. Such tasks depend
on runtime and/or OS support in order to be scheduled appropriately.
You can expect better library support for these use cases in the future.
-->

Ceci étant dit, la plupart des écosystèmes asynchrones prévoient un
environnement d'exécution _multi-processus_. Cela rend plus difficile
d'apprécier les bienfaits sur les performances théoriques des applications
asynchrone sur un seul processus, appelée aussi synchronisation allégée.
Un autre domaine d'application sous-côté est celui des _tâches sensibles à la
latence_, qui sont importantes pour les pilotes, les applications avec
interface graphique, parmi d'autres. Ces tâches dépendent de l'environnement
d'exécution et/ou de la prise en charge du système d'exploitation pour être
orchestrées correctement. Vous pouvez donc espérer de meilleures prises en
charge des bibliothèques pour cas d'usages à l'avenir.
