from __future__ import annotations

import argparse
from dataclasses import dataclass
from typing import Iterator


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

    def volume(self) -> int:
        return self.height * self.width * self.length

    def sorted_dimensions(self) -> tuple[int, int, int]:
        a, b, c = sorted([self.height, self.width, self.length])
        return a, b, c

    @staticmethod
    def from_line(line: str) -> Box:
        height, width, length = map(int, line.split("x"))
        return Box(height=height, width=width, length=length)


@dataclass
class Order:
    wrapping_paper: int
    ribbon: int

    @staticmethod
    def from_box(box: Box) -> Order:
        return Order(
            wrapping_paper=paper_needed(box),
            ribbon=ribbon_needed(box),
        )

    def __add__(self, other: Order) -> Order:
        return Order(
            wrapping_paper=self.wrapping_paper + other.wrapping_paper,
            ribbon=self.ribbon + other.ribbon,
        )


def read(path: str) -> Iterator[Box]:
    with open(path) as handle:
        yield from map(Box.from_line, handle.readlines())


def paper_needed(box: Box) -> int:
    smallest, second_smallest, _ = box.sorted_dimensions()
    return box.area() + smallest * second_smallest


def ribbon_needed(box: Box) -> int:
    smallest, second_smallest, _ = box.sorted_dimensions()
    return 2 * (smallest + second_smallest) + box.volume()


def main(args) -> int:
    from functools import reduce
    from operator import add

    total = reduce(add, map(Order.from_box, read(args.input)))
    print(total)
    # order = sum(map(Order.from_box, read(args.input)))
    # print(f"{order = }")
    return 0


if __name__ == "__main__":
    args = argparse.ArgumentParser()
    args.add_argument("input")
    raise SystemExit(main(args.parse_args()))
