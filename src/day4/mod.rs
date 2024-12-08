use crate::common;

pub fn run() {
    let input = common::string_utils::read_file_to_string("input/day4.txt");
    let grid = Grid::new(&input);

    let NORTH = Direction { x: -1, y: 0 };
    let NE: Direction = Direction { x: -1, y: 1 };
    let EAST: Direction = Direction { x: 0, y: 1 };
    let SE: Direction = Direction { x: 1, y: 1 };
    let SOUTH: Direction = Direction { x: 1, y: 0 };
    let SW: Direction = Direction { x: 1, y: -1 };
    let WEST: Direction = Direction { x: 0, y: -1 };
    let NW: Direction = Direction { x: -1, y: -1 };

    let directions: Vec<Direction> = vec![NORTH, NE, EAST, SE, SOUTH, SW, WEST, NW];

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut xmas_count = 0;

    while y < grid.height as i32 {
        while x < grid.width as i32 {
            // is it an X
            if grid.at(x, y).unwrap_or('.') == 'X' {
                for direction in directions.iter() {
                    let mut checkX = x + direction.x;
                    let mut checkY = y + direction.y;
                    if grid.at(checkX, checkY).unwrap_or('.') == 'M' {
                        checkX = checkX + direction.x;
                        checkY = checkY + direction.y;
                        if grid.at(checkX, checkY).unwrap_or('.') == 'A' {
                            checkX = checkX + direction.x;
                            checkY = checkY + direction.y;
                            if grid.at(checkX, checkY).unwrap_or('.') == 'S' {
                                // found one
                                println!("found at ({},{}), direction {:?}", x, y, direction);
                                xmas_count = xmas_count + 1;
                            }
                        }
                    }
                }
            }

            x = x + 1;
        }

        y = y + 1;
        x = 0;
    }
    println!("found {} xmases", xmas_count);
}

pub fn run2() {
    let input = common::string_utils::read_file_to_string("input/day4.txt");
    let grid = Grid::new(&input);

    let NORTH = Direction { x: -1, y: 0 };
    let NE: Direction = Direction { x: -1, y: 1 };
    let EAST: Direction = Direction { x: 0, y: 1 };
    let SE: Direction = Direction { x: 1, y: 1 };
    let SOUTH: Direction = Direction { x: 1, y: 0 };
    let SW: Direction = Direction { x: 1, y: -1 };
    let WEST: Direction = Direction { x: 0, y: -1 };
    let NW: Direction = Direction { x: -1, y: -1 };

    let directions: Vec<Direction> = vec![NORTH, NE, EAST, SE, SOUTH, SW, WEST, NW];

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut mas_count = 0;

    while y < grid.height as i32 {
        while x < grid.width as i32 {
            // is it an A
            if grid.at(x, y).unwrap_or('.') == 'A' {
                let mut firstDiag = false;
                // check for S or M at NW
                if grid.at(x + NW.x, y + NW.y).unwrap_or('.') == 'M' {
                    // find S at SE
                    if grid.at(x + SE.x, y + SE.y).unwrap_or('.') == 'S' {
                        // found one
                        firstDiag = true;
                    }
                } else if grid.at(x + NW.x, y + NW.y).unwrap_or('.') == 'S' {
                    // find M at SE
                    if grid.at(x + SE.x, y + SE.y).unwrap_or('.') == 'M' {
                        // found one
                        firstDiag = true;
                    }
                }

                if firstDiag {
                    // check for S or M at NE
                    if grid.at(x + NE.x, y + NE.y).unwrap_or('.') == 'M' {
                        // find S at SW
                        if grid.at(x + SW.x, y + SW.y).unwrap_or('.') == 'S' {
                            // found one
                            println!("found at ({},{})", x, y);
                            mas_count = mas_count + 1;
                        }
                    } else if grid.at(x + NE.x, y + NE.y).unwrap_or('.') == 'S' {
                        // find M at SE
                        if grid.at(x + SW.x, y + SW.y).unwrap_or('.') == 'M' {
                            // found one
                            mas_count = mas_count + 1;
                        }
                    }
                }
            }

            x = x + 1;
        }

        y = y + 1;
        x = 0;
    }

    println!("mas count: {}", mas_count);
}

