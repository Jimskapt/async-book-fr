> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/async-book-fr/pull/2).

<!--
# Why Async?
-->

# Pourquoi l'asynchrone ?

<!--
We all love how Rust empowers us to write fast, safe software.
But how does asynchronous programming fit into this vision?
-->

Nous apprécions tous la façon dont Rust nous permet d'écrire rapidement des
programmes sûrs. Mais comment la programmation asynchrone s'inscrit-elle dans
cette démarche ?

<!--
Asynchronous programming, or async for short, is a _concurrent programming model_
supported by an increasing number of programming languages.
It lets you run a large number of concurrent
tasks on a small number of OS threads, while preserving much of the
look and feel of ordinary synchronous programming, through the
`async/await` syntax.
-->

La programmation asynchrone, abrégé async, est un _modèle de programmation
concurrent_ pris en charge par un nombre croissant de langages de
programmation. Il vous permet d'exécuter un grand nombre de tâches concurrentes
sur un petit nombre de processus du Système d'Exploitation, tout en conservant
l'apparence et la convivialité de la programmation synchrone habituelle, grâce
à la syntaxe `async/await`.

<!--
## Async vs other concurrency models
-->

## L'asynchrone et les autres modèles de concurrence

<!--
Concurrent programming is less mature and "standardized" than
regular, sequential programming. As a result, we express concurrency
differently depending on which concurrent programming model
the language is supporting.
A brief overview of the most popular concurrency models can help
you understand how asynchronous programming fits within the broader
field of concurrent programming:
-->

La programmation concurrente est moins mûre et moins "formalisée" que la
programmation séquentielle classique. Par conséquent, nous formulons la
concurrence différemment selon le modèle de programmation pris en charge par le
langage.
Un bref panorama des modèles de concurrence les plus populaires peut vous aider
à comprendre où se situe la programmation asynchrone dans le domaine plus large
de la programmation asynchrone :

<!--
- **OS threads** don't require any changes to the programming model,
  which makes it very easy to express concurrency. However, synchronizing
  between threads can be difficult, and the performance overhead is large.
  Thread pools can mitigate some of these costs, but not enough to support
  massive IO-bound workloads.
- **Event-driven programming**, in conjunction with _callbacks_, can be very
  performant, but tends to result in a verbose, "non-linear" control flow.
  Data flow and error propagation is often hard to follow.
- **Coroutines**, like threads, don't require changes to the programming model,
  which makes them easy to use. Like async, they can also support a large
  number of tasks. However, they abstract away low-level details that
  are important for systems programming and custom runtime implementors.
- **The actor model** divides all concurrent computation into units called
  actors, which communicate through fallible message passing, much like
  in distributed systems. The actor model can be efficiently implemented, but it leaves
  many practical issues unanswered, such as flow control and retry logic.
-->

- Les **processus du système d'exploitation** ne nécessitent aucun changement
  dans le modèle de programmation, ce qui facilite l'expression de la
  concurrence. Cependant, la synchronisation entre les processus peut être
  difficile, et la conséquence sur les performances est importante. Les groupes
  de processus peuvent réduire certains coûts, mais pas suffisamment pour faire
  face à la charge de travail d'une grosse masse d'entrées/sorties.
- La **programmation orientée évènements**, conjuguée avec les _fonctions de
  rappel_, peut s'avérer très performante, mais a tendance à produire un
  contrôle de flux "non-linéaire" et verbeux. Les flux de données et les
  propagations d'erreurs sont souvent difficiles à suivre.
- Les **coroutines**, comme les processus, ne nécessitent pas de changements
  sur le modèle de programmation, ce qui facilite leur utilisation. Comme
  l'asynchrone, elles peuvent supporter de nombreuses tâches. Cependant, elles
  font abstraction des détails de bas niveau, qui sont importants pour la
  programmation système et les implémentations personnalisées d'environnements
  d'exécution.
- Le **modèle acteur** divise tous les calculs concurrents en différentes
  parties que l'on appelle acteurs, qui communiquent par le biais de passage de
  messages faillibles, comme dans les systèmes distribués. Le modèle d'acteur
  peut être implémenté efficacement, mais il ne répondra pas à tous les
  problèmes, comme le contrôle de flux et la logique de relance.

<!--
In summary, asynchronous programming allows highly performant implementations
that are suitable for low-level languages like Rust, while providing
most of the ergonomic benefits of threads and coroutines.
-->

