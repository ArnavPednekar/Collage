from collections import deque


GOAL_STATE = (1, 2, 3, 4, 5, 6, 7, 8, 0)


def print_state(state):
    for row in range(3):
        start = row * 3
        print(state[start], state[start + 1], state[start + 2])


def get_neighbours(state):
    index = state.index(0)
    row, col = divmod(index, 3)

    moves = [(-1, 0), (1, 0), (0, -1), (0, 1)]

    for dr, dc in moves:
        new_row, new_col = row + dr, col + dc

        if 0 <= new_row < 3 and 0 <= new_col < 3:
            new_index = new_row * 3 + new_col
            new_state = list(state)
            new_state[index], new_state[new_index] = (
                new_state[new_index],
                new_state[index],
            )
            yield tuple(new_state)


def solve_puzzle(initial_state, goal_state):
    queue = deque([(initial_state, [])])
    visited = {initial_state}

    while queue:
        state, path = queue.popleft()

        if state == goal_state:
            return path + [state]

        for neighbour in get_neighbours(state):
            if neighbour not in visited:
                visited.add(neighbour)
                queue.append((neighbour, path + [state]))

    return None


if __name__ == "__main__":
    initial_state = (1, 2, 3, 4, 0, 6, 7, 5, 8)

    print("Eight Puzzle Problem")
    print("\nInitial State:")
    print_state(initial_state)

    print("\nGoal State:")
    print_state(GOAL_STATE)

    solution = solve_puzzle(initial_state, GOAL_STATE)

    if solution:
        print(f"\nSolution found in {len(solution) - 1} moves:")
        for step, state in enumerate(solution):
            print(f"\nStep {step}:")
            print_state(state)
    else:
        print("\nNo solution found.")
