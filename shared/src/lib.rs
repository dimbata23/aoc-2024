use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Add;
use std::ops::Sub;

pub fn parse_2d_map(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    Ok(File::open(file_path)
        .map(io::BufReader::new)?
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.trim().chars().collect())
        .collect())
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Pos2D<T> {
    pub row: T,
    pub col: T,
}

impl<T> Pos2D<T> {
    pub fn new(row: T, col: T) -> Self {
        Self { row, col }
    }

    pub fn to_vec(self) -> Vec2D<T> {
        Vec2D {
            x: self.row,
            y: self.col,
        }
    }
}

impl<T> Pos2D<T>
where
    T: Sub<Output = T>,
{
    pub fn make_vec_to(self, other: Self) -> Vec2D<T> {
        Vec2D::from_pos(other - self)
    }
}

impl<T> Sub for Pos2D<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn from_pos(pos: Pos2D<T>) -> Self {
        Self {
            x: pos.row,
            y: pos.col,
        }
    }

    pub fn to_pos(self) -> Pos2D<T> {
        Pos2D::new(self.x, self.y)
    }
}

impl<T> Add for Vec2D<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Add<Pos2D<T>> for Vec2D<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, pos: Pos2D<T>) -> Self::Output {
        Self {
            x: self.x + pos.row,
            y: self.y + pos.col,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
