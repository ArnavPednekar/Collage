def steepest_ascent_hill_climbing(values, neighbours, start):
    current = start
    step = 1

    while True:
        current_value = values[current]
        neighbour_list = neighbours.get(current, [])

        print(f"\nStep {step}: Current State = {current}({current_value})")

        if neighbour_list:
            print("Neighbours examined:",
                  ", ".join(f"{n}({values[n]})" for n in neighbour_list))
        else:
            print("Neighbours examined: None")

        best_neighbour = None
        best_value = current_value

        for neighbour in neighbour_list:
            if values[neighbour] > best_value:
                best_value = values[neighbour]
                best_neighbour = neighbour

        if best_neighbour is None:
            print(f"No better neighbour found. Stopping at {current}({current_value})")
            break

        print(f"Best neighbour: {best_neighbour}({values[best_neighbour]})")
        print(f"Move to {best_neighbour}({values[best_neighbour]})")
        current = best_neighbour
        step += 1

    return current


if __name__ == "__main__":
    values = {
        'A': 5,
        'B': 8,
        'C': 12,
        'D': 6,
        'E': 10,
        'F': 15,
        'G': 9
    }

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

    print("Steepest-Ascent Hill Climbing (Maximization)")
    print(f"Starting state: {start}({values[start]})")

    solution = steepest_ascent_hill_climbing(values, neighbours, start)

    print("\nFinal solution:", f"{solution}({values[solution]})")
