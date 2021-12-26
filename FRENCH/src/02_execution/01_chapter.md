<!--
# Under the Hood: Executing `Future`s and Tasks
-->

# Sous le capot : exécuter les `Future`s et les tâches

<!--
In this section, we'll cover the underlying structure of how `Future`s and
asynchronous tasks are scheduled. If you're only interested in learning
how to write higher-level code that uses existing `Future` types and aren't
interested in the details of how `Future` types work, you can skip ahead to
the `async`/`await` chapter. However, several of the topics discussed in this
chapter are useful for understanding how `async`/`await` code works,
understanding the runtime and performance properties of `async`/`await` code,
and building new asynchronous primitives. If you decide to skip this section
now, you may want to bookmark it to revisit in the future.
-->

Dans cette section, nous allons voir la structure sous-jacente de comment les
`Future`s et les tâches asynchrones sont ordonnancées. Si vous vous intéressez
uniquement à l'apprentissage de l'écriture de code de haut niveau qui utilise
les types `Future` existants et que vous n'êtes pas intéressés par détails du
fonctionnement des types `Future`, vous pouvez sauter au chapitre "`async` et
`await`". Cependant, certains sujets abordés dans ce chapitre sont utiles pour
comprendre comme le code de `async` et `await` fonctionne, ainsi que comprendre
l'environnement d'exécution et les caractéristiques de performance du code
`async` et `await`, ainsi que la création de nouvelles primitives asynchrones.
Si vous décidez de sauter cette section, vous devriez le marquer pour le
consulter à nouveau à l'avenir.

<!--
Now, with that out of the way, let's talk about the `Future` trait.
-->

Maintenance que vous savez cela, parlons du trait `Future`.
