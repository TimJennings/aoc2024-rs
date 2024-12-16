use std::fmt::Display;

use itertools::Itertools;
use nom::IResult;

use crate::common::{
    grid::Grid,
    string_utils::{read_file_to_string, read_file_to_vec},
};

pub fn run2() {
    // modify the grid
    let mut grid = &read_file_to_string("input/day15-grid.txt");
    let updated_grid = modify_grid(grid);
    let mut grid = parse_grid(&updated_grid);
    // println!("{:?}", grid);
    let program = parse_program(&read_file_to_string("input/day15-program.txt"));
    // println!("{:?}", program);

    for instruction in program.iter() {
        step(instruction, &mut grid);
    }

    println!("{}", grid);

    // sum gps positions
    let mut gps_sum = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if *grid.at(x as i32, y as i32).unwrap() == GridTile::LEFT_HALF_BOX {
                gps_sum = gps_sum + (y * 100) + x;
            }
        }
    }
    println!("GPS : {}", gps_sum);
}

pub fn run() {
    let mut grid = parse_grid(&read_file_to_string("input/day15-grid.txt"));
    // println!("{:?}", grid);
    let program = parse_program(&read_file_to_string("input/day15-program.txt"));
    // println!("{:?}", program);

    for instruction in program.iter() {
        step(instruction, &mut grid);
    }

    println!("{}", grid);

    // sum gps positions
    let mut gps_sum = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if *grid.at(x as i32, y as i32).unwrap() == GridTile::BOX {
                gps_sum = gps_sum + (y * 100) + x;
            }
        }
    }
    println!("GPS : {}", gps_sum);
}

#[derive(Debug, PartialEq, Clone)]
enum GridTile {
    WALL,
    ROBOT,
    BOX,
    SPACE,
    LEFT_HALF_BOX,
    RIGHT_HALF_BOX,
}

