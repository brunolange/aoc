How many solutions are there to:

$$
a + b + c + d = 100
$$

where $a$, $b$, $c$, and $d$ are natural numbers?

$$
||||||+|||...|+|||||+| \\
6 + 94 + 5 + 1
$$

So the answer is

$$
\binom{103}{100,3} = \frac{103 \times 102 \times 101 \times 100!}{100! \times 3!} = 176,851
$$

It would be nice to have an iterator that yields every solution, one at a time, with $\mathcal{O}(1)$ memory footprint.
