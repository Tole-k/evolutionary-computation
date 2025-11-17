use crate::local_search_base::{generate_neighborhood, inter, intra};
use crate::utils::DataPoint;
use crate::utils::generate_random_solution;
use ndarray::Array2;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

trait Move {
    fn delta(&self) -> f64;
}
#[derive(Clone)]
struct IntraEdgeMove {
    delta: f64,
    edge1_first_node: usize,
    edge1_second_node: usize,
    edge2_first_node: usize,
    edge2_second_node: usize,
}
#[derive(Clone)]
struct IntraNodeMove {
    delta: f64,
    before_node1: usize,
    node1: usize,
    after_node1: usize,
    before_node2: usize,
    node2: usize,
    after_node2: usize,
}

#[derive(Clone)]
struct InterNodeMove {
    delta: f64,
    before_node1: usize,
    node1: usize,
    after_node1: usize,
    outside_node: usize,
}
#[derive(Clone)]
enum MoveType {
    IntraEdge(IntraEdgeMove),
    IntraNode(IntraNodeMove),
    InterNode(InterNodeMove),
}
impl Eq for MoveType {}

impl PartialEq for MoveType {
    fn eq(&self, other: &Self) -> bool {
        self.delta() == other.delta()
    }
}

impl PartialOrd for MoveType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.delta().partial_cmp(&other.delta())
    }
}
impl Ord for MoveType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.delta().total_cmp(&other.delta())
    }
}

impl Move for MoveType {
    fn delta(&self) -> f64 {
        match self {
            MoveType::IntraEdge(m) => m.delta,
            MoveType::IntraNode(m) => m.delta,
            MoveType::InterNode(m) => m.delta,
        }
    }
}
fn intra_edges(
    solution: &Vec<usize>,
    a: usize,
    prev_a: usize,
    b: usize,
    next_b: usize,
    distance_matrix: &Array2<f64>,
) -> f64 {
    return -distance_matrix[[solution[prev_a], solution[a]]]
        - distance_matrix[[solution[b], solution[next_b]]]
        + distance_matrix[[solution[prev_a], solution[b]]]
        + distance_matrix[[solution[a], solution[next_b]]];
}
fn apply_if_valid(
    current_solution: &Vec<usize>,
    stored_moves: &mut Vec<MoveType>,
    mv: MoveType,
) -> Option<(Vec<usize>, MoveType)> {
    match mv {
        MoveType::IntraEdge(mv) => match case_intra_edges(current_solution, mv, stored_moves) {
            None => return None,
            Some(value) => return Some((value.0, MoveType::IntraEdge(value.1))),
        },
        MoveType::IntraNode(mv) => match case_intra_nodes(current_solution, mv) {
            None => return None,
            Some(value) => return Some((value.0, MoveType::IntraNode(value.1))),
        },
        MoveType::InterNode(mv) => match case_inter_nodes(current_solution, mv) {
            None => return None,
            Some(value) => return Some((value.0, MoveType::InterNode(value.1))),
        },
    }
}

