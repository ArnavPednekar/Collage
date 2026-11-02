def simple_hill_climbing(values, neighbours, start):
    current = start
    step = 1

    while True:
        current_value = values[current]
        neighbor_list = neighbours.get(current, [])
        print(f"\nStep {step}: Current State = {current}({current_value})")
        print(f"Neighbours examined: {', '.join([f'{n}({values[n]})' for n in neighbor_list]) if neighbor_list else 'None'}")

        moved = False
        for neighbour in neighbor_list:
            # Maximization problem: move to first better neighbour found
            if values[neighbour] > current_value:
                print(f"Move to {neighbour}({values[neighbour]}) as {values[neighbour]} > {current_value}")
                current = neighbour
                moved = True
                break
            else:
                print(f"{neighbour}({values[neighbour]}) is not better than {current}({current_value}), checking next...")

        if not moved:
            print(f"No better neighbour found. Stopping at {current}({current_value})")
            break

        step += 1

    return current


if __name__ == "__main__":
    # Heuristic / evaluation values for each state
    values = {
        'A': 5,
        'B': 8,
        'C': 12,
        'D': 6,
        'E': 10,
        'F': 15,
        'G': 9
    }

    # Neighbours in fixed order to demonstrate Simple Hill Climbing behaviour.
    # A -> B, C : B is examined first, so algorithm moves to B(8)
    #             even though C(12) is better (first-better, not best).
    # B -> D, E : moves to E(10)
    # E -> G    : G(9) is worse, so stops at E (local maximum).
    # C -> F    : F(15) is the global maximum but is never reached.
    neighbours = {
        'A': ['B', 'C'],
        'B': ['D', 'E'],
        'C': ['F'],
        'D': [],
        'E': ['G'],
        'F': [],
        'G': []
    }

    start = 'A'
    print("Simple Hill Climbing (Maximization)")
    print(f"Starting state: {start}({values[start]})")

    solution = simple_hill_climbing(values, neighbours, start)

    print("\nFinal solution:", f"{solution}({values[solution]})")
    print("Note: E(10) is a Local Maximum.")
    print("F(15) is the Global Maximum (via C -> F) which Simple Hill Climbing misses.")
