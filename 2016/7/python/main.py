from __future__ import annotations
from dataclasses import dataclass
import sys


@dataclass
class IPPart:
    seq: str
    hypernet: str


@dataclass
class IP:
    parts: list[IPPart]
    extra: str | None

    @staticmethod
    def parse(s: str) -> IP:
        pairs_and_rest = [p.split("[") for p in s.split("]")]
        if len(pairs_and_rest[-1]) != 2:
            pairs_and_rest[-1].append(None)

        return IP(
            parts=[
                IPPart(seq=seq, hypernet=hypernet)
                for seq, hypernet in pairs_and_rest[:-1]
            ],
            extra=pairs_and_rest[-1][0],
        )


def supports_tls(ip):
    return (
        ip.parts
        and (
            any(is_abba(seq) for seq in (part.seq for part in ip.parts))
            or is_abba(ip.extra)
            if ip.extra
            else False
        )
        and all(not is_abba(part.hypernet) for part in ip.parts)
    )


def is_abba(s: str) -> bool:
    try:
        a, b, c, d, *rest = s
    except ValueError:
        return False

    return a != b and a == d and b == c or is_abba(s[1:])


def count(xs) -> int:
    return sum(1 for _ in xs)


def main() -> int:
    total = count(
        filter(supports_tls, map(lambda line: IP.parse(line.strip()), sys.stdin))
    )

    print(total)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
