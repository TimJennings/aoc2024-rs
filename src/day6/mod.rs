use std::collections::HashSet;

use nom::combinator::peek;

use crate::common::{
    grid::{self, Direction, Grid},
    string_utils::read_file_to_string,
};

pub fn run() {
    let input = read_file_to_string("input/day6.txt");
    let grid = grid::Grid::new(&input);

    let mut set = HashSet::new();

    println!("{:?}", grid);

    // find the guard
    println!("{:?}", grid.find_first('^'));
    let (mut guardX, mut guardY) = grid.find_first('^').unwrap();

    let mut guardDirection = NORTH;
    let mut character_at = grid.at(guardX, guardY).unwrap_or(',');
    while character_at != ',' {
        // record where the guard has been
        set.insert((guardX, guardY));

        // calc peek them
        let peek_x = guardX + guardDirection.x;
        let peek_y = guardY + guardDirection.y;
        // peek next character
        let next_character = grid.at(peek_x, peek_y).unwrap_or(',');
        println!(
            "consider moving guard to ({},{}), it is a {}",
            peek_x, peek_y, next_character
        );
        if next_character == '#' {
            // needs to turn
            guardDirection = turn_right(&guardDirection);
        }

        // move
        guardX = guardX + guardDirection.x;
        guardY = guardY + guardDirection.y;
        character_at = grid.at(guardX, guardY).unwrap_or(',');
        println!(
            "moved guard to ({},{}), it is a {}",
            peek_x, peek_y, next_character
        );
    }

    println!("guard visited {} places", set.len());
}

pub fn run2() {
    let input = read_file_to_string("input/day6.txt");
    let grid = grid::Grid::new(&input);

    let mut set = HashSet::new();

    // println!("{:?}", grid);

    // find the guard
    // println!("{:?}", grid.find_first('^'));
    let (mut guardX, mut guardY) = grid.find_first('^').unwrap();

    let mut guardDirection = NORTH;
    let mut character_at = grid.at(guardX, guardY).unwrap_or(',');
    while character_at != ',' {
        // record where the guard has been
        set.insert((guardX, guardY));

        // calc peek them
        let peek_x = guardX + guardDirection.x;
        let peek_y = guardY + guardDirection.y;
        // peek next character
        let next_character = grid.at(peek_x, peek_y).unwrap_or(',');
        // println!(
        //     "consider moving guard to ({},{}), it is a {}",
        //     peek_x, peek_y, next_character
        // );
        if next_character == '#' {
            // needs to turn
            guardDirection = turn_right(&guardDirection);
        }

        // move
        guardX = guardX + guardDirection.x;
        guardY = guardY + guardDirection.y;
        character_at = grid.at(guardX, guardY).unwrap_or(',');
        // println!(
        //     "moved guard to ({},{}), it is a {}",
        //     peek_x, peek_y, next_character
        // );
    }

    // with the exception of the initial position
    let (newGuardX, newGuardY) = grid.find_first('^').unwrap();
    set.remove(&(newGuardX, newGuardY));
    // regenerate the grid with that space blocked and check for loops
    let mut block_positions = HashSet::new();
    for position in set.iter() {
        let mut modified_grid = grid.clone();
        modified_grid.set(position.0, position.1, '#');
        let mut walk_set = HashSet::new();
        // walk_set.insert((newGuardX, newGuardY, NORTH));

        if contains_loop(modified_grid, walk_set, newGuardX, newGuardY, NORTH) {
            // can block at position
            block_positions.insert((position.0, position.1));
        }
    }

    println!("{:?}", block_positions.len());
}

const NORTH: Direction = Direction { x: 0, y: -1 };
const EAST: Direction = Direction { x: 1, y: 0 };
const SOUTH: Direction = Direction { x: 0, y: 1 };
const WEST: Direction = Direction { x: -1, y: 0 };

pub fn turn_right(direction: &Direction) -> Direction {
    match *direction {
        NORTH => EAST,
        EAST => SOUTH,
        SOUTH => WEST,
        WEST => NORTH,
        _ => panic!("unknown direction"),
    }
}

