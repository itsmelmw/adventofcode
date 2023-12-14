use std::{ops::{Add, Sub}, iter::StepBy, slice::Iter};

use num::Integer;

pub type UPoint = Point<usize>;
pub type IPoint = Point<isize>;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Integer + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    pub fn dir_steps(&self, dir: &Dir, steps: T) -> Self {
        match dir {
            Dir::Up => Point::new(self.x, self.y - steps),
            Dir::Right => Point::new(self.x + steps, self.y),
            Dir::Down => Point::new(self.x, self.y + steps),
            Dir::Left => Point::new(self.x - steps, self.y),
        }
    }
    pub fn dir(&self, dir: &Dir) -> Self {
        self.dir_steps(dir, T::one())
    }
    pub fn dir_wide(&self, dir: &Dir) -> [Self; 3] {
        match dir {
            Dir::Up => [
                Point::new(self.x - T::one(), self.y - T::one()),
                Point::new(self.x, self.y - T::one()),
                Point::new(self.x + T::one(), self.y - T::one()),
            ],
            Dir::Right => [
                Point::new(self.x + T::one(), self.y - T::one()),
                Point::new(self.x + T::one(), self.y),
                Point::new(self.x + T::one(), self.y + T::one()),
            ],
            Dir::Down => [
                Point::new(self.x - T::one(), self.y + T::one()),
                Point::new(self.x, self.y + T::one()),
                Point::new(self.x + T::one(), self.y + T::one()),
            ],
            Dir::Left => [
                Point::new(self.x - T::one(), self.y - T::one()),
                Point::new(self.x - T::one(), self.y),
                Point::new(self.x - T::one(), self.y + T::one()),
            ],
        }
    }
    pub fn neighbors_8(&self) -> [Self; 8] {
        [
            Point::new(self.x - T::one(), self.y - T::one()),
            Point::new(self.x, self.y - T::one()),
            Point::new(self.x + T::one(), self.y - T::one()),
            Point::new(self.x - T::one(), self.y),
            Point::new(self.x + T::one(), self.y),
            Point::new(self.x - T::one(), self.y + T::one()),
            Point::new(self.x, self.y + T::one()),
            Point::new(self.x + T::one(), self.y + T::one()),
        ]
    }
    pub fn neighbors_4(&self) -> [Self; 4] {
        [
            Point::new(self.x, self.y - T::one()),
            Point::new(self.x - T::one(), self.y),
            Point::new(self.x + T::one(), self.y),
            Point::new(self.x, self.y + T::one()),
        ]
    }
    pub fn neighbors_4_in(&self, width: T, height: T) -> Vec<Self> {
        let mut positions = Vec::new();
        if self.x != T::zero() {
            positions.push(Point::new(self.x - T::one(), self.y));
        }
        if self.x != width - T::one() {
            positions.push(Point::new(self.x + T::one(), self.y));
        }
        if self.y != T::zero() {
            positions.push(Point::new(self.x, self.y - T::one()));
        }
        if self.y != height - T::one() {
            positions.push(Point::new(self.x, self.y + T::one()));
        }
        positions
    }
}

impl<T> Add for Point<T>
where
    T: Integer + Copy,
{
    type Output = Point<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Sub for Point<T>
where
    T: Integer + Copy,
{
    type Output = Point<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

pub enum TurnDir {
    Right,
    Left,
}

impl Dir {
    pub fn opposite(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
        }
    }
    pub fn clockwise(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
    pub fn counter_clockwise(&self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
        }
    }
    pub fn turn(&self, turn: TurnDir) -> Dir {
        match turn {
            TurnDir::Right => self.clockwise(),
            TurnDir::Left => self.counter_clockwise(),
        }
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    vec: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn from_vec(vec: Vec<T>, width: usize) -> Self {
        Self { vec, width }
    }
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.vec[x + y * self.width]
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.vec.len() / self.width
    }
    pub fn iter(&self) -> Iter<T> {
        self.vec.iter()
    }
    pub fn iter_row(&self, row: usize) -> StepBy<Iter<T>> {
        if row >= self.height() {
            panic!("Tried to access row {} while the grid has {} rows", row, self.height());
        }
        let start_idx = row * self.width;
        self.vec[start_idx..start_idx + self.width].iter().step_by(1)
    }
    pub fn iter_col(&self, col: usize) -> StepBy<Iter<T>> {
        if col >= self.width() {
            panic!("Tried to access column {} while the grid has {} columns", col, self.width());
        }
        self.vec[col..].iter().step_by(self.width)
    }
    pub fn iter_rows(&self) -> GridRows<T>{
        GridRows { grid: self, idx: 0, idx_back: self.height() }
    }
    pub fn iter_cols(&self) -> GridCols<T>{
        GridCols { grid: self, idx: 0, idx_back: self.width() }
    }
    pub fn transposed(&self) -> Self 
    where
        T: Clone + Copy,
    {
        let vec = self.iter_cols().flatten().copied().collect::<Vec<T>>();
        let width = self.height();
        Grid { vec, width }
    }
    pub fn as_vec(&self) -> &Vec<T> {
        &self.vec
    }
}

pub struct GridRows<'a, T> {
    grid: &'a Grid<T>,
    idx: usize,
    idx_back: usize,
}

impl<'a, T> Iterator for GridRows<'a, T> {
    type Item = StepBy<Iter<'a, T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx + 1 > self.idx_back {
            return None;
        }
        let row = self.grid.iter_row(self.idx);
        self.idx += 1;
        Some(row)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.idx_back - self.idx;
        (size, Some(size))
    }
}

impl<'a, T> DoubleEndedIterator for GridRows<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx_back < self.idx + 1 {
            return None;
        }
        self.idx_back -= 1;
        let row = self.grid.iter_row(self.idx_back);
        Some(row)
    }
}

impl<'a, T> ExactSizeIterator for GridRows<'a, T> {}

pub struct GridCols<'a, T> {
    grid: &'a Grid<T>,
    idx: usize,
    idx_back: usize,
}

impl<'a, T> Iterator for GridCols<'a, T> {
    type Item = StepBy<Iter<'a, T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx + 1 > self.idx_back {
            return None;
        } 
        let col = self.grid.iter_col(self.idx);
        self.idx += 1;
        Some(col)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.idx_back - self.idx;
        (size, Some(size))
    }
}

impl<'a, T> DoubleEndedIterator for GridCols<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx_back < self.idx + 1 {
            return None;
        }
        self.idx_back -= 1;
        let col = self.grid.iter_col(self.idx_back);
        Some(col)
    }
}

impl<'a, T> ExactSizeIterator for GridCols<'a, T> {}