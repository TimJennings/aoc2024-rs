use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{map, opt},
    multi::many0,
    sequence::{terminated, tuple},
    IResult,
};

use crate::common::{nom_utils::parser_i64, string_utils::read_file_to_string};

#[derive(Debug)]
struct Robot {
    initial_postition: (i64, i64),
    vector: (i64, i64),
}

#[derive(Copy, Clone, Debug)]
struct Quad {
    start_x: i64,
    end_x: i64,
    start_y: i64,
    end_y: i64,
    robot_count: i64,
}

pub fn run() {
    let input = read_file_to_string("input/day14.txt");
    let mut robots = many0(parse_robot)(input.as_str()).unwrap().1;
    println!("{:?}", robots);

    let mut quad1 = Quad {
        start_x: 0,
        end_x: 49,
        start_y: 0,
        end_y: 50,
        robot_count: 0,
    };

    let mut quad2 = Quad {
        start_x: 51,
        end_x: 101,
        start_y: 0,
        end_y: 51,
        robot_count: 0,
    };

    let mut quad3 = Quad {
        start_x: 0,
        end_x: 49,
        start_y: 52,
        end_y: 103,
        robot_count: 0,
    };

    let mut quad4 = Quad {
        start_x: 51,
        end_x: 101,
        start_y: 52,
        end_y: 103,
        robot_count: 0,
    };

    let mut quads = vec![&mut quad1, &mut quad2, &mut quad3, &mut quad4];

    let width = 101;
    let height = 103;
    let iterations = 100;
    for robot in robots.iter_mut() {
        // multiple its vec 100 times and add to starting pos, then mod by size to get current position

        let mut new_position = (
            ((robot.vector.0 * iterations) + robot.initial_postition.0) % width,
            ((robot.vector.1 * iterations) + robot.initial_postition.1) % height,
        );

        if new_position.0 < 0 {
            new_position.0 = width + new_position.0;
        }

        if new_position.1 < 0 {
            new_position.1 = height + new_position.1;
        }
        println!(
            "robot moved from {:?} to {:?}",
            robot.initial_postition, new_position
        );

        // count quad

        if new_position.0 == 50 || new_position.1 == 51 {
            continue;
        }
        for quad in quads.iter_mut() {
            if new_position.0 >= quad.start_x
                && new_position.0 <= quad.end_x
                && new_position.1 >= quad.start_y
                && new_position.1 <= quad.end_y
            {
                // in quad
                quad.robot_count = quad.robot_count + 1;
                // println!("Robot in quad {:?}", quad);
            }
        }
    }

    // count robots in each quarter
    println!("{:?}", quads);
    let safety_factor =
        quad1.robot_count * quad2.robot_count * quad3.robot_count * quad4.robot_count;
    println!("Safety Factor: {}", safety_factor);
}

