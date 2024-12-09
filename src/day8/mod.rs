#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    use itertools::Itertools;

    use crate::common::{grid::Grid, string_utils::read_file_to_string};

    const TEST_DATA: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    pub fn test1() {
        let grid = Grid::new(&TEST_DATA);

        let mut x: i32 = 0;
        let mut y: i32 = 0;

        let mut antenas = HashMap::new();

        while y < grid.height as i32 {
            while x < grid.width as i32 {
                let c = grid.at(x, y).unwrap_or('.');
                if grid.at(x, y).unwrap_or('.') != '.' {
                    match antenas.get_mut(&c) {
                        None => {
                            antenas.insert(c, vec![(x, y)]);
                        }
                        Some(locations) => {
                            locations.push((x, y));
                        }
                    };
                }
                x = x + 1;
            }
            x = 0;
            y = y + 1;
        }

        println!("{:?}\n", antenas);

        let mut antinode_set = HashSet::new();
        // go through the groups and create the antinodes in a set (if they're in bounds)
        for (id, antena_list) in antenas.iter() {
            for pair in antena_list.into_iter().permutations(2) {
                // antinode locations
                let (firstx, firsty) = pair[0];
                let (secondx, secondy) = pair[1];

                let antix = (firstx - secondx);
                let antiy = (firsty - secondy);

                let antinode = (firstx + antix, firsty + antiy);
                println!(
                    "{} -> {:?} diff {:?} = {:?}",
                    id,
                    pair,
                    (antix, antiy),
                    antinode
                );

                if grid.at(antinode.0, antinode.1).is_some() {
                    antinode_set.insert(antinode);
                } else {
                    println!("{:?} is out of bounds", antinode);
                }
            }
        }

        println!("number of antinode {}", antinode_set.len());
    }

    const TEST_DATA_2: &str = r"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

    #[test]
    pub fn test2() {
        let grid = Grid::new(&TEST_DATA_2);

        let mut x: i32 = 0;
        let mut y: i32 = 0;

        let mut antenas = HashMap::new();

        while y < grid.height as i32 {
            while x < grid.width as i32 {
                let c = grid.at(x, y).unwrap_or('.');
                if grid.at(x, y).unwrap_or('.') != '.' {
                    match antenas.get_mut(&c) {
                        None => {
                            antenas.insert(c, vec![(x, y)]);
                        }
                        Some(locations) => {
                            locations.push((x, y));
                        }
                    };
                }
                x = x + 1;
            }
            x = 0;
            y = y + 1;
        }

        println!("{:?}\n", antenas);

        let mut antinode_set = HashSet::new();
        // go through the groups and create the antinodes in a set (if they're in bounds)
        for (id, antena_list) in antenas.iter() {
            for pair in antena_list.into_iter().permutations(2) {
                // antinode locations
                let (firstx, firsty) = pair[0];
                let (secondx, secondy) = pair[1];

                let antix = (firstx - secondx);
                let antiy = (firsty - secondy);

                let mut antinode = (firstx + antix, firsty + antiy);
                println!(
                    "{} -> {:?} diff {:?} = {:?}",
                    id,
                    pair,
                    (antix, antiy),
                    antinode
                );
                // always insert first
                antinode_set.insert((*firstx, *firsty));
                while grid.at(antinode.0, antinode.1).is_some() {
                    antinode_set.insert(antinode);

                    antinode = (antinode.0 + antix, antinode.1 + antiy);

                    println!(
                        "{} -> {:?} diff {:?} = {:?}",
                        id,
                        pair,
                        (antix, antiy),
                        antinode
                    );
                }
                println!("{:?} out of bounds", antinode);
            }
        }

        println!("number of antinode {}", antinode_set.len());
    }

    #[test]
    pub fn test_full() {
        let input = read_file_to_string("input/day8.txt");
        let grid = Grid::new(&input);

        let mut x: i32 = 0;
        let mut y: i32 = 0;

        let mut antenas = HashMap::new();

        while y < grid.height as i32 {
            while x < grid.width as i32 {
                let c = grid.at(x, y).unwrap_or('.');
                if grid.at(x, y).unwrap_or('.') != '.' {
                    match antenas.get_mut(&c) {
                        None => {
                            antenas.insert(c, vec![(x, y)]);
                        }
                        Some(locations) => {
                            locations.push((x, y));
                        }
                    };
                }
                x = x + 1;
            }
            x = 0;
            y = y + 1;
        }

        println!("{:?}", antenas);

        let mut antinode_set = HashSet::new();
        // go through the groups and create the antinodes in a set (if they're in bounds)
        for (id, antena_list) in antenas.iter() {
            for pair in antena_list.into_iter().permutations(2) {
                // antinode locations
                let (firstx, firsty) = pair[0];
                let (secondx, secondy) = pair[1];

                let antix = (firstx - secondx);
                let antiy = (firsty - secondy);

                let antinode = (firstx + antix, firsty + antiy);
                println!(
                    "{} -> {:?} diff {:?} = {:?}",
                    id,
                    pair,
                    (antix, antiy),
                    antinode
                );

                if grid.at(antinode.0, antinode.1).is_some() {
                    antinode_set.insert(antinode);
                } else {
                    println!("{:?} is out of bounds", antinode);
                }
            }
        }

        println!("number of antinode {}", antinode_set.len());
    }

    #[test]
    pub fn test_full2() {
        let input = read_file_to_string("input/day8.txt");
        let grid = Grid::new(&input);

        let mut x: i32 = 0;
        let mut y: i32 = 0;

        let mut antenas = HashMap::new();

        while y < grid.height as i32 {
            while x < grid.width as i32 {
                let c = grid.at(x, y).unwrap_or('.');
                if grid.at(x, y).unwrap_or('.') != '.' {
                    match antenas.get_mut(&c) {
                        None => {
                            antenas.insert(c, vec![(x, y)]);
                        }
                        Some(locations) => {
                            locations.push((x, y));
                        }
                    };
                }
                x = x + 1;
            }
            x = 0;
            y = y + 1;
        }

        println!("{:?}\n", antenas);

        let mut antinode_set = HashSet::new();
        // go through the groups and create the antinodes in a set (if they're in bounds)
        for (id, antena_list) in antenas.iter() {
            for pair in antena_list.into_iter().permutations(2) {
                // antinode locations
                let (firstx, firsty) = pair[0];
                let (secondx, secondy) = pair[1];

                let antix = (firstx - secondx);
                let antiy = (firsty - secondy);

                let mut antinode = (firstx + antix, firsty + antiy);
                println!(
                    "{} -> {:?} diff {:?} = {:?}",
                    id,
                    pair,
                    (antix, antiy),
                    antinode
                );
                // always insert first
                antinode_set.insert((*firstx, *firsty));
                while grid.at(antinode.0, antinode.1).is_some() {
                    antinode_set.insert(antinode);

                    antinode = (antinode.0 + antix, antinode.1 + antiy);

                    println!(
                        "{} -> {:?} diff {:?} = {:?}",
                        id,
                        pair,
                        (antix, antiy),
                        antinode
                    );
                }
                println!("{:?} out of bounds", antinode);
            }
        }

        println!("number of antinode {}", antinode_set.len());
    }
}