En résumé, la programmation asynchrone permet des implémentations très
performantes qui sont nécessaires pour des langages bas-niveau comme Rust, tout
en offrant les avantages ergonomiques aux processus et aux coroutines.

<!--
## Async in Rust vs other languages
-->

## L'asynchrone en Rust et dans les autres langages

<!--
Although asynchronous programming is supported in many languages, some
details vary across implementations. Rust's implementation of async
differs from most languages in a few ways:
-->

Bien que la programmation asynchrone soit prise en charge dans de nombreux
langages, certains détails changent selon les implémentations. L'implémentation
en Rust de async se distingue des autres langages de plusieurs manières :

<!--
- **Futures are inert** in Rust and make progress only when polled. Dropping a
  future stops it from making further progress.
- **Async is zero-cost** in Rust, which means that you only pay for what you use.
  Specifically, you can use async without heap allocations and dynamic dispatch,
  which is great for performance!
  This also lets you use async in constrained environments, such as embedded systems.
- **No built-in runtime** is provided by Rust. Instead, runtimes are provided by
  community maintained crates.
- **Both single- and multithreaded** runtimes are available in Rust, which have
  different strengths and weaknesses.
-->

- Les **futures sont inertes** en Rust et progressent uniquement lorsqu'elles
  sont sollicitées. Libérer une future va arrêter sa progression.
- **L'asynchrone n'a pas de coût** en Rust, ce qui signifie que vous ne payez que
  ce que vous utilisez. Plus précisément, vous pouvez utiliser async sans
  allouer sur le tas et sans répartition dynamique, ce qui est très intéressant
  pour les performances !
  Cela vous permet également d'utiliser async dans des environnements
  restreints, comme par exemple sur des systèmes embarqués.
- **Il n'y a pas d'environnement d'exécution intégré** par défaut dans Rust. Par
  contre, des environnements d'exécution sont disponibles dans des crates maintenues
  par la communauté.
- **Des environnements d'exécution mono-processus et multi-processus** existent
  en Rust, qui ont chacun leurs avantages et inconvénients.

<!--
## Async vs threads in Rust
-->

## L'asynchrone et les processus en Rust

