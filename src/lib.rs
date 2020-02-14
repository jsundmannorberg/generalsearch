use std::hash::Hash;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone)]
struct SearchTreeNode<B: Clone> {
    node: B,
    id: usize,
    parent: usize,
}

#[derive(Clone)]
struct SearchTree<B: Clone> {
    nodes: Vec<SearchTreeNode<B>>,
}

impl<B: Clone> SearchTree<B> {
    fn to_vec(&self) -> Vec<B> {
        let mut res = Vec::new();
        let mut idx = self.nodes.len() - 1;
        loop {
            res.push(self.nodes[idx].clone());
            if idx == 0 {
                break;
            }
            idx = self.nodes[idx].parent;
        }
        res.iter().rev().map(|n: &SearchTreeNode<B>| { n.clone().node }).collect::<Vec<B>>()
    }

    fn push(&mut self, node: B, parent: usize) {
        self.nodes.push(SearchTreeNode { node, id: self.nodes.len(), parent });
    }
}

pub fn breadth_first_search<A, B: Hash+Eq+Clone>
(
    maze: &A, 
    initial_state: B, 
    final_state: impl Fn(&A, &B) -> bool, 
    get_new_states: impl Fn(&A, &B) -> Vec<B>
) -> Option<Vec<B>> {
    let mut visited: HashSet<B> = HashSet::new();
    let mut tree: SearchTree<B> = SearchTree { nodes: Vec::new() };
    let mut moves: VecDeque<usize> = VecDeque::new();
    let start = SearchTreeNode { node: initial_state, id: 0, parent: 0 };
    
    tree.nodes.push(start);
    moves.push_back(0);

    loop {
        match moves.pop_front() {
            Some(m) => {
                if visited.contains(&tree.nodes[m].node) {
                    continue;
                }
                if final_state(&maze, &tree.nodes[m].node) {
                    tree.push(tree.nodes[m].node.clone(), tree.nodes[m].parent);
                    return Some(tree.to_vec());
                }
                let mut new_moves = get_new_states(&maze, &tree.nodes[m].node);
                loop {
                    match new_moves.pop() {
                        Some(new_move) => {
                            moves.push_back(tree.nodes.len());
                            tree.push(new_move, tree.nodes[m].id);                
                        },
                        None => {
                            break;
                        }
                    }
                }
                
                visited.insert(tree.nodes[m].node.clone());
            },
            None => {
                return None
            }
        }
    }
}


#[cfg(test)]
mod tests {

    fn simple_final_state(_: &Vec<usize>, x: &usize) -> bool {
        *x == 5
    }
    fn simple_get_new_states(_: &Vec<usize>, x: &usize) -> Vec<usize> {
        vec![*x + 1]
    }

    #[test]
    fn simple_maze() {
        let maze: Vec<usize> = vec![0, 0, 0, 0, 0, 0];
        let initial_state = 0 as usize;
        let final_state = simple_final_state;
        let get_new_states = simple_get_new_states;
        let res = super::faster_breadth_first_search(&maze, initial_state, final_state, get_new_states);
        assert_eq!(res, Some(vec![0, 1, 2, 3, 4, 5]));
    }

    #[test]
    fn more_complex_maze() {
        let maze: Vec<Vec<i32>> = vec![vec![1, 1, 1, 1, 1, 1],
                                       vec![1, 0, 1, 0, 0, 1],
                                       vec![1, 0, 0, 0, 0, 1],
                                       vec![1, 0, 1, 1, 2, 1],
                                       vec![1, 0, 0, 0, 1, 1],
                                       vec![1, 1, 1, 0, 1, 1],
                                       vec![1, 0, 0, 0, 0, 1],
                                       vec![1, 1, 1, 1, 1, 1]];
        let res = super::faster_breadth_first_search(
            &maze, 
            (4, 6), 
            |maze: &Vec<Vec<i32>>, x: &(usize, usize)| { maze[x.1][x.0] == 2 }, 
            |maze: &Vec<Vec<i32>>, pos: &(usize, usize)| {
                let (x, y) = *pos;
                let positions = vec![(x-1, y), (x, y-1), (x+1, y), (x, y+1)];
                positions.into_iter().filter(|&p| {
                    maze[p.1][p.0] != 1
                }).collect::<Vec<(usize, usize)>>()
            }
        );
        assert_eq!(
            res, 
            Some(
                vec![
                    (4, 6), 
                    (3, 6), 
                    (3, 5), 
                    (3, 4), 
                    (2, 4), 
                    (1, 4), 
                    (1, 3), 
                    (1, 2), 
                    (2, 2), 
                    (3, 2), 
                    (4, 2), 
                    (4, 3)
                ]
            )
        );
    }

    #[test]
    fn more_complex_maze_no_solution() {
        let maze: Vec<Vec<i32>> = vec![vec![1, 1, 1, 1, 1, 1],
                                       vec![1, 0, 1, 0, 0, 1],
                                       vec![1, 0, 0, 0, 0, 1],
                                       vec![1, 0, 1, 1, 0, 1],
                                       vec![1, 0, 0, 0, 1, 1],
                                       vec![1, 1, 1, 0, 1, 1],
                                       vec![1, 0, 0, 0, 0, 1],
                                       vec![1, 1, 1, 1, 1, 1]];
        let res = super::breadth_first_search(
            &maze, 
            (4, 6), 
            |maze: &Vec<Vec<i32>>, x: &(usize, usize)| { maze[x.1][x.0] == 2 }, 
            |maze: &Vec<Vec<i32>>, pos: &(usize, usize)| {
                let (x, y) = *pos;
                let positions = vec![(x-1, y), (x, y-1), (x+1, y), (x, y+1)];
                positions.into_iter().filter(|&p| {
                    maze[p.1][p.0] != 1
                }).collect::<Vec<(usize, usize)>>()
            }
        );
        assert_eq!(res, None);
    }   
    
}