// walk to an x, look in the directions for a M then A then S in that same direction, if found increment count and continue

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct Direction {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Grid {
    grid: String,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(mut input: &str) -> Self {
        let vecI = common::string_utils::string_to_vec(input);

        let width = vecI[0].len();
        let height = vecI.len();
        Grid {
            grid: vecI.join(""),
            height: height,
            width: width,
        }
    }

    pub fn at(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return None;
        } else {
            if (y as usize * self.width) + x as usize >= 19739 {
                println!("aaah {},{}", x, y);
            }

            return Some(self.grid.as_bytes()[(y as usize * self.width) + x as usize] as char);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common;

    use super::{Direction, Grid, *};

    const TEST_DATA: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    pub fn test_parse() {
        // let grid = common::string_utils::string_to_vec(TEST_DATA);

        let grid = Grid::new(TEST_DATA);

        println!("{:?}", grid);
        assert_eq!('M', grid.at(0, 0).unwrap());
        assert_eq!('M', grid.at(1, 0).unwrap());
        assert_eq!('M', grid.at(2, 0).unwrap());
        assert_eq!('S', grid.at(3, 0).unwrap());
        assert_eq!('X', grid.at(4, 0).unwrap());
        assert_eq!('X', grid.at(5, 0).unwrap());
        assert_eq!('M', grid.at(6, 0).unwrap());
        assert_eq!('A', grid.at(7, 0).unwrap());
        assert_eq!('S', grid.at(8, 0).unwrap());
        assert_eq!('M', grid.at(9, 0).unwrap());
        assert_eq!('M', grid.at(0, 1).unwrap());
        assert_eq!('S', grid.at(1, 1).unwrap());
        assert_eq!('X', grid.at(9, 9).unwrap());
    }

    #[test]
    pub fn test_2() {
        let grid = Grid::new(TEST_DATA);

        let NORTH = Direction { x: -1, y: 0 };
        let NE: Direction = Direction { x: -1, y: 1 };
        let EAST: Direction = Direction { x: 0, y: 1 };
        let SE: Direction = Direction { x: 1, y: 1 };
        let SOUTH: Direction = Direction { x: 1, y: 0 };
        let SW: Direction = Direction { x: 1, y: -1 };
        let WEST: Direction = Direction { x: 0, y: -1 };
        let NW: Direction = Direction { x: -1, y: -1 };

        let directions: Vec<Direction> = vec![NORTH, NE, EAST, SE, SOUTH, SW, WEST, NW];

        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut mas_count = 0;

        while y < grid.height as i32 {
            while x < grid.width as i32 {
                // is it an A
                if grid.at(x, y).unwrap_or('.') == 'A' {
                    let mut firstDiag = false;
                    // check for S or M at NW
                    if grid.at(x + NW.x, y + NW.y).unwrap_or('.') == 'M' {
                        // find S at SE
                        if grid.at(x + SE.x, y + SE.y).unwrap_or('.') == 'S' {
                            // found one
                            firstDiag = true;
                        }
                    } else if grid.at(x + NW.x, y + NW.y).unwrap_or('.') == 'S' {
                        // find M at SE
                        if grid.at(x + SE.x, y + SE.y).unwrap_or('.') == 'M' {
                            // found one
                            firstDiag = true;
                        }
                    }

                    if firstDiag {
                        // check for S or M at NE
                        if grid.at(x + NE.x, y + NE.y).unwrap_or('.') == 'M' {
                            // find S at SW
                            if grid.at(x + SW.x, y + SW.y).unwrap_or('.') == 'S' {
                                // found one
                                println!("found at ({},{})", x, y);
                                mas_count = mas_count + 1;
                            }
                        } else if grid.at(x + NE.x, y + NE.y).unwrap_or('.') == 'S' {
                            // find M at SE
                            if grid.at(x + SW.x, y + SW.y).unwrap_or('.') == 'M' {
                                // found one
                                mas_count = mas_count + 1;
                            }
                        }
                    }
                }

                x = x + 1;
            }

            y = y + 1;
            x = 0;
        }

        assert_eq!(9, mas_count);
    }

    #[test]
    pub fn test_find() {
        let grid = Grid::new(TEST_DATA);

        let NORTH = Direction { x: -1, y: 0 };
        let NE: Direction = Direction { x: -1, y: 1 };
        let EAST: Direction = Direction { x: 0, y: 1 };
        let SE: Direction = Direction { x: 1, y: 1 };
        let SOUTH: Direction = Direction { x: 1, y: 0 };
        let SW: Direction = Direction { x: 1, y: -1 };
        let WEST: Direction = Direction { x: 0, y: -1 };
        let NW: Direction = Direction { x: -1, y: -1 };

        let directions: Vec<Direction> = vec![NORTH, NE, EAST, SE, SOUTH, SW, WEST, NW];

        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut xmas_count = 0;

        while y < grid.height as i32 {
            while x < grid.width as i32 {
                // is it an X
                if grid.at(x, y).unwrap_or('.') == 'X' {
                    for direction in directions.iter() {
                        let mut checkX = x + direction.x;
                        let mut checkY = y + direction.y;
                        if grid.at(checkX, checkY).unwrap_or('.') == 'M' {
                            checkX = checkX + direction.x;
                            checkY = checkY + direction.y;
                            if grid.at(checkX, checkY).unwrap_or('.') == 'A' {
                                checkX = checkX + direction.x;
                                checkY = checkY + direction.y;
                                if grid.at(checkX, checkY).unwrap_or('.') == 'S' {
                                    // found one
                                    println!("found at ({},{}), direction {:?}", x, y, direction);
                                    xmas_count = xmas_count + 1;
                                }
                            }
                        }
                    }
                }

                x = x + 1;
            }

            y = y + 1;
            x = 0;
        }

        assert_eq!(18, xmas_count);
    }
}