fn case_inter_nodes(
    current_solution: &Vec<usize>,
    mv: InterNodeMove,
) -> Option<(Vec<usize>, InterNodeMove)> {
    let InterNodeMove {
        delta,
        before_node1,
        node1,
        after_node1,
        outside_node,
    } = mv;
    let node1_position: usize;
    let before_node1_position: usize;
    let after_node1_position: usize;
    let n = current_solution.len();
    match current_solution.iter().position(|x| *x == node1) {
        None => {
            return None;
        }
        Some(position) => {
            node1_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == before_node1) {
        None => {
            return None;
        }
        Some(position) => {
            before_node1_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == after_node1) {
        None => {
            return None;
        }
        Some(position) => {
            after_node1_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == outside_node) {
        None => {}
        Some(_position) => {
            return None;
        }
    }
    if (before_node1_position + 1) % n == node1_position
        && node1_position == (after_node1_position + n - 1) % n
    {
        let mut new_solution = current_solution.clone();
        new_solution[node1_position] = outside_node;
        return Some((
            new_solution,
            InterNodeMove {
                delta,
                before_node1: before_node1_position,
                node1: node1_position,
                after_node1: after_node1_position,
                outside_node,
            },
        ));
    } else {
        return None;
    }
}

fn case_intra_nodes(
    current_solution: &Vec<usize>,
    mv: IntraNodeMove,
) -> Option<(Vec<usize>, IntraNodeMove)> {
    let IntraNodeMove {
        delta,
        before_node1,
        node1,
        after_node1,
        before_node2,
        node2,
        after_node2,
    } = mv;
    let node1_position: usize;
    let before_node1_position: usize;
    let after_node1_position: usize;
    let before_node2_position: usize;
    let node2_position: usize;
    let after_node2_position: usize;
    let n = current_solution.len();
    match current_solution.iter().position(|x| *x == node1) {
        None => {
            return None;
        }
        Some(position) => {
            node1_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == before_node1) {
        None => {
            return None;
        }
        Some(position) => {
            before_node1_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == after_node1) {
        None => {
            return None;
        }
        Some(position) => {
            after_node1_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == node2) {
        None => {
            return None;
        }
        Some(position) => {
            node2_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == before_node2) {
        None => {
            return None;
        }
        Some(position) => {
            before_node2_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == after_node2) {
        None => {
            return None;
        }
        Some(position) => {
            after_node2_position = position;
        }
    }
    if (before_node1_position + 1) % n == node1_position
        && node1_position == (after_node1_position + n - 1) % n
        && (before_node2_position + 1) % n == node2_position
        && node2_position == (after_node2_position + n - 1) % n
    {
        let mut new_solution = current_solution.clone();
        new_solution.swap(node1_position, node2_position);
        return Some((
            new_solution,
            IntraNodeMove {
                delta,
                before_node1: before_node1_position,
                node1: node1_position,
                after_node1: after_node1_position,
                before_node2: before_node2_position,
                node2: node2_position,
                after_node2: after_node2_position,
            },
        ));
    } else {
        return None;
    }
}

fn case_intra_edges(
    current_solution: &Vec<usize>,
    mv: IntraEdgeMove,
    stored_moves: &mut Vec<MoveType>,
) -> Option<(Vec<usize>, IntraEdgeMove)> {
    let IntraEdgeMove {
        delta,
        edge1_first_node,
        edge1_second_node,
        edge2_first_node,
        edge2_second_node,
    } = mv;
    let edge1_first_node_position: usize;
    let edge1_second_node_position: usize;
    let edge2_first_node_position: usize;
    let edge2_second_node_position: usize;
    let n = current_solution.len();
    match current_solution.iter().position(|x| *x == edge1_first_node) {
        None => return None,
        Some(position) => {
            edge1_first_node_position = position;
        }
    }
    match current_solution
        .iter()
        .position(|x| *x == edge1_second_node)
    {
        None => return None,
        Some(position) => {
            edge1_second_node_position = position;
        }
    }
    match current_solution.iter().position(|x| *x == edge2_first_node) {
        None => return None,
        Some(position) => {
            edge2_first_node_position = position;
        }
    }
    match current_solution
        .iter()
        .position(|x| *x == edge2_second_node)
    {
        None => return None,
        Some(position) => {
            edge2_second_node_position = position;
        }
    }
    if ((edge1_first_node_position + 1) % n == edge1_second_node_position
        && (edge2_first_node_position + 1) % n == edge2_second_node_position)
        || ((edge1_first_node_position + n - 1) % n == edge1_second_node_position
            && (edge2_first_node_position + n - 1) % n == edge2_second_node_position)
    {
        let mut new_solution = current_solution.clone();
        let sub_slice = if edge1_second_node_position < edge2_first_node_position {
            &mut new_solution[edge1_second_node_position..=edge2_first_node_position]
        } else {
            &mut new_solution[edge2_first_node_position..=edge1_second_node_position]
        };
        sub_slice.reverse();
        return Some((
            new_solution,
            IntraEdgeMove {
                delta,
                edge1_first_node: edge1_first_node_position,
                edge1_second_node: edge1_second_node_position,
                edge2_first_node: edge2_first_node_position,
                edge2_second_node: edge2_second_node_position,
            },
        ));
    } else if ((edge1_first_node_position + 1) % n == edge1_second_node_position
        && (edge2_first_node_position - 1) % n == edge2_second_node_position)
        || ((edge1_first_node_position + n - 1) % n == edge1_second_node_position
            && (edge2_first_node_position + n + 1) % n == edge2_second_node_position)
    {
        stored_moves.push(MoveType::IntraEdge(mv));
        return None;
    } else {
        return None;
    }
}

fn add_all_moves(
    data: &Vec<DataPoint>,
    distance_matrix: &Array2<f64>,
    lm: &mut BinaryHeap<Reverse<MoveType>>,
    current_solution: &Vec<usize>,
    change_edges: bool,
) {
    let moves = generate_neighborhood(current_solution, data);
    for (move_type, i, j) in moves {
        if move_type == 0 {
            if change_edges {
                add_intra_edge_move(distance_matrix, lm, current_solution, i, j);
            } else {
                add_intra_node_move(distance_matrix, lm, current_solution, i, j);
            }
        } else if move_type == 1 {
            add_inter_node_move(distance_matrix, lm, current_solution, i, j, data);
        }
    }
}

fn add_intra_edge_move(
    distance_matrix: &Array2<f64>,
    lm: &mut BinaryHeap<Reverse<MoveType>>,
    current_solution: &Vec<usize>,
    i: usize,
    j: usize,
) {
    let n = current_solution.len();
    let (mut a, mut b) = (i, j);
    if (j + 1) % n == i {
        a = j;
        b = i;
    }
    let prev_a = (a + n - 1) % n;
    let next_b = (b + 1) % n;
    let delta = intra_edges(current_solution, a, prev_a, b, next_b, distance_matrix);
    if delta >= 0. {
        return;
    }
    let node1 = current_solution[a];
    let prev_node1 = current_solution[prev_a];
    let node2 = current_solution[b];
    let node2_next = current_solution[next_b];
    let intra_edge_move = IntraEdgeMove {
        delta,
        edge1_first_node: prev_node1,
        edge1_second_node: node1,
        edge2_first_node: node2,
        edge2_second_node: node2_next,
    };
    lm.push(Reverse(MoveType::IntraEdge(intra_edge_move)));
    let delta2 = intra_edges(current_solution, prev_a, a, next_b, b, distance_matrix);
    let node1 = current_solution[a];
    let prev_node1 = current_solution[prev_a];
    let node2 = current_solution[b];
    let node2_next = current_solution[next_b];
    let intra_edge_move = IntraEdgeMove {
        delta: delta2,
        edge1_first_node: node1,
        edge1_second_node: prev_node1,
        edge2_first_node: node2_next,
        edge2_second_node: node2,
    };
    lm.push(Reverse(MoveType::IntraEdge(intra_edge_move)));
}
fn add_intra_node_move(
    distance_matrix: &Array2<f64>,
    lm: &mut BinaryHeap<Reverse<MoveType>>,
    current_solution: &Vec<usize>,
    i: usize,
    j: usize,
) {
    let n = current_solution.len();
    let delta = intra(current_solution, i, j, distance_matrix);
    if delta >= 0. {
        return;
    }
    let node1 = current_solution[i];
    let prev_node1 = current_solution[(i - 1 + n) % n];
    let node1_next = current_solution[(i + 1) % n];
    let node2 = current_solution[j];
    let prev_node2 = current_solution[(j - 1 + n) % n];
    let node2_next = current_solution[(j + 1) % n];
    let intra_node_move = IntraNodeMove {
        delta,
        before_node1: prev_node1,
        node1,
        after_node1: node1_next,
        before_node2: prev_node2,
        node2,
        after_node2: node2_next,
    };
    lm.push(Reverse(MoveType::IntraNode(intra_node_move)));
}
fn add_inter_node_move(
    distance_matrix: &Array2<f64>,
    lm: &mut BinaryHeap<Reverse<MoveType>>,
    current_solution: &Vec<usize>,
    i: usize,
    j: usize,
    data: &Vec<DataPoint>,
) {
    let n = current_solution.len();
    let delta = inter(current_solution, i, j, distance_matrix, data);
    if delta >= 0. {
        return;
    }
    let node1 = current_solution[i];
    let prev_node1 = current_solution[(i - 1 + n) % n];
    let node1_next = current_solution[(i + 1) % n];
    let inter_node_move = InterNodeMove {
        delta,
        before_node1: prev_node1,
        node1,
        after_node1: node1_next,
        outside_node: j,
    };
    lm.push(Reverse(MoveType::InterNode(inter_node_move)));
}

fn recover_stored_moves(lm: &mut BinaryHeap<Reverse<MoveType>>, stored_moves: &mut Vec<MoveType>) {
    for mv in &mut *stored_moves {
        lm.push(Reverse(mv.clone()));
    }
    stored_moves.clear();
}
fn add_inter_node_moves(
    data: &Vec<DataPoint>,
    distance_matrix: &Array2<f64>,
    lm: &mut BinaryHeap<Reverse<MoveType>>,
    solution: &Vec<usize>,
    i: usize,
) {
    let hash_solution: HashSet<usize> = HashSet::from_iter(solution.clone());
    let m = data.len();
    let all: HashSet<usize> = (0..m).collect();
    let difference: Vec<usize> = all.difference(&hash_solution).cloned().collect();

    for j in &difference {
        add_inter_node_move(distance_matrix, lm, solution, i, *j, data);
    }
}
fn add_intra_edge_moves(
    distance_matrix: &Array2<f64>,
    lm: &mut BinaryHeap<Reverse<MoveType>>,
    solution: &Vec<usize>,
    i: usize,
) {
    for j in 0..solution.len() {
        if i == j {
            continue;
        }
        let (a, b) = if j < i { (j, i) } else { (i, j) };
        add_intra_edge_move(distance_matrix, lm, solution, a, b);
    }
}

fn add_intra_node_moves(
    distance_matrix: &Array2<f64>,
    lm: &mut BinaryHeap<Reverse<MoveType>>,
    solution: &Vec<usize>,
    i: usize,
) {
    for j in 0..solution.len() {
        if i == j {
            continue;
        }
        let (a, b) = if j < i { (j, i) } else { (i, j) };
        add_intra_node_move(distance_matrix, lm, solution, a, b);
    }
}
fn add_new_moves(
    data: &Vec<DataPoint>,
    distance_matrix: &Array2<f64>,
    change_edges: bool,
    lm: &mut BinaryHeap<Reverse<MoveType>>,
    best_solution: &Vec<usize>,
    bm: MoveType,
) {
    match bm {
        MoveType::IntraEdge(mv) => {
            add_inter_node_moves(
                data,
                distance_matrix,
                lm,
                best_solution,
                mv.edge1_first_node,
            );
            add_inter_node_moves(
                data,
                distance_matrix,
                lm,
                best_solution,
                mv.edge1_second_node,
            );
            add_inter_node_moves(
                data,
                distance_matrix,
                lm,
                best_solution,
                mv.edge2_first_node,
            );
            add_inter_node_moves(
                data,
                distance_matrix,
                lm,
                best_solution,
                mv.edge2_second_node,
            );

            add_intra_edge_moves(distance_matrix, lm, best_solution, mv.edge1_first_node);
            add_intra_edge_moves(distance_matrix, lm, best_solution, mv.edge1_second_node);
            add_intra_edge_moves(distance_matrix, lm, best_solution, mv.edge2_first_node);
            add_intra_edge_moves(distance_matrix, lm, best_solution, mv.edge2_second_node);
        }
        MoveType::IntraNode(mv) => {
            add_inter_node_moves(data, distance_matrix, lm, best_solution, mv.before_node1);
            add_inter_node_moves(data, distance_matrix, lm, best_solution, mv.node1);
            add_inter_node_moves(data, distance_matrix, lm, best_solution, mv.after_node1);
            add_inter_node_moves(data, distance_matrix, lm, best_solution, mv.before_node2);
            add_inter_node_moves(data, distance_matrix, lm, best_solution, mv.node2);
            add_inter_node_moves(data, distance_matrix, lm, best_solution, mv.after_node2);

            add_intra_node_moves(distance_matrix, lm, best_solution, mv.before_node1);
            add_intra_node_moves(distance_matrix, lm, best_solution, mv.node1);
            add_intra_node_moves(distance_matrix, lm, best_solution, mv.after_node1);
            add_intra_node_moves(distance_matrix, lm, best_solution, mv.before_node2);
            add_intra_node_moves(distance_matrix, lm, best_solution, mv.node2);
            add_intra_node_moves(distance_matrix, lm, best_solution, mv.after_node2);
        }
        MoveType::InterNode(mv) => {
            add_inter_node_moves(data, distance_matrix, lm, best_solution, mv.before_node1);
            add_inter_node_moves(data, distance_matrix, lm, best_solution, mv.node1);
            add_inter_node_moves(data, distance_matrix, lm, best_solution, mv.after_node1);
            if change_edges {
                add_intra_edge_moves(distance_matrix, lm, best_solution, mv.before_node1);
                add_intra_edge_moves(distance_matrix, lm, best_solution, mv.node1);
                add_intra_edge_moves(distance_matrix, lm, best_solution, mv.after_node1);
            } else {
                add_intra_node_moves(distance_matrix, lm, best_solution, mv.before_node1);
                add_intra_node_moves(distance_matrix, lm, best_solution, mv.node1);
                add_intra_node_moves(distance_matrix, lm, best_solution, mv.after_node1);
            }
        }
    }
}

pub fn local_search_w_cached_deltas(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
    change_edges: bool,
) -> Vec<usize> {
    let mut lm: BinaryHeap<Reverse<MoveType>> = BinaryHeap::new();
    let mut current_solution =
        generate_random_solution(data, starting_point_index, distance_matrix);
    add_all_moves(
        data,
        distance_matrix,
        &mut lm,
        &current_solution,
        change_edges,
    );
    loop {
        let mut best_solution: Vec<usize> = current_solution.clone();
        let mut best_delta: Option<f64> = None;
        let mut best_move: Option<MoveType> = None;
        let mut stored_moves: Vec<MoveType> = vec![];

        while !lm.is_empty() {
            let Reverse(mv) = lm.pop().unwrap();
            match apply_if_valid(&current_solution, &mut stored_moves, mv) {
                None => {}
                Some((new_solution, mv)) => {
                    best_delta = Some(mv.delta());
                    best_solution = new_solution;
                    best_move = Some(mv);
                    break;
                }
            }
        }
        if let Some(_delta) = best_delta {
            recover_stored_moves(&mut lm, &mut stored_moves);
            if let Some(bm) = best_move {
                add_new_moves(
                    data,
                    distance_matrix,
                    change_edges,
                    &mut lm,
                    &best_solution,
                    bm,
                );
                current_solution = best_solution;
            }
        } else {
            break;
        }
    }
    current_solution
}

pub fn local_search_w_cached_deltas_full(
    data: &Vec<DataPoint>,
    starting_point_index: usize,
    distance_matrix: &Array2<f64>,
    change_edges: bool,
) -> Vec<Vec<usize>> {
    let mut lm: BinaryHeap<Reverse<MoveType>> = BinaryHeap::new();
    let mut full_solution: Vec<Vec<usize>> = Vec::new();
    let mut current_solution =
        generate_random_solution(data, starting_point_index, distance_matrix);
    full_solution.push(current_solution.clone());
    add_all_moves(
        data,
        distance_matrix,
        &mut lm,
        &current_solution,
        change_edges,
    );
    loop {
        let mut best_solution: Vec<usize> = current_solution.clone();
        let mut best_delta: Option<f64> = None;
        let mut best_move: Option<MoveType> = None;
        let mut stored_moves: Vec<MoveType> = vec![];

        while !lm.is_empty() {
            let Reverse(mv) = lm.pop().unwrap();
            match apply_if_valid(&current_solution, &mut stored_moves, mv) {
                None => {}
                Some((new_solution, mv)) => {
                    best_delta = Some(mv.delta());
                    best_solution = new_solution;
                    best_move = Some(mv);
                    break;
                }
            }
        }
        if let Some(_delta) = best_delta {
            recover_stored_moves(&mut lm, &mut stored_moves);
            if let Some(bm) = best_move {
                add_new_moves(
                    data,
                    distance_matrix,
                    change_edges,
                    &mut lm,
                    &best_solution,
                    bm,
                );
                current_solution = best_solution;
                full_solution.push(current_solution.clone());
            }
        } else {
            break;
        }
    }
    full_solution
}
