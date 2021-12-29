> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.

<!--
# `async` in Traits
-->

# `async` dans les traits

<!--
Currently, `async fn` cannot be used in traits. The reasons for this are
somewhat complex, but there are plans to remove this restriction in the
future.
-->

Actuellement, les fonctions asynchrones ne peuvent pas être utilisées dans les
traits. Les raisons à cela sont un peu complexes, mais il a des solutions en
préparation pour lever cette restriction à l'avenir.

<!--
In the meantime, however, this can be worked around using the
[async-trait crate from crates.io](https://github.com/dtolnay/async-trait).
-->

Cependant, cette restriction peut être contournée en utilisant la [crate
async-trait à partir de crates.io](https://github.com/dtolnay/async-trait).

<!--
Note that using these trait methods will result in a heap allocation
per-function-call. This is not a significant cost for the vast majority
of applications, but should be considered when deciding whether to use
this functionality in the public API of a low-level function that is expected
to be called millions of times a second.
-->

Notez toutefois que l'utilisation de ces méthodes de trait vont provoquer des
allocations sur le tas à chaque appel de fonction. Cela n'a pas d'impact
significatif sur la grande majorité des applications, mais cela doit être pris
en compte lorsqu'on décide d'utiliser cette fonctionnalité dans l'API publique
d'une fonction bas-niveau qui peut être appelé des millions de fois par
seconde.
