# A Z3 primer

Z3 is a theorem prover. It can be used to check the satisfiability of logical formulas over one or more theories.

### Z3 as a SAT solver


```python
from z3 import *

s = Solver()
x1 = Bool("x1")
x2 = Bool("x2")
x3 = Bool("x3")
x4 = Bool("x4")

"""
SAT Instance (CNF)

        (x1 or x3) 
    and x3
    and (x3 or x4)
"""
s.add(Or(x1, x2))
s.add(x3)  # unit clause
s.add(Or(x3, Not(x4)))

s.check()  # sat
s.model()  # [x3 = True, x2 = False, x1 = True, x4 = False]
```

### Z3 is a SMT Solver

SMT (Satisfiability Modulo Theories) generalize SAT to more complex formulas involving arbitrary data structures (numbers, arrays, etc...).

```python
from z3 import *

s = Solver()
x1 = Real("x1")
x2 = Real("x2")

s.add(x1**2 + x2**2 == 25)
s.add(x1 + x2 == -1)

s.check()  # sat
s.model()  # [x2 = -4, x1 = 3]
```