<!--
The primary alternative to async in Rust is using OS threads, either
directly through [`std::thread`](https://doc.rust-lang.org/std/thread/)
or indirectly through a thread pool.
Migrating from threads to async or vice versa
typically requires major refactoring work, both in terms of implementation and
(if you are building a library) any exposed public interfaces. As such,
picking the model that suits your needs early can save a lot of development time.
-->

La première alternative à l'asynchrone en Rust est d'utiliser les processus du
Système d'Exploitation, soit directement via
[`std::thread`](https://doc.rust-lang.org/std/thread/), soit indirectement via
un groupe de processus.
La migration des processus vers de l'asynchrone et vice-versa nécessite
généralement un gros chantier de remaniement, que ce soit pour leur implémentation
ou pour leurs interfaces publique (si vous écrivez une bibliothèque) . Par
conséquent, vous pouvez vous épargner beaucoup de temps de développement si
vous choisissez très tôt le modèle qui convient bien à vos besoins.

<!--
**OS threads** are suitable for a small number of tasks, since threads come with
CPU and memory overhead. Spawning and switching between threads
is quite expensive as even idle threads consume system resources.
A thread pool library can help mitigate some of these costs, but not all.
However, threads let you reuse existing synchronous code without significant
code changes—no particular programming model is required.
In some operating systems, you can also change the priority of a thread,
which is useful for drivers and other latency sensitive applications.
-->

Les **processus de Système d'Exploitation** sont préférables pour un petit
nombre de tâches, puisque les processus s'accompagnent d'une surcharge du
processeur et de la mémoire. Créer et basculer entre les processus est assez
gourmand, car même les processus inutilisés consomment des ressources système.
Une bibliothèque implémentant des groupe de tâches peut aider à atténuer certains
coûts, mais pas tous. Cependant, les processus vous permet de réutiliser du code
synchrone existant sans avoir besoin de changement significatif du code — il n'y
a pas besoin d'avoir de modèle de programmation en particulier.
Avec certains systèmes d'exploitation, vous pouvez aussi changer la priorité
d'un processus, ce qui peut être pratique pour les pilotes et les autres
utilisations sensibles à la latence.

<!--
**Async** provides significantly reduced CPU and memory
overhead, especially for workloads with a
large amount of IO-bound tasks, such as servers and databases.
All else equal, you can have orders of magnitude more tasks than OS threads,
because an async runtime uses a small amount of (expensive) threads to handle
a large amount of (cheap) tasks.
However, async Rust results in larger binary blobs due to the state
machines generated from async functions and since each executable
bundles an async runtime.
-->

**L'asynchrone** permet de réduire significativement la surcharge du processeur
et de la mémoire, en particulier pour les charges de travail avec un grand
nombre de tâches liées à des entrées/sorties, comme les serveurs et les bases
de données. Pour comparaison à la même échelle, vous pouvez avoir un nombre bien
plus élevé de tâches qu'avec les processus du Système d'Exploitation, car comme
un environnement d'exécution asynchrone utilise une petite partie des (coûteux)
processus pour gérer une grande quantité de tâches (peu coûteuses).
Cependant, le Rust asynchrone produit des binaires plus lourds à cause des
machines à états générés à partir des fonctions asynchrones et que par conséquent
chaque exécutable embarque un environnement d'exécution asynchrone.

<!--
On a last note, asynchronous programming is not _better_ than threads,
but different.
If you don't need async for performance reasons, threads can often be
the simpler alternative.
-->

Une dernière remarque, la programmation asynchrone n'est pas _meilleure_ que
les processus, c'est différent.
Si vous n'avez pas besoin de l'asynchrone pour des raisons de performance, les
processus sont souvent une alternative plus simple.

<!--
### Example: Concurrent downloading
-->

### Exemple : un téléchargement concurrent

<!--
In this example our goal is to download two web pages concurrently.
In a typical threaded application we need to spawn threads
to achieve concurrency:
-->

Dans cet exemple, notre objectif est de télécharger deux pages web en
concurrence. Dans une application traditionnelle avec des processus nous avons
besoin de créer des processus pour appliquer la concurrence :

<!--
```rust,ignore
{{#include ../../examples-sources/01_02_why_async/src/lib.rs:get_two_sites}}
```
-->

```rust,ignore
{{#include ../../examples/01_02_why_async/src/lib.rs:get_two_sites}}
```

<!--
However, downloading a web page is a small task; creating a thread
for such a small amount of work is quite wasteful. For a larger application, it
can easily become a bottleneck. In async Rust, we can run these tasks
concurrently without extra threads:
-->

Cependant, le téléchargement d'une page web est une petite tâche, donc créer un
processus pour une si petite quantité de travail est un peu du gaspillage. Pour
une application plus importante, cela peut rapidement devenir un goulot
d'étranglement. Grâce au Rust asynchrone, nous pouvons exécuter ces tâches en
concurrence sans avoir besoin de processus supplémentaires :

<!--
```rust,ignore
{{#include ../../examples-sources/01_02_why_async/src/lib.rs:get_two_sites_async}}
```
-->

```rust,ignore
{{#include ../../examples/01_02_why_async/src/lib.rs:get_two_sites_async}}
```

<!--
Here, no extra threads are created. Additionally, all function calls are statically
dispatched, and there are no heap allocations!
However, we need to write the code to be asynchronous in the first place,
which this book will help you achieve.
-->

Notez bien que ici, il n'y a pas de processus supplémentaires qui sont créés.
De plus, tous les appels à des fonctions sont distribués statiquement, et il
n'y a pas d'allocation sur le tas !
Cependant, nous avons d'abord besoin d'écrire le code pour être asynchrone, ce
que ce livre va vous aider à accomplir.

<!--
## Custom concurrency models in Rust
-->

## Les modèles personnalisés de concurrence en Rust

<!--
On a last note, Rust doesn't force you to choose between threads and async.
You can use both models within the same application, which can be
useful when you have mixed threaded and async dependencies.
In fact, you can even use a different concurrency model altogether,
such as event-driven programming, as long as you find a library that
implements it.
-->

Une dernière remarque, Rust ne vous forçait pas à choisir entre les
processus et l'asynchrone. Vous pouvez utiliser ces deux modèles au sein d'une
même application, ce qui peut être utile lorsque vous mélangez les dépendances
de processus et d'asynchrone.
En fait, vous pouvez même utiliser un modèle de concurrence complètement
différent en même temps, du moment que vous trouvez une bibliothèque qui
l'implémente.
