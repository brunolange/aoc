import sys

def lines():
    if len(sys.argv) > 1:
        with open(sys.argv[1]) as handle:
            contents = handle.readlines()
    else:
        contents = sys.stdin

    return (line.strip() for line in contents)
    

def main() -> int:
    for line in lines():
        print(f"{line = }")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
