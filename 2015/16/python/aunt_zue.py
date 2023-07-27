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


@dataclass
class TickerTape:
    children: int
    cats: int
    samoyeds: int
    pomeranians: int
    akitas: int
    vizslas: int
    goldfish: int
    trees: int
    cars: int
    perfumes: int


def check(aunt_sue: AuntSue, ticker: TickerTape) -> bool:
    solver = z3.Solver()

    for i, (value, op, reading) in enumerate(
        [
            (aunt_sue.children, eq, ticker.children),
            (aunt_sue.cats, gt, ticker.cats),
            (aunt_sue.samoyeds, eq, ticker.samoyeds),
            (aunt_sue.pomeranians, lt, ticker.pomeranians),
            (aunt_sue.akitas, eq, ticker.akitas),
            (aunt_sue.vizslas, eq, ticker.vizslas),
            (aunt_sue.goldfish, lt, ticker.goldfish),
            (aunt_sue.trees, gt, ticker.trees),
            (aunt_sue.cars, eq, ticker.cars),
            (aunt_sue.perfumes, eq, ticker.perfumes),
        ]
    ):
        literal = z3.Int(f"x_{i}")
        solver.add(op(literal, reading))

        if value is None:
            continue
        solver.add(literal == value)

    return solver.check() == z3.sat


def main() -> int:
    match: AuntSue | None = None

    ticker = TickerTape(
        children=3,
        cats=7,
        samoyeds=2,
        pomeranians=3,
        akitas=0,
        vizslas=0,
        goldfish=5,
        trees=3,
        cars=2,
        perfumes=1,
    )

    for aunt_sue in parse_line("input"):
        if check(aunt_sue, ticker):
            match = aunt_sue
            break

    if not match:
        print(f"Found no Aunt Sue...", file=sys.stderr)
        return 1

    print(f"Found Aunt Sue: {match}")
    return 0


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


if __name__ == "__main__":
    raise SystemExit(main())