fn contains_loop(
    grid: grid::Grid,
    mut loop_set: HashSet<(i32, i32, Direction)>,
    mut guardX: i32,
    mut guardY: i32,
    mut guardDirection: Direction,
) -> bool {
    let mut character_at = grid.at(guardX, guardY).unwrap_or(',');

    while character_at != ',' {
        // has guard been here before?
        if loop_set.contains(&(guardX, guardY, guardDirection)) {
            // loop!
            return true;
        }
        loop_set.insert((guardX, guardY, guardDirection));

        // calc peek them
        let mut peek_x = guardX + guardDirection.x;
        let mut peek_y = guardY + guardDirection.y;
        // peek next character
        let mut next_character = grid.at(peek_x, peek_y).unwrap_or(',');
        // println!(
        //     "consider moving guard to ({},{}), it is a {}",
        //     peek_x, peek_y, next_character
        // );

        while next_character == '#' {
            // needs to turn
            guardDirection = turn_right(&guardDirection);
            peek_x = guardX + guardDirection.x;
            peek_y = guardY + guardDirection.y;
            // peek next character
            next_character = grid.at(peek_x, peek_y).unwrap_or(',');
        }
        //move
        guardX = guardX + guardDirection.x;
        guardY = guardY + guardDirection.y;

        character_at = grid.at(guardX, guardY).unwrap_or(',');
        // println!(
        //     "moved guard to ({},{}), it is a {}",
        //     peek_x, peek_y, next_character
        // );
    }
    false
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{
        common::{
            self,
            grid::{self},
        },
        day6::{contains_loop, turn_right, NORTH},
    };

    const TEST_DATA: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    pub fn test() {
        let grid = grid::Grid::new(&TEST_DATA);

        let mut set = HashSet::new();

        println!("{:?}", grid);

        // find the guard
        println!("{:?}", grid.find_first('^'));
        let (mut guardX, mut guardY) = grid.find_first('^').unwrap();

        let mut guardDirection = NORTH;
        let mut character_at = grid.at(guardX, guardY).unwrap_or(',');
        while character_at != ',' {
            // record where the guard has been
            set.insert((guardX, guardY));

            // calc peek them
            let peek_x = guardX + guardDirection.x;
            let peek_y = guardY + guardDirection.y;
            // peek next character
            let next_character = grid.at(peek_x, peek_y).unwrap_or(',');
            println!(
                "consider moving guard to ({},{}), it is a {}",
                peek_x, peek_y, next_character
            );
            if next_character == '#' {
                // needs to turn
                guardDirection = turn_right(&guardDirection);
            }

            // move
            guardX = guardX + guardDirection.x;
            guardY = guardY + guardDirection.y;
            character_at = grid.at(guardX, guardY).unwrap_or(',');
            println!(
                "moved guard to ({},{}), it is a {}",
                peek_x, peek_y, next_character
            );
        }

        assert_eq!(41, set.len());
        println!("guard visited {} places", set.len());
    }

    #[test]
    pub fn loop_trace() {
        // let input = common::string_utils::read_file_to_string("input/day6.txt");
        let grid = grid::Grid::new(&TEST_DATA);

        let mut set = HashSet::new();

        println!("{:?}", grid);

        // find the guard
        println!("{:?}", grid.find_first('^'));
        let (mut guardX, mut guardY) = grid.find_first('^').unwrap();

        let mut guardDirection = NORTH;
        let mut character_at = grid.at(guardX, guardY).unwrap_or(',');
        while character_at != ',' {
            // record where the guard has been
            set.insert((guardX, guardY));

            // calc peek them
            let mut peek_x = guardX + guardDirection.x;
            let mut peek_y = guardY + guardDirection.y;
            // peek next character
            let mut next_character = grid.at(peek_x, peek_y).unwrap_or(',');
            println!(
                "consider moving guard to ({},{}), it is a {}",
                peek_x, peek_y, next_character
            );
            while next_character == '#' {
                // needs to turn
                guardDirection = turn_right(&guardDirection);
                peek_x = guardX + guardDirection.x;
                peek_y = guardY + guardDirection.y;
                // peek next character
                next_character = grid.at(peek_x, peek_y).unwrap_or(',');
            }
            //move
            guardX = guardX + guardDirection.x;
            guardY = guardY + guardDirection.y;

            character_at = grid.at(guardX, guardY).unwrap_or(',');
            println!(
                "moved guard to ({},{}), it is a {}",
                peek_x, peek_y, next_character
            );
        }

        // with the exception of the initial position
        let (newGuardX, newGuardY) = grid.find_first('^').unwrap();
        set.remove(&(newGuardX, newGuardY));
        // regenerate the grid with that space blocked and check for loops
        let mut block_positions = HashSet::new();
        for position in set.iter() {
            let mut modified_grid = grid.clone();
            modified_grid.set(position.0, position.1, '#');
            let mut walk_set = HashSet::new();
            // walk_set.insert((newGuardX, newGuardY, NORTH));

            if contains_loop(modified_grid, walk_set, newGuardX, newGuardY, NORTH) {
                // can block at position
                block_positions.insert((position.0, position.1));
            }
        }
        assert_eq!(6, block_positions.len());
    }
}