pub fn run2() {
    let input = read_file_to_string("input/day14.txt");
    let mut robots = many0(parse_robot)(input.as_str()).unwrap().1;
    println!("{:?}", robots);

    let mut danger = Vec::new();
    for i in 8005..=8007 {
        let mut quad1 = Quad {
            start_x: 0,
            end_x: 49,
            start_y: 0,
            end_y: 50,
            robot_count: 0,
        };

        let mut quad2 = Quad {
            start_x: 51,
            end_x: 101,
            start_y: 0,
            end_y: 51,
            robot_count: 0,
        };

        let mut quad3 = Quad {
            start_x: 0,
            end_x: 49,
            start_y: 52,
            end_y: 103,
            robot_count: 0,
        };

        let mut quad4 = Quad {
            start_x: 51,
            end_x: 101,
            start_y: 52,
            end_y: 103,
            robot_count: 0,
        };

        let mut quads = vec![&mut quad1, &mut quad2, &mut quad3, &mut quad4];

        let width = 101;
        let height = 103;
        let iterations = i;
        let mut image = HashSet::new();
        // print!("Iteration {}", i);
        for robot in robots.iter_mut() {
            // multiple its vec 100 times and add to starting pos, then mod by size to get current position

            let mut new_position = (
                ((robot.vector.0 * iterations) + robot.initial_postition.0) % width,
                ((robot.vector.1 * iterations) + robot.initial_postition.1) % height,
            );

            if new_position.0 < 0 {
                new_position.0 = width + new_position.0;
            }

            if new_position.1 < 0 {
                new_position.1 = height + new_position.1;
            }
            image.insert(new_position);
            // println!(
            //     "robot moved from {:?} to {:?}",
            //     robot.initial_postition, new_position
            // );

            // count quad

            if new_position.0 == 50 || new_position.1 == 51 {
                continue;
            }
            for quad in quads.iter_mut() {
                if new_position.0 >= quad.start_x
                    && new_position.0 <= quad.end_x
                    && new_position.1 >= quad.start_y
                    && new_position.1 <= quad.end_y
                {
                    // in quad
                    quad.robot_count = quad.robot_count + 1;
                    // println!("Robot in quad {:?}", quad);
                }
            }
        }

        // count robots in each quarter
        // println!("{:?}", quads);
        let safety_factor =
            quad1.robot_count * quad2.robot_count * quad3.robot_count * quad4.robot_count;

        // count robots in the middle
        let centre_count = image
            .clone()
            .into_iter()
            .filter(|r| r.0 > 45 && r.0 < 55 && r.1 > 45 && r.1 < 55)
            .count();

        let x_group = image.clone().into_iter().into_group_map_by(|r| r.0);
        // find largest x
        let max_x = x_group.into_iter().max_by_key(|x| x.1.len());

        danger.push((i, max_x));
        // if (centre_count > 10) {
        // maybe found one
        println!("Maybe iteration {}", i);
        for y in 0..height {
            for x in 0..width {
                if image.contains(&(x, y)) {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            print!("\n");
        }
        print!("\n");
        //     // break;
        // }
        // println!("Safety Factor: {}", safety_factor);
        // danger.push((i, safety_factor));
    }

    let max = danger
        .into_iter()
        .max_by_key(|s| s.1.clone().unwrap().1.len());
    println!("{:?}", max);
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let parse = terminated(
        map(
            tuple((
                tag("p="),
                parser_i64,
                tag(","),
                parser_i64,
                tag(" v="),
                parser_i64,
                tag(","),
                parser_i64,
            )),
            |(_, p1, _, p2, _, v1, _, v2)| Robot {
                initial_postition: (p1, p2),
                vector: (v1, v2),
            },
        ),
        opt(line_ending),
    )(input);

    parse
}

#[cfg(test)]
mod test {
    use nom::multi::many0;

    use crate::day14::{parse_robot, Quad, Robot};

    const TEST_DATA: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    pub fn test() {
        let mut robots = many0(parse_robot)(TEST_DATA).unwrap().1;
        println!("{:?}", robots);

        let mut quad1 = Quad {
            start_x: 0,
            end_x: 4,
            start_y: 0,
            end_y: 2,
            robot_count: 0,
        };

        let mut quad2 = Quad {
            start_x: 6,
            end_x: 10,
            start_y: 0,
            end_y: 2,
            robot_count: 0,
        };

        let mut quad3 = Quad {
            start_x: 0,
            end_x: 4,
            start_y: 4,
            end_y: 6,
            robot_count: 0,
        };

        let mut quad4 = Quad {
            start_x: 6,
            end_x: 10,
            start_y: 4,
            end_y: 6,
            robot_count: 0,
        };

        let mut quads = vec![&mut quad1, &mut quad2, &mut quad3, &mut quad4];

        let width = 11;
        let height = 7;
        let iterations = 100;

        for robot in robots.iter_mut() {
            // multiple its vec 100 times and add to starting pos, then mod by size to get current position

            let mut new_position = (
                ((robot.vector.0 * iterations) + robot.initial_postition.0) % width,
                ((robot.vector.1 * iterations) + robot.initial_postition.1) % height,
            );

            if new_position.0 < 0 {
                new_position.0 = width + new_position.0;
            }

            if new_position.1 < 0 {
                new_position.1 = height + new_position.1;
            }
            println!(
                "robot moved from {:?} to {:?}",
                robot.initial_postition, new_position
            );

            // count quad

            for quad in quads.iter_mut() {
                if new_position.0 >= quad.start_x
                    && new_position.0 <= quad.end_x
                    && new_position.1 >= quad.start_y
                    && new_position.1 <= quad.end_y
                {
                    // in quad
                    quad.robot_count = quad.robot_count + 1;
                    // println!("Robot in quad {:?}", quad);
                }
            }
        }

        // count robots in each quarter
        println!("{:?}", quads);
        let safety_factor =
            quad1.robot_count * quad2.robot_count * quad3.robot_count * quad4.robot_count;
        println!("Safety Factor: {}", safety_factor);
    }

    #[test]
    pub fn test2() {
        let mut robots = vec![Robot {
            initial_postition: (2, 4),
            vector: (2, -3),
        }];
        println!("{:?}", robots);

        let width = 11;
        let height = 7;
        let iterations = 5;
        for robot in robots.iter_mut() {
            // multiple its vec 100 times and add to starting pos, then mod by size to get current position

            let mut newPosition = (
                ((robot.vector.0 * iterations) + robot.initial_postition.0) % width,
                ((robot.vector.1 * iterations) + robot.initial_postition.1) % height,
            );

            if newPosition.0 < 0 {
                newPosition.0 = width + newPosition.0;
            }

            if newPosition.1 < 0 {
                newPosition.1 = height + newPosition.1;
            }
            println!(
                "robot moved from {:?} to {:?}",
                robot.initial_postition, newPosition
            );
        }
    }
}
