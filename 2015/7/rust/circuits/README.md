# Circuits

## TODO
- build dependency graph for symbols. at each new value resolved, collapse their resolved dependencies, artifax style.
- if we're only interested in a particular wire, we don't have to evaluate the whole circuit!
- the dependency graph should be a DAG.
- could we instead unwrap the whole expression? 

```
b AND c <- a
d OR e <- c
NOT f <- e
LSHIFT w 3 <- f
b <- 2

b AND (c OR (NOT (LSHIFT w 3)) <- a, resolve b, c, and w and then finally a.
```

- if the wire is provided, I could truncate the ts soon as the wire shows up.
- does work but savings are negligible for those input sizes.
