use num;
use num::traits::ConstZero;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vector2d<T> {
    x: T,
    y: T,
}

impl<T: Copy> Vector2d<T> {
    pub fn new(x: T, y: T) -> Vector2d<T> {
        Vector2d { x, y }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }
}

impl<T: Copy + Add> Add for Vector2d<T>
where
    <T as Add>::Output: Copy,
{
    type Output = Vector2d<<T as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2d::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Copy + Sub> Sub for Vector2d<T>
where
    <T as Sub>::Output: Copy,
{
    type Output = Vector2d<<T as Sub>::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2d::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Copy + Mul<S>, S: Copy> Mul<S> for Vector2d<T>
where
    <T as Mul<S>>::Output: Copy,
{
    type Output = Vector2d<<T as Mul<S>>::Output>;

    fn mul(self, rhs: S) -> Self::Output {
        Vector2d::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Copy + Div<S>, S: Copy> Div<S> for Vector2d<T>
where
    <T as Div<S>>::Output: Copy,
{
    type Output = Vector2d<<T as Div<S>>::Output>;

    fn div(self, rhs: S) -> Self::Output {
        Vector2d::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: ConstZero + Sub + Copy> Vector2d<T>
where
    <T as Sub>::Output: Copy,
{
    pub fn inverse(self) -> Vector2d<<T as Sub>::Output> {
        Vector2d::new(T::ZERO, T::ZERO) - self
    }
}

impl<T: Display> Display for Vector2d<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
