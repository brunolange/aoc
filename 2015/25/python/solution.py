from itertools import count


def code(row: int, column: int) -> int:
    assert row > 0 and column > 0

    curr = 20_151_125

    max_row = row + column - 1
    n = max_row * (max_row-1) // 2
    steps = n + column - 1

    for _ in range(steps):
        curr *= 252_533
        curr %= 33_554_393

    return curr

def main() -> int:
    print(code(3_010, 3_019))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
