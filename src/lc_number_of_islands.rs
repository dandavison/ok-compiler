struct Solution;

use std::collections::HashMap;

type Idx = i32;
type Cell = (Idx, Idx);

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        // Represent grid as map
        let mut grid_map = HashMap::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                grid_map.insert((i as Idx, j as Idx), val);
            }
        }

        let mut n_islands = 0;

        // Make a single pass through the grid. If cell is land, increment
        // counter and perform a dfs to explore the connected component.
        let cells: Vec<Cell> = grid_map.keys().cloned().collect();
        for cell in cells {
            if grid_map.get(&cell).unwrap() == &&'1' {
                n_islands += 1;
                dfs(&cell, &mut grid_map);
            }
        }

        n_islands
    }
}

fn dfs(cell: &Cell, grid_map: &mut HashMap<Cell, &char>) {
    let (i, j) = *cell;
    for neighbor in &[
        (i.checked_sub(1).unwrap(), j),
        (i + 1, j),
        (i, j.checked_sub(1).unwrap()),
        (i, j + 1),
    ] {
        if let Some('1') = grid_map.get(neighbor) {
            grid_map.insert(*neighbor, &'0');
            dfs(neighbor, grid_map)
        }
    }
}

pub fn main() {
    let n = Solution::num_islands(vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ]);
    dbg!(n);
}
