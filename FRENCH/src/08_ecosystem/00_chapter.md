> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.

<!--
# The Async Ecosystem
Rust currently provides only the bare essentials for writing async code.
Importantly, executors, tasks, reactors, combinators, and low-level I/O futures and traits
are not yet provided in the standard library. In the meantime,
community-provided async ecosystems fill in these gaps.
-->

# L'écosystème asynchrone

Actuellement, Rust ne fournit que l'essentiel pour écrire du code asynchrone.
En particulier, les exécuteurs, les tâches, les réacteurs, les combinateurs, et
les futures et les traits de bas-niveau d'entrée/sortie ne sont pas encore
fournis par la bibliothèque standard. Mais en attendant, les écosystèmes
asynchrones fournis par la communauté répondent à ce besoin.

<!--
The Async Foundations Team is interested in extending examples in the Async Book to cover multiple runtimes.
If you're interested in contributing to this project, please reach out to us on
[Zulip](https://rust-lang.zulipchat.com/#narrow/stream/201246-wg-async-foundations.2Fbook).
-->

L'équipe en charge des fondations de l'asynchrone est intéressée par le
développement dans le livre sur l'asynchrone pour couvrir plusieurs
environnements d'exécution. Si vous êtes intéressé pour contribuer à ce projet,
veuillez vous rendre sur
[Zulip](https://rust-lang.zulipchat.com/#narrow/stream/201246-wg-async-foundations.2Fbook).

<!--
## Async Runtimes
Async runtimes are libraries used for executing async applications.
Runtimes usually bundle together a *reactor* with one or more *executors*.
Reactors provide subscription mechanisms for external events, like async I/O, interprocess communication, and timers.
In an async runtime, subscribers are typically futures representing low-level I/O operations.
Executors handle the scheduling and execution of tasks.
They keep track of running and suspended tasks, poll futures to completion, and wake tasks when they can make progress.
The word "executor" is frequently used interchangeably with "runtime".
Here, we use the word "ecosystem" to describe a runtime bundled with compatible traits and features.
-->

## Les environnements d'exécution asynchrone

Les environnements d'exécution asynchrones sont des bibliothèques utilisées
pour exécuter des applications asynchrones.
Les environnements d'exécution embarquent généralement ensemble un *réacteur*
avec un ou plusieurs *exécuteurs*.
Les réacteurs fournissent des mécanismes d'abonnement pour les évènements
externes, comme les entrées/sorties asynchrones, la communication entre les
processus, et les temporisations.
Dans un environnement d'exécution asynchrone, les abonnés sont typiquement des
futures qui représentent les opérations d'entrées/sorties de bas-niveau.
Les exécuteurs gèrent la planification et l'exécution des tâches.
Ils assurent le suivi les tâches en cours d'exécution et celles qui sont
suspendues, l'appel des futures jusqu'à ce qu'elles terminent, et réaniment les
tâches lorsqu'elles peuvent progresser.
Le mot "exécuteur" est souvent permuté avec "l'environnement d'exécution".
Ici, nous utilisons le mot "écosystème" pour décrire un environnement
d'exécution accompagné des traits et fonctionnalités compatibles.

<!--
## Community-Provided Async Crates
-->

## Les crates asynchrones fournies par la communauté

<!--
### The Futures Crate
The [`futures` crate](https://docs.rs/futures/) contains traits and functions useful for writing async code.
This includes the `Stream`, `Sink`, `AsyncRead`, and `AsyncWrite` traits, and utilities such as combinators.
These utilities and traits may eventually become part of the standard library.
-->

### La crate `Futures`

La [crate `futures`](https://docs.rs/futures/) contient les traits et les
fonctions utiles pour écrire du code asynchrone. Cela comprend les traits
`Stream`, `Sink`, `AsyncRead`, et `AsyncWrite`, et des utilitaires comme les
combinateurs. Ces utilitaires et ces traits pourraient éventuellement faire
partie un jour de la bibliothèque standard.

<!--
`futures` has its own executor, but not its own reactor, so it does not support execution of async I/O or timer futures.
For this reason, it's not considered a full runtime.
A common choice is to use utilities from `futures` with an executor from another crate.
-->

Les `futures` ont leur propre exécuteur, mais pas son propre réacteur, donc
cela ne prend pas en charge l'exécution d'entrées/sorties asynchrones ou de
futures de temporisation. C'est pour cette raison que ce n'est pas considéré
comme un environnement d'exécution complet.
Il est courant d'employer les utilitaires de `futures` avec un exécuteur d'une
autre crate.

<!--
### Popular Async Runtimes
There is no asynchronous runtime in the standard library, and none are officially recommended.
The following crates provide popular runtimes.
- [Tokio](https://docs.rs/tokio/): A popular async ecosystem with HTTP, gRPC, and tracing frameworks.
- [async-std](https://docs.rs/async-std/): A crate that provides asynchronous counterparts to standard library components.
- [smol](https://docs.rs/smol/): A small, simplified async runtime.
Provides the `Async` trait that can be used to wrap structs like `UnixStream` or `TcpListener`.
- [fuchsia-async](https://fuchsia.googlesource.com/fuchsia/+/master/src/lib/fuchsia-async/):
An executor for use in the Fuchsia OS.
-->

### Les environnements d'exécution asynchrones populaires

Il n'y a pas d'environnement d'exécution asynchrone dans la bibliothèque
standard, et aucune n'est officiellement recommandée.
Les crates suivantes offrent des environnement d'exécution populaires.

- [Tokio](https://docs.rs/tokio/) : un écosystème asynchrone populaire pour des
  cadriciels travaillant avec HTTP, gRPC et du traçage.
- [async-std](https://docs.rs/async-std/) : une crate qui fournit des
  équivalents asynchrones aux composants de la bibliothèque standard.
- [smol](https://docs.rs/smol/) : un environnement d'exécution asynchrone,
  minimisé et simplifié.

<!--
## Determining Ecosystem Compatibility
Not all async applications, frameworks, and libraries are compatible with each other, or with every OS or platform.
Most async code can be used with any ecosystem, but some frameworks and libraries require the use of a specific ecosystem.
Ecosystem constraints are not always documented, but there are several rules of thumb to determine
whether a library, trait, or function depends on a specific ecosystem.
-->

## La compatibilité des écosystèmes

Toutes les applications, cadriciels, et bibliothèques asynchrones ne sont pas
compatibles entre elles, ou avec tous les systèmes d'exploitation ou
plateformes. La plupart du code asynchrone peut être utilisé avec n'importe
quel écosystème, mais certains cadriciels et bibliothèques nécessitent
l'utilisation d'un écosystème précis. Les contraintes d'un écosystème ne sont
pas toujours documentées, mais quelques méthodes empiriques pour déterminer si
une bibliothèque, un trait, ou une fonction dépends d'un écosystème précis.

<!--
Any async code that interacts with async I/O, timers, interprocess communication, or tasks
generally depends on a specific async executor or reactor.
All other async code, such as async expressions, combinators, synchronization types, and streams
are usually ecosystem independent, provided that any nested futures are also ecosystem independent.
Before beginning a project, it's recommended to research relevant async frameworks and libraries to ensure
compatibility with your chosen runtime and with each other.
-->

Tout code asynchrone qui interagit avec des entrées/sorties, temporisations,
communication inter-processus, ou des tâches asynchrones dépend généralement
d'un exécuteur ou réacteur asynchrone.
Tous les autres codes asynchrones, comme les expressions, combinateurs, types
de sychronisation, et les `Stream` asynchrones sont généralement indépendants
des écosystèmes, à condition que toutes les futures imbriquées sont aussi
indépendantes de tout écosystème.
Avant de commencer un projet, il est recommandé de rechercher les cadriciels
et bibliothèques asynchrones que vous aurez besoin pour vous assurer la
compatibilité entre eux et avec l'environnement d'exécution que vous avez
choisi.

<!--
Notably, `Tokio` uses the `mio` reactor and defines its own versions of async I/O traits,
including `AsyncRead` and `AsyncWrite`.
On its own, it's not compatible with `async-std` and `smol`,
which rely on the [`async-executor` crate](https://docs.rs/async-executor), and the `AsyncRead` and `AsyncWrite`
traits defined in `futures`.
-->

En particulier, `Tokio` utilise le réacteur `mio` et définit ses propres
versions des traits d'entrées/sorties asynchrones, y compris `AsyncRead` et
`AsyncWrite`. Seul, il n'est pas compatible avec `async-std` et `smol`, qui
reposent sur [la crate `async-executor`](https://docs.rs/async-executor), et
les traits `AsyncRead` et `AsyncWrite` sont définis dans `futures`.

<!--
Conflicting runtime requirements can sometimes be resolved by compatibility layers
that allow you to call code written for one runtime within another.
For example, the [`async_compat` crate](https://docs.rs/async_compat) provides a compatibility layer between
`Tokio` and other runtimes.
-->

Les pré-requis d'environnement d'exécution en conflit peuvent parfois être
résolus avec des couches de compatibilité qui vous permettent d'appeler du code
écrit pour un environnement d'exécution dans un autre.
Par exemple, [la crate `async_compat`](https://docs.rs/async_compat) fournit
une couche de compatibilité entre `Tokio` et les autres environnements
d'exécution.

<!--
Libraries exposing async APIs should not depend on a specific executor or reactor,
unless they need to spawn tasks or define their own async I/O or timer futures.
Ideally, only binaries should be responsible for scheduling and running tasks.
-->

Les bibliothèques qui exposent des APIs asynchrones ne devraient pas dépendre
d'un exécuteur ou d'un réacteur en particulier, à moins qu'ils aient besoin de
créer des tâches ou de définir leurs propres entrées/sorties asynchrones ou des
futures de temporisation. Dans l'idéal, seuls les binaires devraient être
responsables de la planification et de l'exécution des tâches.

<!--
## Single Threaded vs Multi-Threaded Executors
Async executors can be single-threaded or multi-threaded.
For example, the `async-executor` crate has both a single-threaded `LocalExecutor` and a multi-threaded `Executor`.
-->

## Les exécuteurs mono-processus versus multi-processus

Les exécuteurs asynchrones peuvent être mono-processus ou multi-processus.
Par exemple, la crate `async-executor` a deux `LocalExecutor` mono-processus et
un `Executor` multi-processus.

<!--
A multi-threaded executor makes progress on several tasks simultaneously.
It can speed up the execution greatly for workloads with many tasks,
but synchronizing data between tasks is usually more expensive.
It is recommended to measure performance for your application
when you are choosing between a single- and a multi-threaded runtime.
-->

Les exécuteurs multi-processus permettent de faire progresser plusieurs tâches
en simultané. Cela peut accélérer considérablement l'exécution pour les charges
de travail avec beaucoup de tâches, mais la synchronisation des données entre
les tâches est habituellement moins rentable. Il est recommandé de mesurer les
performances de votre application lorsque vous choisissez entre un
environnement d'exécution mono-processus et multi-processus.

<!--
Tasks can either be run on the thread that created them or on a separate thread.
Async runtimes often provide functionality for spawning tasks onto separate threads.
Even if tasks are executed on separate threads, they should still be non-blocking.
In order to schedule tasks on a multi-threaded executor, they must also be `Send`.
Some runtimes provide functions for spawning non-`Send` tasks,
which ensures every task is executed on the thread that spawned it.
They may also provide functions for spawning blocking tasks onto dedicated threads,
which is useful for running blocking synchronous code from other libraries.
-->

Les tâches peuvent être exécutées soit sur le processus qui les a créés, ou sur
processus séparé. Les environnements d'exécution asynchrones fournissent
souvent des fonctionnalités pour créer des tâches sur des processus séparés.
Même si les tâches sont exécutées sur des processus séparés, ils ne doivent
toujours pas être bloquants. Pour pouvoir planifier l'exécution des tâches sur
un exécuteur multi-processus, elles doivent être aussi être `Send`. Certains
environnements d'exécution fournissent des fonctions pour créer des tâches qui
ne sont pas `Send`, ce qui permet de s'assurer que les tâches sont exécutées
sur le processus qui les ont créés.
Ils peuvent également fournir des fonctions pour créer des tâches bloquantes
sur des processus dédiés, ce qui est pratique pour exécuter du code synchrone
bloquant des autres bibliothèques.
