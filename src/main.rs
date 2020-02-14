use generalsearch::breadth_first_search;

fn more_complex_maze(i: i32) {
        let maze: Vec<Vec<i32>> = vec![vec![1, 1, 1, 1, 1, 1],
                                       vec![1, 0, 1, 0, 0, 1],
                                       vec![1, 0, 0, 0, 0, 1],
                                       vec![1, 0, 1, 1, 2, 1],
                                       vec![1, 0, 0, 0, 1, 1],
                                       vec![1, 1, 1, 0, 1, 1],
                                       vec![1, 0, 0, 0, 0, 1],
                                       vec![1, 1, 1, 1, 1, 1]];
        let res = breadth_first_search(
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
        println!("{}", i);
    }


fn main() {
    for i in 0..10000 {
        more_complex_maze(i);
    }
}