use super::string_utils;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Grid {
    grid: String,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(mut input: &str) -> Self {
        let vecI = string_utils::string_to_vec(input);

        let width = vecI[0].len();
        let height = vecI.len();
        Grid {
            grid: vecI.join(""),
            height: height,
            width: width,
        }
    }

    pub fn set(&mut self, x: i32, y: i32, value: char) {
        self.grid.replace_range(
            (y as usize * self.width) + x as usize
                ..(y as usize * self.width) + x as usize + 1 as usize,
            &value.to_string(),
        );
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

    pub fn find_first(&self, to_find: char) -> Option<(i32, i32)> {
        // walk the grid until we find the character
        let found = self.grid.find(to_find);

        match found {
            None => None,
            Some(a) => {
                let x = a / self.width;
                let y = a % self.width;
                Some((y as i32, x as i32))
            }
        }
    }
}
