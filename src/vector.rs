extern crate float;

use std::ops::{Add, Sub, Mul, Div, Rem, Neg};
use float::Sqrt;

#[derive(Copy,Clone,Debug)]
pub struct Vec2<T> {
    x: T,
    y: T,
}

impl<T: Add<S>, S: Copy> Add<Vec2<S>> for Vec2<T> {
    type Output = Vec2<<T as Add<S>>::Output>;

    fn add(self, other: Vec2<S>) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<S>, S: Copy> Sub<Vec2<S>> for Vec2<T> {
    type Output = Vec2<<T as Sub<S>>::Output>;

    fn sub(self, other: Vec2<S>) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Mul<S>, S: Copy> Mul<S> for Vec2<T> {
    type Output = Vec2<<T as Mul<S>>::Output>;

    fn mul(self, scalar: S) -> Self::Output {
        Self::Output {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T: Div<S>, S: Copy> Div<S> for Vec2<T> {
    type Output = Vec2<<T as Div<S>>::Output>;

    fn div(self, scalar: S) -> Self::Output {
        Self::Output {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl<T: Rem<S>, S: Copy> Rem<S> for Vec2<T> {
    type Output = Vec2<<T as Rem<S>>::Output>;

    fn rem(self, scalar: S) -> Self::Output {
        Self::Output {
            x: self.x % scalar,
            y: self.y % scalar,
        }
    }
}

impl<T: Neg> Neg for Vec2<T> {
    type Output = Vec2<<T as Neg>::Output>;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: - self.x,
            y: - self.y
        }
    }
}

impl<T: Copy + Mul<Output = A>, A: Add<Output = S>, S: Sqrt> Vec2<T> {
    fn length(&self) -> S {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl<T: Copy + Mul<Output = A> + Div<S>, A: Add<Output = S>, S: Sqrt + Copy> Vec2<T> {
    fn normalize(self) -> Vec2<<T as Div<S>>::Output> {
        self / self.length()
    }
}

pub trait Dot<RHS = Self> {
    type Output;

    fn dot(self, rhs: RHS) -> Self::Output;
}

impl<T: Mul<U, Output = S>, S: Add, U: Copy> Dot<Vec2<U>> for Vec2<T> {
    type Output = <S as Add>::Output;

    fn dot (self, other: Vec2<U>) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

pub fn dot<T: Dot<U>, U> (a: T, b: U) -> <T as Dot<U>>::Output {
    a.dot(b)
}

// clashes with core implementation
// impl<T, S: From<T> + T> From<Vec2<T>> for Vec2<S> {
//     fn from(prev: Vec2<T>) -> Self {
//         Vec2 {
//             x: S::from(prev.x),
//             y: S::from(prev.y),
//         }
//     }
// }