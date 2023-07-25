"""
children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1
"""
import z3

from dataclasses import dataclass


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
class MFCSAM:
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

    def test(self, aunt_sue: AuntSue) -> bool:
        left = (
            self.children,
            self.cats,
            self.samoyeds,
            self.pomeranians,
            self.akitas,
            self.vizslas,
            self.goldfish,
            self.trees,
            self.cars,
            self.perfumes,
        )
        right = (
            aunt_sue.children,
            aunt_sue.cats,
            aunt_sue.samoyeds,
            aunt_sue.pomeranians,
            aunt_sue.akitas,
            aunt_sue.vizslas,
            aunt_sue.goldfish,
            aunt_sue.trees,
            aunt_sue.cars,
            aunt_sue.perfumes,
        )

        return all(r is None or l == r for l, r in zip(left, right))


def main() -> int:
    mfcsam = MFCSAM(
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
    for i, aunt_sue in enumerate(parse_line()):
        # print(f"{aunt_sue = }")
        if mfcsam.test(aunt_sue):
            print(f"Found Aunt Sue! {i+1}: {aunt_sue}")
    return 0


from typing import Iterator


def parse_line() -> Iterator[AuntSue]:
    with open("input") as handle:
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
