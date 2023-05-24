# Circuits

## TODO
- build dependency graph for symbols. at each new value resolved, collapse their resolved dependencies, artifax style.
- if we're only interested in a particular wire, we don't have to evaluate the whole circuit!
- the dependency graph should be a DAG. the direction should be "unlocks", not "depends on".
- maybe we maintain a graph and a co-graph?

