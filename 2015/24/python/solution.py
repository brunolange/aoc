from itertools import combinations
import math
import sys
import tqdm

def find_partitions(weights, k):
    total_weight = sum(weights)

    assert total_weight % k == 0

    target_weight = total_weight // k

    # Generate all combinations of weights
    for i in tqdm.tqdm(range(1, len(weights))):
        for combo in combinations(weights, i):
            # Check if the sum of the combination is equal to the target weight
            if sum(combo) == target_weight:
                remaining_weights = set(weights) - set(combo)

                # Generate all combinations for the remaining partitions
                found_partitions = [list(combo)]
                for _ in range(k - 1):
                    for j in range(1, len(remaining_weights)):
                        for combo_j in combinations(remaining_weights, j):
                            # Check if the sum of the combination is equal to the target weight
                            if sum(combo_j) == target_weight:
                                # Add the current combination to the found partitions
                                found_partitions.append(list(combo_j))
                                remaining_weights -= set(combo_j)
                                break

                # If all partitions are found, yield them
                if len(found_partitions) == k:
                    yield tuple(found_partitions)

def score(partition: list[list[int]]) -> tuple[int, int]:
    group1, *_ = partition
    quantum_entanglement = math.prod(group1)
    return len(group1), quantum_entanglement

def read_weights(args) -> list[int]:
    if not args:
        return [1, 2, 3, 4, 5, 7, 8, 9, 10, 11]
    
    source = args[0]
    
    if source == "-":
        return list(map(int, sys.stdin.readlines()))
    
    with open(source) as handle:
        return list(map(int, handle.readlines()))
    
def main() -> int:
    if len(sys.argv) < 2:
        print("ERROR: need number of partitions", file=sys.stderr)
        return 1

    number_of_partitions, *args = sys.argv[1:]

    weights = read_weights(args)
    partitions = find_partitions(weights, int(number_of_partitions))

    try:
        best_partition = min(partitions, key=score)
    except ValueError:
        print("ERROR: There is no solution", file=sys.stderr)
        return 1

    print(f"{best_partition = }")
    _, quantum_entanglement = score(best_partition)
    print(f"Best quantum entanglement = {quantum_entanglement}")


if __name__ == "__main__":
    raise SystemExit(main())
