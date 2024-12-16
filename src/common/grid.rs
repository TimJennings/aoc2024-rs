use std::fmt::{Display, Write};

use super::string_utils;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    grid: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Display for Grid<T>
where
    T: PartialEq<T> + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                self.at(x as i32, y as i32).unwrap().fmt(f);
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

pub fn parse_string_grid(input: &str) -> (Vec<char>, usize, usize) {
    let vecI = string_utils::string_to_vec(input);
    let mut vecC = Vec::new();
    for s in vecI.iter() {
        for c in s.chars() {
            vecC.push(c);
        }
    }

    let width = vecI[0].len();
    let height = vecI.len();
    (vecC, width, height)
}

impl<T> Grid<T>
where
    T: PartialEq<T>,
{
    pub fn new(input: Vec<T>, width: usize, height: usize) -> Self {
        Grid {
            grid: input,
            height: height,
            width: width,
        }
    }

    pub fn set(&mut self, x: i32, y: i32, value: T) {
        self.grid[(y as usize * self.width) + x as usize] = value;
    }

    pub fn at(&self, x: i32, y: i32) -> Option<&T> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return None;
        } else {
            return Some(&self.grid[(y as usize * self.width) + x as usize]);
        }
    }

    pub fn find_first(&self, to_find: &T) -> Option<(i32, i32)> {
        // walk the grid until we find the character
        let mut index = None;
        for (i, item) in self.grid.iter().enumerate() {
            if item == to_find {
                index = Some(i);
                break;
            }
        }

        match index {
            None => None,
            Some(a) => {
                let x = a / self.width;
                let y = a % self.width;
                Some((y as i32, x as i32))
            }
        }
    }
}
