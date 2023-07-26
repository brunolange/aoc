from dataclasses import dataclass
from enum import Enum, auto
from typing import Iterator, assert_never

OptionalInt = int | None


@dataclass
class AuntSue:
    name: str
    children: OptionalInt
    cats: OptionalInt
    samoyeds: OptionalInt
    pomeranians: OptionalInt
    akitas: OptionalInt
    vizslas: OptionalInt
    goldfish: OptionalInt
    trees: OptionalInt
    cars: OptionalInt
    perfumes: OptionalInt


class Mode(Enum):
    EQUAL = auto()
    UNDERSHOOT = auto()
    OVERSHOOT = auto()


@dataclass
class Attribute:
    value: int
    mode: Mode


@dataclass
class Machine:
    children: Attribute
    cats: Attribute
    samoyeds: Attribute
    pomeranians: Attribute
    akitas: Attribute
    vizslas: Attribute
    goldfish: Attribute
    trees: Attribute
    cars: Attribute
    perfumes: Attribute

    @staticmethod
    def match(reading: Attribute, value: int) -> bool:
        match reading.mode:
            case Mode.EQUAL:
                return reading.value == value
            case Mode.OVERSHOOT:
                return reading.value > value
            case Mode.UNDERSHOOT:
                return reading.value < value
            case other:
                assert_never(other)

    def test(self, aunt_sue: AuntSue) -> bool:
        return all(
            value is None or Machine.match(reading, value)
            for reading, value in (
                (self.children, aunt_sue.children),
                (self.cats, aunt_sue.cats),
                (self.samoyeds, aunt_sue.samoyeds),
                (self.pomeranians, aunt_sue.pomeranians),
                (self.akitas, aunt_sue.akitas),
                (self.vizslas, aunt_sue.vizslas),
                (self.goldfish, aunt_sue.goldfish),
                (self.trees, aunt_sue.trees),
                (self.cars, aunt_sue.cars),
                (self.perfumes, aunt_sue.perfumes),
            )
        )


def main() -> int:
    mfcsam = Machine(
        children=Attribute(value=3, mode=Mode.EQUAL),
        cats=Attribute(value=7, mode=Mode.UNDERSHOOT),
        samoyeds=Attribute(value=2, mode=Mode.EQUAL),
        pomeranians=Attribute(value=3, mode=Mode.OVERSHOOT),
        akitas=Attribute(value=0, mode=Mode.EQUAL),
        vizslas=Attribute(value=0, mode=Mode.EQUAL),
        goldfish=Attribute(value=5, mode=Mode.OVERSHOOT),
        trees=Attribute(value=3, mode=Mode.UNDERSHOOT),
        cars=Attribute(value=2, mode=Mode.EQUAL),
        perfumes=Attribute(value=1, mode=Mode.EQUAL),
    )

    match: AuntSue | None = None
    for aunt_sue in parse_line():
        if mfcsam.test(aunt_sue):
            match = aunt_sue

    if not match:
        print(f"Found no Aunt Sue...")
        return 1

    print(f"Found Aunt Sue: {match}")
    return 0


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
