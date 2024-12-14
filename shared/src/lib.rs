use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::{Add, RemAssign, Sub};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Hash, Eq, PartialEq, Copy, Clone, EnumIter)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(
    Copy,
    Clone,
    Hash,
    Eq,
    PartialEq,
    Debug,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

#[derive(
    Copy,
    Clone,
    Hash,
    Eq,
    PartialEq,
    Debug,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
)]
pub struct Pos2D<T> {
    pub row: T,
    pub col: T,
}

pub fn determinant<T>(col1: Vec2D<T>, col2: Vec2D<T>) -> T
where
    T: std::ops::Mul<Output = T> + std::ops::Sub<Output = T>,
{
    col1.x * col2.y - col2.x * col1.y
}

pub fn intersect_vecs<T>(vec1: &Vec<T>, vec2: &Vec<T>) -> Vec<T>
where
    T: PartialEq + Clone,
{
    vec1.iter().filter(|x| vec2.contains(x)).cloned().collect()
}

pub fn parse_2d_map(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    Ok(File::open(file_path)
        .map(io::BufReader::new)?
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.trim().chars().collect())
        .collect())
}

impl Dir {
    pub fn opposite(self) -> Self {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

pub fn hashset_dirs_to_vec(set: &HashSet<Dir>) -> Vec<bool> {
    Dir::iter().map(|dir| set.contains(&dir)).collect()
}

pub fn dir_in(vec_set: &[bool], dir: Dir) -> bool {
    vec_set[dir as usize]
}

impl<T> Pos2D<T> {
    pub const fn new(row: T, col: T) -> Self {
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
    T: TryFrom<i32> + Sub<Output = T>,
    <T as TryFrom<i32>>::Error: Debug,
{
    pub fn left(self) -> Self {
        Self {
            row: self.row,
            col: self.col - 1.try_into().unwrap(),
        }
    }

    pub fn up(self) -> Self {
        Self {
            row: self.row - 1.try_into().unwrap(),
            col: self.col,
        }
    }
}

impl<T> Pos2D<T>
where
    T: TryFrom<i32> + Add<Output = T>,
    <T as TryFrom<i32>>::Error: Debug,
{
    pub fn right(self) -> Self {
        Self {
            row: self.row,
            col: self.col + 1.try_into().unwrap(),
        }
    }

    pub fn down(self) -> Self {
        Self {
            row: self.row + 1.try_into().unwrap(),
            col: self.col,
        }
    }
}

impl<T> Pos2D<T>
where
    T: TryFrom<i32> + Add<Output = T> + Sub<Output = T>,
    <T as TryFrom<i32>>::Error: Debug,
{
    pub fn moved(self, dir: Dir) -> Self {
        match dir {
            Dir::Up => self.up(),
            Dir::Down => self.down(),
            Dir::Left => self.left(),
            Dir::Right => self.right(),
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

impl<T> Vec2D<T> {
    pub const fn new(x: T, y: T) -> Self {
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

impl<T> std::ops::Rem for Vec2D<T>
where
    T: std::ops::Rem<Output = T>,
    Vec2D<T>: std::ops::RemAssign,
{
    type Output = Vec2D<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        let mut new = self;
        new.rem_assign(rhs);
        new
    }
}

impl<T> std::ops::RemAssign for Vec2D<T>
where
    T: std::ops::RemAssign,
{
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}
