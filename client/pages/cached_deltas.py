from components import algorithm_comparison_page
from utils import Algorithm
from pages.local import ALGORITHMS as ls_algorithms, INITIAL_RANDOM, NODES, EDGES

NODES += r"""
fn check_intra_validity(solution, move):
    if (move.prev1, move.node1, move.next1) in solution and (move.prev2, move.node2, move.next2) in solution:
        return true
    return false
    
fn add_all_possible_improving_moves_intra(lm, solution, move):
    add_inter_moves_from_node(lm, solution, move.prev1)
    add_inter_moves_from_node(lm, solution, move.node1)
    add_inter_moves_from_node(lm, solution, move.next1)
    add_inter_moves_from_node(lm, solution, move.prev2)
    add_inter_moves_from_node(lm, solution, move.node2)
    add_inter_moves_from_node(lm, solution, move.next2)
    
    add_intra_moves_from_node(lm, solution, move.prev1)
    add_intra_moves_from_node(lm, solution, move.node1)
    add_intra_moves_from_node(lm, solution, move.next1)
    add_intra_moves_from_node(lm, solution, move.prev2)
    add_intra_moves_from_node(lm, solution, move.node2)
    add_intra_moves_from_node(lm, solution, move.next2)
"""
EDGES += r"""
fn check_intra_validity(solution, move):
    if (move.edge1 in solution and move.edge2 in solution)
        or (inverted(move.edge1) in solution and inverted(move.edge2) in solution):
            return true
    else if (move.edge1 in solution and inverted(move.edge2) in solution))
        or (inverted(move.edge1) in solution and move.edge2 in solution):
            store_move(move)
        return false
    return false

fn add_all_possible_improving_moves_intra(lm, solution, move):
    add_inter_moves_from_node(lm, solution, move.edge1_first_node)
    add_inter_moves_from_node(lm, solution, move.edge1_second_node)
    add_inter_moves_from_node(lm, solution, move.edge2_first_node)
    add_inter_moves_from_node(lm, solution, move.edge2_second_node)
    
    add_intra_moves_from_node(lm, solution, move.edge1_first_node)
    add_intra_moves_from_node(lm, solution, move.edge1_second_node)
    add_intra_moves_from_node(lm, solution, move.edge2_first_node)
    add_intra_moves_from_node(lm, solution, move.edge2_second_node)
    
"""

LS_CACHED_DELTAS = r"""
fn inter(i, j):
    return -Distance[i-1][i] - Distance[i][i+1] - Cost[i]
        + Distance[i-1][j] + Distance[j][i+1] + Cost[j]
        
fn inter_change(solution, i, j):
    solution = copy(solution)
    solution.replace(i, j)
    return solution
    
fn check_inter_validity(solution, move):
    if (move.prev, move.node, move.next) in solution and move.outside_node not in solution:
        and move.outside_node not in solution:
        return true
    return false

fn apply_move(solution, move):
    if move.type == "inter":
        new_solution = inter_change(solution, move.node, move.outside_node)
    else:
        new_solution = intra_change(solution, move.node1, move.node2)

fn valid(move, solution):
    if move.type == "inter":
        return check_inter_validity(solution, move)
    else:
        return check_intra_validity(solution, move)

fn add_new_improving_moves(lm, solution, move):
    if move.type == "inter":
        add_all_possible_improving_moves_inter(lm, solution, move)
    else:
        add_all_possible_improving_moves_intra(lm, solution, move)
        
fn add_all_possible_improving_inter_moves(lm, solution, move):
    add_inter_moves_from_node(lm, solution, move.prev)
    add_inter_moves_from_node(lm, solution, move.node)
    add_inter_moves_from_node(lm, solution, move.next)
    add_intra_moves_from_node(lm, solution, move.prev)
    add_intra_moves_from_node(lm, solution, move.node)
    add_intra_moves_from_node(lm, solution, move.next)
    
lm = MinPriorityQueue()
add_all_possible_improving_moves(lm, current_solution)

while True:
    best_move = null
    while not lm.is_empty():
        move = lm.pop()
        if valid(move, current_solution):
            new_solution, delta = apply_move(current_solution, move)
            best_move = move
            break
            
    if best_move == null:
        break
    recover_stored_moves(lm)
    add_new_improving_moves(lm, new_solution, best_move)
    current_solution = new_solution
    
return current_solution
        
"""

CONCLUSIONS = r"""
- Scores are almost identical to those of the steepest local search algorithm with negligible deviations.
- Running times are vastly improved, resulting in a speedup of around 3x.
"""

ALGORITHMS = [
    Algorithm(
        "ls_cached_deltas_edges",
        "ls_cached_deltas_edges",
        INITIAL_RANDOM + EDGES + LS_CACHED_DELTAS
    ),
    Algorithm(
        "ls_cached_deltas_nodes",
        "ls_cached_deltas_nodes",
        INITIAL_RANDOM + NODES + LS_CACHED_DELTAS
    )
]
if __name__ == "__main__":
    algorithm_comparison_page(
        ALGORITHMS, "Cached Deltas", [ls_algorithms[4], ls_algorithms[6]], conclusions=CONCLUSIONS
    )
