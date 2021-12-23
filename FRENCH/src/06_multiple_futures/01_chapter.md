<!--
# Executing Multiple Futures at a Time
-->

# Exécuter plusieurs futures en même temps

<!--
Up until now, we've mostly executed futures by using `.await`, which blocks
the current task until a particular `Future` completes. However, real
asynchronous applications often need to execute several different
operations concurrently.
-->

Jusqu'à présent, nous avons principalement exécuté les futures en utilisant
`.await`, ce qui bloque la tâche courante jusqu'à ce qu'une `Future` soit
terminée. Cependant, les applications asynchrones de la vraie vie ont souvent
besoin d'exécuter plusieurs opérations différentes en concurrence.

<!--
In this chapter, we'll cover some ways to execute multiple asynchronous
operations at the same time:
-->

Dans ce chapitre, nous allons voir différentes manières d'exécuter plusieurs
opérations asynchrones en même temps :

<!--
- `join!`: waits for futures to all complete
- `select!`: waits for one of several futures to complete
- Spawning: creates a top-level task which ambiently runs a future to completion
- `FuturesUnordered`: a group of futures which yields the result of each subfuture
-->

- `join!` : attends que toutes les futures se terminent
- `select!` : attends qu'une des futures se termine
- Spawning : crée une tâche de haut-niveau qui exécute de manière globale une
  future jusqu'à ce qu'elle se termine
- `FuturesUnordered` : un groupe de futures qui retourne le résultat de chaque
  sous-futures