impl Display for GridTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WALL => write!(f, "#"),
            Self::ROBOT => write!(f, "@"),
            Self::BOX => write!(f, "O"),
            Self::SPACE => write!(f, "."),
            Self::LEFT_HALF_BOX => write!(f, "["),
            Self::RIGHT_HALF_BOX => write!(f, "]"),
            _ => write!(f, "?"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Program {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Program {
    pub fn get_direction(&self) -> (i32, i32) {
        match self {
            Program::DOWN => (0, 1),
            Program::UP => (0, -1),
            Program::LEFT => (-1, 0),
            Program::RIGHT => (1, 0),
            _ => (0, 0),
        }
    }
}

fn parse_grid(input: &str) -> Grid<GridTile> {
    let lines: Vec<String> = input.split('\n').map(|s| String::from(s.trim())).collect();

    let width = lines[0].len();
    let mut grid = Vec::new();
    for line in lines.iter() {
        for c in line.chars() {
            let tile = match c {
                '#' => GridTile::WALL,
                'O' => GridTile::BOX,
                '@' => GridTile::ROBOT,
                '.' => GridTile::SPACE,
                '[' => GridTile::LEFT_HALF_BOX,
                ']' => GridTile::RIGHT_HALF_BOX,
                x => panic!("unknown grid character {x}"),
            };
            grid.push(tile);
        }
    }

    Grid::new(grid, width, lines.len())
}

fn parse_program(input: &str) -> Vec<Program> {
    let program = input.split('\n').map(|s| String::from(s.trim())).join("");
    let mut result = Vec::new();
    for c in program.chars() {
        let program_entry = match c {
            '^' => Program::UP,
            'v' => Program::DOWN,
            '>' => Program::RIGHT,
            '<' => Program::LEFT,
            x => panic!("unknown program input {x}"),
        };
        result.push(program_entry);
    }
    result
}

fn step(instruction: &Program, grid: &mut Grid<GridTile>) {
    // locate robot
    let robot = grid.find_first(&GridTile::ROBOT).unwrap();

    // attempt instruction
    let offset = instruction.get_direction();

    match grid.at(robot.0 + offset.0, robot.1 + offset.1).unwrap() {
        GridTile::WALL => {
            println!("can't move into a wall");
        }
        GridTile::SPACE => {
            println!("can move into a space");
            // update grid
            grid.set(robot.0 + offset.0, robot.1 + offset.1, GridTile::ROBOT);
            grid.set(robot.0, robot.1, GridTile::SPACE);
        }
        GridTile::ROBOT => {
            println!("can't move into a robot");
        }
        GridTile::BOX => {
            print!("try and push the box...");
            let can_push = attempt_move_into(
                robot.0,
                robot.1,
                robot.0 + offset.0,
                robot.1 + offset.1,
                grid,
                instruction,
                true,
            );
            if can_push {
                println!("success");
            } else {
                println!("failure");
            }
        }
        GridTile::LEFT_HALF_BOX => {
            // left and right pushes work the same but up down needs extra handling
            if *instruction == Program::LEFT || *instruction == Program::RIGHT {
                print!("try and push the box...");
                let can_push = attempt_move_into(
                    robot.0,
                    robot.1,
                    robot.0 + offset.0,
                    robot.1 + offset.1,
                    grid,
                    instruction,
                    true,
                );
                if can_push {
                    println!("success");
                } else {
                    println!("failure");
                }
            } else {
                // up or down
                // check if this box can move but also the one to the right (the right half)
                let we_can_move = attempt_move_into(
                    robot.0,
                    robot.1,
                    robot.0 + offset.0,
                    robot.1 + offset.1,
                    grid,
                    instruction,
                    false,
                );
                let left_can_move = attempt_move_into(
                    robot.0,
                    robot.1,
                    robot.0 + offset.0 + 1,
                    robot.1 + offset.1,
                    grid,
                    instruction,
                    false,
                );
                if we_can_move && left_can_move {
                    double_move_into(
                        robot.0,
                        robot.1,
                        robot.0 + offset.0,
                        robot.1 + offset.1,
                        robot.0 + offset.0 + 1,
                        robot.1 + offset.1,
                        grid,
                        instruction,
                    );
                }
            }
        }
        GridTile::RIGHT_HALF_BOX => {
            // left and right pushes work the same but up down needs extra handling
            if *instruction == Program::LEFT || *instruction == Program::RIGHT {
                // print!("try and push the box...");
                let can_push = attempt_move_into(
                    robot.0,
                    robot.1,
                    robot.0 + offset.0,
                    robot.1 + offset.1,
                    grid,
                    instruction,
                    true,
                );
                if can_push {
                    // println!("success");
                } else {
                    // println!("failure");
                }
            } else {
                // up or down
                // check if this box can move but also the one to the left (the left half)
                let we_can_move = attempt_move_into(
                    robot.0,
                    robot.1,
                    robot.0 + offset.0,
                    robot.1 + offset.1,
                    grid,
                    instruction,
                    false,
                );
                let left_can_move = attempt_move_into(
                    robot.0,
                    robot.1,
                    robot.0 + offset.0 - 1,
                    robot.1 + offset.1,
                    grid,
                    instruction,
                    false,
                );
                if we_can_move && left_can_move {
                    double_move_into(
                        robot.0,
                        robot.1,
                        robot.0 + offset.0,
                        robot.1 + offset.1,
                        robot.0 + offset.0 - 1,
                        robot.1 + offset.1,
                        grid,
                        instruction,
                    );
                }
            }
        }
    }
}

fn double_move_into(
    robot_x: i32,
    robot_y: i32,
    first_half_x: i32,
    first_half_y: i32,
    second_half_x: i32,
    second_half_y: i32,
    grid: &mut Grid<GridTile>,
    instruction: &Program,
) {
    // println!(
    //     "move robot from ({},{}) into box of ({},{}),({},{})",
    //     robot_x, robot_y, first_half_x, first_half_y, second_half_x, second_half_y
    // );

    // push both halfs then swap the robot
    let (direction_x, direction_y) = instruction.get_direction();
    attempt_move_into(
        first_half_x,
        first_half_y,
        first_half_x + direction_x,
        first_half_y + direction_y,
        grid,
        instruction,
        true,
    );
    attempt_move_into(
        second_half_x,
        second_half_y,
        second_half_x + direction_x,
        second_half_y + direction_y,
        grid,
        instruction,
        true,
    );

    grid.set(first_half_x, first_half_y, GridTile::ROBOT);
    grid.set(robot_x, robot_y, GridTile::SPACE);

    // println!("{}", grid);
}

fn attempt_move_into(
    from_x: i32,
    from_y: i32,
    x: i32,
    y: i32,
    grid: &mut Grid<GridTile>,
    instruction: &Program,
    actually_move: bool,
) -> bool {
    let from_tile = grid.at(from_x, from_y).unwrap().clone();
    match grid.at(x, y).unwrap().clone() {
        GridTile::WALL => false,
        GridTile::SPACE => {
            if actually_move {
                grid.set(x, y, from_tile.clone());
                grid.set(from_x, from_y, GridTile::SPACE);
                // println!("{}", grid);
            }
            true
        }
        GridTile::ROBOT => {
            println!("can't move into a robot");
            false
        }
        GridTile::BOX => {
            println!("try and push the box");
            let (direction_x, direction_y) = instruction.get_direction();
            let attempt = attempt_move_into(
                x,
                y,
                x + direction_x,
                y + direction_y,
                grid,
                instruction,
                actually_move,
            );
            if attempt {
                if actually_move {
                    grid.set(x, y, from_tile.clone());
                    grid.set(from_x, from_y, GridTile::SPACE);
                }
                true
            } else {
                false
            }
        }
        GridTile::LEFT_HALF_BOX => {
            // println!("try and push this box but also my right half");
            let (direction_x, direction_y) = instruction.get_direction();
            // left and right pushes work the same but up down needs extra handling
            if *instruction == Program::LEFT || *instruction == Program::RIGHT {
                print!("try and push the box...");
                let can_push = attempt_move_into(
                    x,
                    y,
                    x + direction_x,
                    y + direction_y,
                    grid,
                    instruction,
                    true,
                );
                if can_push {
                    if actually_move {
                        grid.set(x, y, from_tile.clone());
                        grid.set(from_x, from_y, GridTile::SPACE);
                        // println!("{}", grid);
                    }
                    true
                } else {
                    false
                }
            } else {
                // up or down
                // check if this box can move but also the one to the left (the left half)
                let we_can_move = attempt_move_into(
                    x,
                    y,
                    x + direction_x,
                    y + direction_y,
                    grid,
                    instruction,
                    actually_move,
                );
                let right_can_move = attempt_move_into(
                    x + 1,
                    y,
                    x + direction_x + 1,
                    y + direction_y,
                    grid,
                    instruction,
                    actually_move,
                );

                if actually_move && right_can_move && we_can_move {
                    // during actually move we only move ourselves as the second push will come from earlier up the call stack
                    let swap_tile = grid.at(x, y).unwrap().clone();
                    grid.set(x, y, from_tile.clone());
                    grid.set(from_x, from_y, swap_tile);
                }

                right_can_move && we_can_move
            }
        }
        GridTile::RIGHT_HALF_BOX => {
            // println!("try and push this box but also my left half");
            let (direction_x, direction_y) = instruction.get_direction();
            // left and right pushes work the same but up down needs extra handling
            if *instruction == Program::LEFT || *instruction == Program::RIGHT {
                print!("try and push the box...");
                let can_push = attempt_move_into(
                    x,
                    y,
                    x + direction_x,
                    y + direction_y,
                    grid,
                    instruction,
                    true,
                );
                if can_push {
                    if actually_move {
                        grid.set(x, y, from_tile.clone());
                        grid.set(from_x, from_y, GridTile::SPACE);
                        // println!("{}", grid);
                    }
                    true
                } else {
                    false
                }
            } else {
                // up or down
                // check if this box can move but also the one to the left (the left half)
                let we_can_move = attempt_move_into(
                    x,
                    y,
                    x + direction_x,
                    y + direction_y,
                    grid,
                    instruction,
                    actually_move,
                );
                let left_can_move = attempt_move_into(
                    x - 1,
                    y,
                    x + direction_x - 1,
                    y + direction_y,
                    grid,
                    instruction,
                    actually_move,
                );

                if actually_move && left_can_move && we_can_move {
                    // during actually move we only move ourselves as the second push will come from earlier up the call stack
                    let swap_tile = grid.at(x, y).unwrap().clone();
                    grid.set(x, y, from_tile.clone());
                    grid.set(from_x, from_y, swap_tile);
                }

                return left_can_move && we_can_move;
            }
        }
    }
}

fn modify_grid(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars() {
        match c {
            '#' => output.push_str("##"),
            'O' => output.push_str("[]"),
            '@' => output.push_str("@."),
            '.' => output.push_str(".."),
            x => output.push(x),
        }
    }
    output
}

#[cfg(test)]
mod test {
    use crate::day15::{modify_grid, parse_program, step, GridTile};

    use super::parse_grid;

    const TEST_DATA_GRID: &str = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########";

    const TEST_DATA_PROGRAM: &str = r"<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const TEST_DATA_GRID_SMALL: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";

    const TEST_DATA_PROGRAM_SMALL: &str = r"<^^>>>vv<v>>v<<";

    const TEST_DATA_GRID_2_SMALL: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######";
    const TEST_DATA_PROGRAM_2_SMALL: &str = r"<vv<<^^<<^^";

    #[test]
    pub fn test1() {
        let mut grid = parse_grid(TEST_DATA_GRID);
        // println!("{:?}", grid);
        let program = parse_program(TEST_DATA_PROGRAM);
        // println!("{:?}", program);

        for instruction in program.iter() {
            step(instruction, &mut grid);
        }

        println!("{}", grid);

        // sum gps positions
        let mut gps_sum = 0;
        for y in 0..grid.height {
            for x in 0..grid.width {
                if *grid.at(x as i32, y as i32).unwrap() == GridTile::BOX {
                    gps_sum = gps_sum + (y * 100) + x;
                }
            }
        }
        println!("GPS : {}", gps_sum);
    }

    #[test]
    pub fn test_small() {
        let mut grid = parse_grid(TEST_DATA_GRID_SMALL);
        println!("{:?}", grid);
        let program = parse_program(TEST_DATA_PROGRAM_SMALL);
        // println!("{:?}", program);

        for instruction in program.iter() {
            step(instruction, &mut grid);
            println!("{}", grid);
        }

        // sum gps positions
        let mut gps_sum = 0;
        for y in 0..grid.height {
            for x in 0..grid.width {
                if *grid.at(x as i32, y as i32).unwrap() == GridTile::BOX {
                    gps_sum = gps_sum + (y * 100) + x;
                }
            }
        }
        println!("GPS : {}", gps_sum);
    }

    #[test]
    pub fn test_round_two_small() {
        // modify the grid
        let updated_grid = modify_grid(TEST_DATA_GRID_2_SMALL);
        let mut grid = parse_grid(&updated_grid);
        println!("{}", grid);
        let program = parse_program(TEST_DATA_PROGRAM_2_SMALL);
        // println!("{:?}", program);

        for instruction in program.iter() {
            step(instruction, &mut grid);
            println!("{}", grid);
        }

        // sum gps positions
        let mut gps_sum = 0;
        for y in 0..grid.height {
            for x in 0..grid.width {
                if *grid.at(x as i32, y as i32).unwrap() == GridTile::BOX {
                    gps_sum = gps_sum + (y * 100) + x;
                }
            }
        }
        println!("GPS : {}", gps_sum);
    }

    #[test]
    pub fn test_round_two_large() {
        // modify the grid
        let updated_grid = modify_grid(TEST_DATA_GRID);
        let mut grid = parse_grid(&updated_grid);
        println!("{}", grid);
        let program = parse_program(TEST_DATA_PROGRAM);
        // println!("{:?}", program);

        for instruction in program.iter() {
            step(instruction, &mut grid);
            println!("{}", grid);
        }

        // sum gps positions
        let mut gps_sum = 0;
        for y in 0..grid.height {
            for x in 0..grid.width {
                if *grid.at(x as i32, y as i32).unwrap() == GridTile::LEFT_HALF_BOX {
                    gps_sum = gps_sum + (y * 100) + x;
                }
            }
        }
        println!("GPS : {}", gps_sum);
    }
}
