use std::ops::{Rem, Mul, Div, Add};

#[derive(Clone, Copy)]
pub struct Vec2<T> {
    x: T,
    y: T
}

impl<T: Copy> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        return Self{x, y};
    }

    #[inline]
    pub fn x(&self) -> T {
        return self.x;
    }

    #[inline]
    pub fn y(&self) -> T {
        return self.y;
    }
}

impl<T: Copy + Rem<Output = T>> Rem for Vec2<T> {
    type Output = Vec2<T>;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        return Self::Output::new(self.x % rhs.x, self.y % rhs.y);
    }
}

impl<T: Copy + Mul<Output = T>> Mul for Vec2<T> {
    type Output = Vec2<T>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        return Self::Output::new(self.x * rhs.x, self.y * rhs.y);
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        return Self::Output::new(self.x * rhs, self.y * rhs);
    }
}

impl<T: Copy + Div<Output = T>> Div for Vec2<T> {
    type Output = Vec2<T>;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        return Self::Output::new(self.x / rhs.x, self.y / rhs.y);
    }
}

impl<T: Copy + Add<Output = T>> Add for Vec2<T> {
    type Output = Vec2<T>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        return Self::Output::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl<T: Copy> From<(T, T)> for Vec2<T> {
    #[inline]
    fn from(value: (T, T)) -> Self {
        return Self::new(value.0, value.1);
    }
}

impl<T> From<Vec2<T>> for (T, T) {
    #[inline]
    fn from(val: Vec2<T>) -> Self {
        return (val.x, val.y);
    }
}
