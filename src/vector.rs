use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, Neg};
use float::Sqrt;

//2D vector
#[derive(Copy,Clone,Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
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

impl<T: Add<S, Output = T> + Copy, S: Copy> AddAssign<Vec2<S>> for Vec2<T> {
    fn add_assign(&mut self, other: Vec2<S>) {
        *self = *self + other;
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

impl<T: Sub<S, Output = T> + Copy, S: Copy> SubAssign<Vec2<S>> for Vec2<T> {
    fn sub_assign(&mut self, other: Vec2<S>) {
        *self = *self - other;
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

impl<T: Mul<S, Output = T> + Copy, S: Copy> MulAssign<S> for Vec2<T> {
    fn mul_assign(&mut self, scalar: S) {
        *self = *self * scalar;
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

impl<T: Div<S, Output = T> + Copy, S: Copy> DivAssign<S> for Vec2<T> {
    fn div_assign(&mut self, scalar: S) {
        *self = *self / scalar;
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

impl<T: Rem<S, Output = T> + Copy, S: Copy> RemAssign<S> for Vec2<T> {
    fn rem_assign(&mut self, scalar: S) {
        *self = *self % scalar;
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
    pub fn length(&self) -> S {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl<T: Copy + Mul<Output = A> + Div<S>, A: Add<Output = S>, S: Sqrt + Copy> Vec2<T> {
    pub fn normalise(self) -> Vec2<<T as Div<S>>::Output> {
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

//3D vector
#[derive(Copy,Clone,Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Add<S>, S: Copy> Add<Vec3<S>> for Vec3<T> {
    type Output = Vec3<<T as Add<S>>::Output>;

    fn add(self, other: Vec3<S>) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Add<S, Output = T> + Copy, S: Copy> AddAssign<Vec3<S>> for Vec3<T> {
    fn add_assign(&mut self, other: Vec3<S>) {
        *self = *self + other;
    }
}

impl<T: Sub<S>, S: Copy> Sub<Vec3<S>> for Vec3<T> {
    type Output = Vec3<<T as Sub<S>>::Output>;

    fn sub(self, other: Vec3<S>) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Sub<S, Output = T> + Copy, S: Copy> SubAssign<Vec3<S>> for Vec3<T> {
    fn sub_assign(&mut self, other: Vec3<S>) {
        *self = *self - other;
    }
}

impl<T: Mul<S>, S: Copy> Mul<S> for Vec3<T> {
    type Output = Vec3<<T as Mul<S>>::Output>;

    fn mul(self, scalar: S) -> Self::Output {
        Self::Output {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<T: Mul<S, Output = T> + Copy, S: Copy> MulAssign<S> for Vec3<T> {
    fn mul_assign(&mut self, scalar: S) {
        *self = *self * scalar;
    }
}

impl<T: Div<S>, S: Copy> Div<S> for Vec3<T> {
    type Output = Vec3<<T as Div<S>>::Output>;

    fn div(self, scalar: S) -> Self::Output {
        Self::Output {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl<T: Div<S, Output = T> + Copy, S: Copy> DivAssign<S> for Vec3<T> {
    fn div_assign(&mut self, scalar: S) {
        *self = *self / scalar;
    }
}

impl<T: Rem<S>, S: Copy> Rem<S> for Vec3<T> {
    type Output = Vec3<<T as Rem<S>>::Output>;

    fn rem(self, scalar: S) -> Self::Output {
        Self::Output {
            x: self.x % scalar,
            y: self.y % scalar,
            z: self.z % scalar,
        }
    }
}

impl<T: Rem<S, Output = T> + Copy, S: Copy> RemAssign<S> for Vec3<T> {
    fn rem_assign(&mut self, scalar: S) {
        *self = *self % scalar;
    }
}

impl<T: Neg> Neg for Vec3<T> {
    type Output = Vec3<<T as Neg>::Output>;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: - self.x,
            y: - self.y,
            z: - self.z,
        }
    }
}

impl<T: Copy + Mul<Output = A>, A: Add<Output = A> + Sqrt> Vec3<T> {
    pub fn length(&self) -> A {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl<T: Copy + Mul<Output = A> + Div<A>, A: Add<Output = A> + Sqrt + Copy> Vec3<T> {
    pub fn normalise(self) -> Vec3<<T as Div<A>>::Output> {
        self / self.length()
    }
}

impl<T: Mul<U, Output = S>, S: Add<Output = S>, U: Copy> Dot<Vec3<U>> for Vec3<T> {
    type Output = <S as Add>::Output;

    fn dot (self, other: Vec3<U>) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

pub trait Cross<RHS = Self> {
    type Output;

    fn cross(self, rhs: RHS) -> Self::Output;
}

impl<T: Mul<U, Output = S> + Copy, S: Sub, U: Copy> Cross<Vec3<U>> for Vec3<T> {
    type Output = Vec3<<S as Sub>::Output>;

    fn cross (self, other: Vec3<U>) -> Self::Output {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

pub trait Clamp<RHS> {
    fn clamp (&self, max:RHS) -> Self;
}

impl<T: PartialOrd + Copy> Clamp<T> for Vec3<T> {
    fn clamp (&self, max: T) -> Self {
        Vec3 {
            x: {if self.x > max {max} else {self.x}},
            y: {if self.y > max {max} else {self.y}},
            z: {if self.z > max {max} else {self.z}},
        }
    }
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

//functions
pub fn dot<T: Dot<U>, U> (a: T, b: U) -> <T as Dot<U>>::Output {
    a.dot(b)
}