from collections import deque


def solve_water_jug():
    initial_state = (0, 0)
    queue = deque([(initial_state, [initial_state])])
    visited = set([initial_state])
    
    while queue:
        (x, y), path = queue.popleft()
        
        if x == 2:
            return path
            
        next_states = []
        
        next_states.append((4, y))
        next_states.append((x, 3))
        next_states.append((0, y))
        next_states.append((x, 0))
        
        transfer_to_3 = min(x, 3 - y)
        next_states.append((x - transfer_to_3, y + transfer_to_3))
        
        transfer_to_4 = min(y, 4 - x)
        next_states.append((x + transfer_to_4, y - transfer_to_4))
        
        for state in next_states:
            if state not in visited:
                visited.add(state)
                queue.append((state, path + [state]))
                
    return None

if __name__ == "__main__":
    solution = solve_water_jug()
    if solution:
        print("Solution path:")
        for step in solution:
            print(step)
    else:
        print("No solution found.")
