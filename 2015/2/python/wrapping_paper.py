from typing import Iterator
from dataclasses import dataclass
import argparse


@dataclass
class Box:
    height: int
    width: int
    length: int

    def area(self) -> int:
        return 2 * (
            self.height * self.width
            + self.height * self.length
            + self.width * self.length
        )


def line_to_box(line: str) -> Box:
    height, width, length = map(int, line.split("x"))
    return Box(height=height, width=width, length=length)


def read(path: str) -> Iterator[Box]:
    with open(path) as handle:
        yield from map(line_to_box, handle.readlines())


def paper_needed(box: Box) -> int:
    smallest, second_smallest, _ = sorted([box.height, box.width, box.length])
    return box.area() + smallest * second_smallest


def main(args) -> int:
    order = sum(map(paper_needed, read(args.input)))
    print(f"{order = }")
    return 0


if __name__ == "__main__":
    args = argparse.ArgumentParser()
    args.add_argument("input")
    raise SystemExit(main(args.parse_args()))
