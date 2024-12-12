use crate::common::grid::Direction;

pub fn run() {}

const NORTH: Direction = Direction { x: 0, y: -1 };
const NORTH_EAST: Direction = Direction { x: 1, y: -1 };
const EAST: Direction = Direction { x: 1, y: 0 };
const SOUTH_EAST: Direction = Direction { x: 1, y: 1 };
const SOUTH: Direction = Direction { x: 0, y: 1 };
const SOUTH_WEST: Direction = Direction { x: -1, y: 1 };
const WEST: Direction = Direction { x: -1, y: 0 };
const NORTH_WEST: Direction = Direction { x: -1, y: -1 };

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    use itertools::Itertools;

    use crate::{
        common::{
            grid::{self, Direction, Grid},
            string_utils::read_file_to_string,
        },
        day12::{EAST, NORTH, NORTH_EAST, NORTH_WEST, SOUTH, SOUTH_EAST, SOUTH_WEST, WEST},
    };

    const TEST_DATA_1: &str = r"AAAA
BBCD
BBCC
EEEC";

    const TEST_DATA_2: &str = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const TEST_DATA_3: &str = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const TEST_DATA_4: &str = r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const TEST_DATA_5: &str = r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    type AreaMap = HashMap<(char, (i32, i32)), (u32, u32)>;

    #[test]
    pub fn test() {
        let input = read_file_to_string("input/day12.txt");
        let grid = grid::Grid::new(input.as_str());

        // char,x,y to area_id,fence_count
        let mut areas: HashMap<(char, (i32, i32)), (u32, u32)> = HashMap::new();

        let mut next_id = 0;
        // work through the area, for each item check if a region of the same char is nearby and join it, else create one
        for y in 0..grid.height {
            for x in 0..grid.width {
                let x = x as i32;
                let y = y as i32;
                let current_char = grid.at(x, y).unwrap();

                check_for_region(&mut areas, &grid, current_char, x, y, &mut next_id);
            }
        }

        // walk through again and check how many areas around you aren't in your group, that's the number of fenses needed
        for y in 0..grid.height {
            for x in 0..grid.width {
                let x = x as i32;
                let y = y as i32;
                let current_char = grid.at(x, y).unwrap();

                let (current_area_id, _current_fences) =
                    areas.get(&(current_char, (x, y))).unwrap();

                let mut fences: u32 = 4;
                for direction in [NORTH, EAST, SOUTH, WEST] {
                    if areas
                        .get((&(current_char, (x + direction.x, y + direction.y))))
                        .is_some_and(|(area_id, _fences)| area_id == current_area_id)
                    {
                        // same region id
                        fences = fences - 1;
                    }
                }
                areas.insert((current_char, (x, y)), (*current_area_id, fences));
            }
        }

        let areas_flat = areas
            .clone()
            .into_iter()
            .into_group_map_by(|area| area.1 .0);

        // collapse regions
        // println!("{:?}\n", grid);
        // println!("{:?}\n", areas);
        // println!("{:?}", areas_flat);

        let mut price = 0;
        for (area_id, grid_items) in areas_flat.iter() {
            let area_size = grid_items.len();

            let fence_count: u32 = grid_items.iter().map(|g| g.1 .1).sum();
            println!(
                "Area {} is {} tiles of {} with {} fences",
                area_id, area_size, grid_items[0].0 .0, fence_count
            );

            price = price + (area_size as u32 * fence_count);
        }

        println!("price {}", price);
    }

    #[test]
    pub fn test2() {
        let input = read_file_to_string("input/day12.txt");
        let grid = grid::Grid::new(input.as_str());

        // char,x,y to area_id,fence_count
        let mut areas: HashMap<(char, (i32, i32)), (u32, u32)> = HashMap::new();

        let mut next_id = 0;
        // work through the area, for each item check if a region of the same char is nearby and join it, else create one
        for y in 0..grid.height {
            for x in 0..grid.width {
                let x = x as i32;
                let y = y as i32;
                let current_char = grid.at(x, y).unwrap();

                check_for_region(&mut areas, &grid, current_char, x, y, &mut next_id);
            }
        }

        // walk through again and check if you are surrounded by more out of group that in group
        for y in 0..grid.height {
            for x in 0..grid.width {
                let x = x as i32;
                let y = y as i32;
                let current_char = grid.at(x, y).unwrap();

                let (current_area_id, _current_fences) =
                    areas.get(&(current_char, (x, y))).unwrap();

                let mut corners: u32 = 0;

                // corner 1
                if is_all_direction_different_area(
                    current_char,
                    *current_area_id,
                    x,
                    y,
                    &[NORTH, WEST],
                    &areas,
                ) {
                    println!("({},{}) corner 1", x, y);
                    corners = corners + 1;
                }

                // corner 2
                if is_all_direction_different_area(
                    current_char,
                    *current_area_id,
                    x,
                    y,
                    &[WEST, SOUTH],
                    &areas,
                ) {
                    println!("({},{}) corner 2", x, y);
                    corners = corners + 1;
                }

                // corner 3
                if is_all_direction_different_area(
                    current_char,
                    *current_area_id,
                    x,
                    y,
                    &[NORTH, EAST],
                    &areas,
                ) {
                    println!("({},{}) corner 3", x, y);
                    corners = corners + 1;
                }

                // corner 4
                if is_all_direction_different_area(
                    current_char,
                    *current_area_id,
                    x,
                    y,
                    &[EAST, SOUTH],
                    &areas,
                ) {
                    println!("({},{}) corner 4", x, y);
                    corners = corners + 1;
                }

                // corner 5
                if is_all_direction_different_area(
                    current_char,
                    *current_area_id,
                    x,
                    y,
                    &[EAST],
                    &areas,
                ) && is_all_direction_same_area(
                    current_char,
                    *current_area_id,
                    x,
                    y,
                    &[SOUTH, SOUTH_EAST],
                    &areas,
                ) {
                    println!("({},{}) corner 5", x, y);
                    corners = corners + 1;
                }

                // corner 6
                if is_all_direction_different_area(
                    current_char,
                    *current_area_id,
                    x,
                    y,
                    &[WEST],
                    &areas,
                ) && is_all_direction_same_area(
                    current_char,
                    *current_area_id,
                    x,
                    y,
                    &[SOUTH, SOUTH_WEST],
                    &areas,
                ) {
                    println!("({},{}) corner 6", x, y);
                    corners = corners + 1;
                }

                // corner 7
                if is_all_direction_different_area(
                    current_char,
                    *current_area_id,
                    x,
                    y,
                    &[SOUTH_EAST],
                    &areas,
                ) && is_all_direction_same_area(
                    current_char,
                    *current_area_id,
                    x,
                    y,
                    &[EAST, SOUTH],
                    &areas,
                ) {
                    println!("({},{}) corner 7", x, y);
                    corners = corners + 1;
                }

                // corner 8
                if is_all_direction_different_area(
                    current_char,
                    *current_area_id,
                    x,
                    y,
                    &[SOUTH_WEST],
                    &areas,
                ) && is_all_direction_same_area(
                    current_char,
                    *current_area_id,
                    x,
                    y,
                    &[WEST, SOUTH],
                    &areas,
                ) {
                    println!("({},{}) corner 8", x, y);
                    corners = corners + 1;
                }
                areas.insert((current_char, (x, y)), (*current_area_id, corners));
            }
        }

        let areas_flat = areas
            .clone()
            .into_iter()
            .into_group_map_by(|area| area.1 .0);

        // collapse regions
        // println!("{:?}\n", grid);
        // println!("{:?}\n", areas);
        // println!("{:?}", areas_flat);

        let mut price = 0;
        for (area_id, grid_items) in areas_flat.iter() {
            let area_size = grid_items.len();

            let fence_count: u32 = grid_items.iter().map(|g| g.1 .1).sum();
            println!(
                "Area {} is {} tiles of {} with {} fences",
                area_id, area_size, grid_items[0].0 .0, fence_count
            );

            price = price + (area_size as u32 * fence_count);
        }

        println!("price {}", price);
    }

    fn is_all_direction_same_area(
        current_char: char,
        current_area_id: u32,
        x: i32,
        y: i32,
        directions: &[Direction],
        areas: &AreaMap,
    ) -> bool {
        directions.into_iter().all(|direction| {
            areas
                .get((&(current_char, (x + direction.x, y + direction.y))))
                .is_some_and(|(area_id, _fences)| *area_id == current_area_id)
        })
    }

    fn is_all_direction_different_area(
        current_char: char,
        current_area_id: u32,
        x: i32,
        y: i32,
        directions: &[Direction],
        areas: &AreaMap,
    ) -> bool {
        directions.into_iter().all(|direction| {
            !areas
                .get((&(current_char, (x + direction.x, y + direction.y))))
                .is_some_and(|(area_id, _fences)| *area_id == current_area_id)
        })
    }

    fn check_for_region(
        areas: &mut HashMap<(char, (i32, i32)), (u32, u32)>,
        grid: &Grid,
        current_char: char,
        x: i32,
        y: i32,
        next_id: &mut u32,
    ) {
        if areas.contains_key(&(current_char, (x, y))) {
            // im in a grid, stop
            return;
        } else {
            // not in a grid create one and walk
            areas.insert((current_char, (x, y)), (*next_id, 0));
            *next_id = *next_id + 1;

            let area_id = areas.get(&(current_char, (x, y))).unwrap().0;

            for direction in [NORTH, EAST, SOUTH, WEST] {
                let direction_char = grid.at(x + direction.x, y + direction.y).unwrap_or(',');

                if direction_char == current_char {
                    if !areas.contains_key(&(current_char, (x + direction.x, y + direction.y))) {
                        // untagged, tag it
                        areas.insert(
                            (direction_char, (x + direction.x, y + direction.y)),
                            (area_id, 0),
                        );
                        // walk it
                        walk_region(
                            area_id,
                            current_char,
                            x + direction.x,
                            y + direction.y,
                            areas,
                            grid,
                        );
                    }
                }
            }
        }
    }
    fn walk_region(
        area_id: u32,
        current_char: char,
        x: i32,
        y: i32,
        areas: &mut HashMap<(char, (i32, i32)), (u32, u32)>,
        grid: &Grid,
    ) {
        for direction in [NORTH, EAST, SOUTH, WEST] {
            let direction_char = grid.at(x + direction.x, y + direction.y).unwrap_or(',');

            if direction_char == current_char {
                // that area isn't already tagged
                if !areas.contains_key(&(current_char, (x + direction.x, y + direction.y))) {
                    // matching, tag it
                    areas.insert(
                        (direction_char, (x + direction.x, y + direction.y)),
                        (area_id, 0),
                    );
                    // walk it
                    walk_region(
                        area_id,
                        current_char,
                        x + direction.x,
                        y + direction.y,
                        areas,
                        grid,
                    );
                }
            }
        }
    }
}
