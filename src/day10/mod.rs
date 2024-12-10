use std::collections::HashSet;

use crate::common::{
    grid::{Direction, Grid},
    string_utils::read_file_to_string,
};

pub fn run() {
    let input = read_file_to_string("input/day10.txt");
    let grid = Grid::new(&input.as_str());

    // find all the trailheads (0's)
    let mut trailheads: Vec<((usize, usize), HashSet<(usize, usize)>)> = Vec::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            if let Some('0') = grid.at(x as i32, y as i32) {
                trailheads.push(((x, y), HashSet::new()));
            }
        }
    }

    // for each trailhead, walk a path to find the 9s then sum
    let mut scores = 0;
    for (trailhead, nines) in trailheads.iter_mut() {
        let currentX = trailhead.0;
        let currentY = trailhead.1;

        walk(currentX as i32, currentY as i32, nines, &grid, 0);

        scores = scores + nines.len();
    }

    println!("{:?}", trailheads);

    println!("score: {}", scores);
}

pub fn run2() {
    let input = read_file_to_string("input/day10.txt");
    let grid = Grid::new(&input.as_str());

    // find all the trailheads (0's)
    let mut trailheads: Vec<((usize, usize), Vec<(usize, usize)>)> = Vec::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            if let Some('0') = grid.at(x as i32, y as i32) {
                trailheads.push(((x, y), Vec::new()));
            }
        }
    }

    // for each trailhead, walk a path to find the 9s then sum
    let mut scores = 0;
    for (trailhead, nines) in trailheads.iter_mut() {
        let currentX = trailhead.0;
        let currentY = trailhead.1;

        walk2(currentX as i32, currentY as i32, nines, &grid, 0);

        scores = scores + nines.len();
    }

    // println!("{:?}", trailheads);

    println!("score: {}", scores);
}

const NORTH: Direction = Direction { x: 0, y: -1 };
const EAST: Direction = Direction { x: 1, y: 0 };
const SOUTH: Direction = Direction { x: 0, y: 1 };
const WEST: Direction = Direction { x: -1, y: 0 };

fn walk(x: i32, y: i32, mut nines: &mut HashSet<(usize, usize)>, grid: &Grid, current_value: u32) {
    // terminating case
    if current_value == 9 {
        nines.insert((x as usize, y as usize));
    } else {
        // walk to all items one higher than current
        let target = current_value + 1;
        let target_char = target.to_string().chars().next().unwrap();
        if grid
            .at(x + NORTH.x, y + NORTH.y)
            .is_some_and(|g| g == target_char)
        {
            println!("Found {} at ({},{})", target_char, x + NORTH.x, y + NORTH.y);
            walk(x + NORTH.x, y + NORTH.y, nines, &grid, target);
        }

        if grid
            .at(x + EAST.x, y + EAST.y)
            .is_some_and(|g| g == target_char)
        {
            println!("Found {} at ({},{})", target_char, x + EAST.x, y + EAST.y);
            walk(x + EAST.x, y + EAST.y, nines, &grid, target);
        }

        if grid
            .at(x + SOUTH.x, y + SOUTH.y)
            .is_some_and(|g| g == target_char)
        {
            println!("Found {} at ({},{})", target_char, x + SOUTH.x, y + SOUTH.y);
            walk(x + SOUTH.x, y + SOUTH.y, nines, &grid, target);
        }

        if grid
            .at(x + WEST.x, y + WEST.y)
            .is_some_and(|g| g == target_char)
        {
            println!("Found {} at ({},{})", target_char, x + WEST.x, y + WEST.y);
            walk(x + WEST.x, y + WEST.y, nines, &grid, target);
        }
    }
}

fn walk2(x: i32, y: i32, mut nines: &mut Vec<(usize, usize)>, grid: &Grid, current_value: u32) {
    // terminating case
    if current_value == 9 {
        nines.push((x as usize, y as usize));
    } else {
        // walk to all items one higher than current
        let target = current_value + 1;
        let target_char = target.to_string().chars().next().unwrap();
        if grid
            .at(x + NORTH.x, y + NORTH.y)
            .is_some_and(|g| g == target_char)
        {
            // println!("Found {} at ({},{})", target_char, x + NORTH.x, y + NORTH.y);
            walk2(x + NORTH.x, y + NORTH.y, nines, &grid, target);
        }

        if grid
            .at(x + EAST.x, y + EAST.y)
            .is_some_and(|g| g == target_char)
        {
            // println!("Found {} at ({},{})", target_char, x + EAST.x, y + EAST.y);
            walk2(x + EAST.x, y + EAST.y, nines, &grid, target);
        }

        if grid
            .at(x + SOUTH.x, y + SOUTH.y)
            .is_some_and(|g| g == target_char)
        {
            // println!("Found {} at ({},{})", target_char, x + SOUTH.x, y + SOUTH.y);
            walk2(x + SOUTH.x, y + SOUTH.y, nines, &grid, target);
        }

        if grid
            .at(x + WEST.x, y + WEST.y)
            .is_some_and(|g| g == target_char)
        {
            // println!("Found {} at ({},{})", target_char, x + WEST.x, y + WEST.y);
            walk2(x + WEST.x, y + WEST.y, nines, &grid, target);
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{
        common::grid::Grid,
        day10::{walk, walk2},
    };

    const TEST_DATA: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    pub fn test1() {
        let grid = Grid::new(&TEST_DATA);

        // find all the trailheads (0's)
        let mut trailheads: Vec<((usize, usize), HashSet<(usize, usize)>)> = Vec::new();
        for y in 0..grid.height {
            for x in 0..grid.width {
                if let Some('0') = grid.at(x as i32, y as i32) {
                    trailheads.push(((x, y), HashSet::new()));
                }
            }
        }

        // for each trailhead, walk a path to find the 9s then sum
        let mut scores = 0;
        for (trailhead, nines) in trailheads.iter_mut() {
            let currentX = trailhead.0;
            let currentY = trailhead.1;

            walk(currentX as i32, currentY as i32, nines, &grid, 0);

            scores = scores + nines.len();
        }

        println!("{:?}", trailheads);

        println!("score: {}", scores);
    }

    #[test]
    pub fn test2() {
        let grid = Grid::new(&TEST_DATA);

        // find all the trailheads (0's)
        let mut trailheads: Vec<((usize, usize), Vec<(usize, usize)>)> = Vec::new();
        for y in 0..grid.height {
            for x in 0..grid.width {
                if let Some('0') = grid.at(x as i32, y as i32) {
                    trailheads.push(((x, y), Vec::new()));
                }
            }
        }

        // for each trailhead, walk a path to find the 9s then sum
        let mut scores = 0;
        for (trailhead, nines) in trailheads.iter_mut() {
            let currentX = trailhead.0;
            let currentY = trailhead.1;

            walk2(currentX as i32, currentY as i32, nines, &grid, 0);

            scores = scores + nines.len();
        }

        println!("{:?}", trailheads);

        println!("score: {}", scores);
    }
}
