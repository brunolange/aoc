from dataclasses import dataclass
from enum import Enum, auto
from typing import Iterator, assert_never


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


class MFCSAMPropertyMode(Enum):
    EQUAL = auto()
    GREATER_THAN = auto()
    LESS_THAN = auto()


@dataclass
class MFCSAMReading:
    value: int
    mode: MFCSAMPropertyMode


@dataclass
class MFCSAM:
    children: MFCSAMReading
    cats: MFCSAMReading
    samoyeds: MFCSAMReading
    pomeranians: MFCSAMReading
    akitas: MFCSAMReading
    vizslas: MFCSAMReading
    goldfish: MFCSAMReading
    trees: MFCSAMReading
    cars: MFCSAMReading
    perfumes: MFCSAMReading

    @staticmethod
    def match(reading: MFCSAMReading, value: int) -> bool:
        match reading.mode:
            case MFCSAMPropertyMode.EQUAL:
                return reading.value == value
            case MFCSAMPropertyMode.LESS_THAN:
                return value < reading.value
            case MFCSAMPropertyMode.GREATER_THAN:
                return value > reading.value
            case other:
                assert_never(other)

    def test(self, aunt_sue: AuntSue) -> bool:
        return all(
            value is None or MFCSAM.match(reading, value)
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
    mfcsam = MFCSAM(
        children=MFCSAMReading(value=3, mode=MFCSAMPropertyMode.EQUAL),
        cats=MFCSAMReading(value=7, mode=MFCSAMPropertyMode.GREATER_THAN),
        samoyeds=MFCSAMReading(value=2, mode=MFCSAMPropertyMode.EQUAL),
        pomeranians=MFCSAMReading(value=3, mode=MFCSAMPropertyMode.LESS_THAN),
        akitas=MFCSAMReading(value=0, mode=MFCSAMPropertyMode.EQUAL),
        vizslas=MFCSAMReading(value=0, mode=MFCSAMPropertyMode.EQUAL),
        goldfish=MFCSAMReading(value=5, mode=MFCSAMPropertyMode.LESS_THAN),
        trees=MFCSAMReading(value=3, mode=MFCSAMPropertyMode.GREATER_THAN),
        cars=MFCSAMReading(value=2, mode=MFCSAMPropertyMode.EQUAL),
        perfumes=MFCSAMReading(value=1, mode=MFCSAMPropertyMode.EQUAL),
    )
    for i, aunt_sue in enumerate(parse_line()):
        # print(f"{aunt_sue = }")
        if mfcsam.test(aunt_sue):
            print(f"Found Aunt Sue! {i+1}: {aunt_sue}")
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
