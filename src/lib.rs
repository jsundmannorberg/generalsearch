use std::hash::Hash;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn breadth_first_search<A, B: Hash+Eq+Clone>
(
    maze: &A, 
    initial_state: B, 
    final_state: impl Fn(&A, &B) -> bool, 
    get_new_states: impl Fn(&A, &B) -> Vec<B>
) -> Option<Vec<B>> {
    let mut visited: HashSet<B> = HashSet::new();
    let mut moves: VecDeque<(Vec<B>, B)> = VecDeque::new();
    moves.push_back((Vec::new(), initial_state));
    loop {
        match moves.pop_front() {
            Some(m) => {
                let state = m.1.clone();
                if visited.contains(&state) {
                    continue;
                }
                if final_state(&maze, &state) {
                    let mut nm = m.0;
                    nm.push(state);
                    return Some(nm);
                }
                let new_moves = get_new_states(&maze, &state);
                for i in 0..(new_moves.len()) {
                    let mm = m.1.clone();
                    let mut nm = m.0.clone();
                    nm.push(mm);
                    moves.push_back((nm, new_moves[i].clone()));
                }
                visited.insert(state);
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
        let res = super::breadth_first_search(&maze, initial_state, final_state, get_new_states);
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
