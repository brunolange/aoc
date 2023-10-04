from itertools import count


def code(row: int, column: int) -> int:
    assert row > 0 and column > 0

    curr = 20_151_125

    for i in count(start=1, step=1):
        for j in range(1, i + 1):
            r = i - j + 1
            c = j

            if (r, c) == (row, column):
                return curr

            curr *= 252_533
            curr %= 33_554_393


def main() -> int:
    print(code(3_010, 3_019))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
