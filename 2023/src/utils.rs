use std::ops::{Add, Sub};

pub type UPoint = Point<usize>;
pub type IPoint = Point<isize>;

pub trait Num {
    fn one() -> Self;
    fn zero() -> Self;
}

impl Num for usize {
    fn one() -> Self {
        1usize
    }
    fn zero() -> Self {
        0usize
    }
}

impl Num for isize {
    fn one() -> Self {
        1isize
    }
    fn zero() -> Self {
        0isize
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Add<Output = T> + Sub<Output = T> + Num + Copy + PartialEq,
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
    T: Add<Output = T> + Sub<Output = T> + Num + Copy + PartialEq,
{
    type Output = Point<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Sub for Point<T>
where
    T: Add<Output = T> + Sub<Output = T> + Num + Copy + PartialEq,
{
    type Output = Point<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

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
