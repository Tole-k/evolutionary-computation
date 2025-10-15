from components import algorithm_comparison_page
from utils import Algorithm

NN_TO_ANY_2_REGRET_PSEUDOCODE = r""""""
GREEDY_CYCLE_2_REGRET_PSEUDOCODE = r""""""

CONCLUSIONS = r""""""


ALGORITHMS = [
    Algorithm("NN to any point 2 regret", "nn_to_any_2_regret", NN_TO_ANY_2_REGRET_PSEUDOCODE),
    Algorithm("NN to any point weighted 2 regret", "nn_to_any_weighted_2_regret", NN_TO_ANY_2_REGRET_PSEUDOCODE),
    Algorithm("Greedy Cycle 2 regret", "greedy_cycle_2_regret", GREEDY_CYCLE_2_REGRET_PSEUDOCODE),
    Algorithm("Greedy Cycle weighted 2 regret", "greedy_cycle_weighted_2_regret", GREEDY_CYCLE_2_REGRET_PSEUDOCODE),
]


if __name__ == "__main__":
    algorithm_comparison_page(ALGORITHMS, "Regret algorithms", conclusions=CONCLUSIONS)
