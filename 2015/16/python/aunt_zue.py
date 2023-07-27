import sys
from dataclasses import dataclass
from operator import eq, gt, lt
from typing import Iterator

import z3


@dataclass
class AuntSue:
    name: str
    children: int | None
    cats: int | None
    samoyeds: int | None
    pomeranians: int | None
    akitas: int | None
    vizslas: int | None
    goldfish: int | None
    trees: int | None
    cars: int | None
    perfumes: int | None


def parse_line(filename) -> Iterator[AuntSue]:
    with open(filename) as handle:
        for line in handle.readlines():
            line = line.strip()
            name, properties = line.split(": ", 1)
            attrs: dict[str, int] = {
                k: int(v) for k, v in (kv.split(": ") for kv in properties.split(", "))
            }
            yield AuntSue(
                name=name,
                children=attrs.get("children"),
                cats=attrs.get("cats"),
                samoyeds=attrs.get("samoyeds"),
                pomeranians=attrs.get("pomeranians"),
                akitas=attrs.get("akitas"),
                vizslas=attrs.get("vizslas"),
                goldfish=attrs.get("goldfish"),
                trees=attrs.get("trees"),
                cars=attrs.get("cars"),
                perfumes=attrs.get("perfumes"),
            )


def main() -> int:
    match: AuntSue | None = None

    for aunt_sue in parse_line("input"):
        solver = z3.Solver()

        for i, (value, op, reading) in enumerate(
            [
                (aunt_sue.children, eq, 3),
                (aunt_sue.cats, gt, 7),
                (aunt_sue.samoyeds, eq, 2),
                (aunt_sue.pomeranians, lt, 3),
                (aunt_sue.akitas, eq, 0),
                (aunt_sue.vizslas, eq, 0),
                (aunt_sue.goldfish, lt, 5),
                (aunt_sue.trees, gt, 3),
                (aunt_sue.cars, eq, 2),
                (aunt_sue.perfumes, eq, 1),
            ]
        ):
            literal = z3.Int(f"x_{i}")
            solver.add(op(literal, reading))
            if value is None:
                continue

            solver.add(literal == value)

        if solver.check() == z3.sat:
            breakpoint()
            match = aunt_sue
            break

    if not match:
        print(f"Found no Aunt Sue...", file=sys.stderr)
        return 1

    print(f"Found Aunt Sue: {match}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
