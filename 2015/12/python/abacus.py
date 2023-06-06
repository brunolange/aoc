from __future__ import annotations

import json
import sys
import builtins
from typing import Iterator

Json = None | int | float | bool | str | list["Json"] | dict[str,"Json"]

def read() -> Json:
    if len(sys.argv) > 1:
        with open(sys.argv[1]) as handle:
            return json.load(handle)

    return json.loads(sys.stdin.read())
    
def extract_numbers(value: Json) -> Iterator[int|float]:
    match type(value):
        case builtins.int | builtins.float:
            yield value
        case builtins.list:
            for v in value:
                yield from extract_numbers(v)
        case builtins.dict:
            for v in value.values():
                yield from extract_numbers(v)
        case _:
            pass

def main() -> int:
    value = read()
    print(sum(extract_numbers(value)))
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
