use std::fmt::Debug;
use std::ops::*;

pub trait Sqrt {
    fn sqrti(&self) -> Self;
}

impl Sqrt for f32 {
    fn sqrti(&self) -> Self {
        self.sqrt()
    }
}

impl Sqrt for i32 {
    fn sqrti(&self) -> Self {
        (*self as f32).sqrt() as i32
    }
}

pub trait Invert {
    fn invert(&self) -> Self;
}

impl Invert for f32 {
    fn invert(&self) -> Self {
        1. / *self
    }
}

impl Invert for i32 {
    fn invert(&self) -> Self {
        1 / *self
    }
}

pub trait Scalar:
    Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Neg<Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + Sqrt
    + Invert
    + Clone
    + Copy
    + Debug
    + Default
{
}

impl Scalar for f32 {}
impl Scalar for i32 {}

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec2<S: Scalar>(pub [S; 2]);

#[rustfmt::skip]
impl<S: Scalar> Vec2<S> {
    pub fn new(x: S, y: S) -> Self {
        Vec2([x, y])
    }

    pub fn x(&self) -> S { self.0[0] }
    pub fn y(&self) -> S { self.0[1] }
    pub fn u(&self) -> S { self.0[0] }
    pub fn v(&self) -> S { self.0[1] }

    pub fn length(&self) -> S {
        (self.0[0] * self.0[0] + self.0[1] * self.0[1]).sqrti()
    }

    pub fn squared_length(&self) -> S {
        self.0[0] * self.0[0] + self.0[1] * self.0[1]
    }

    pub fn dot(&self, other: Self) -> S {
        self.0[0] * other.0[0] + self.0[1] * other.0[1]
    }
}

impl<S: Scalar> Add for Vec2<S> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
        )
    }
}

impl<S: Scalar> AddAssign for Vec2<S> {
    fn add_assign(&mut self, other: Self) {
        self.0[0] += other.0[0];
        self.0[1] += other.0[1];
    }
}

impl<S: Scalar> Sub for Vec2<S> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
        )
    }
}

impl<S: Scalar> SubAssign for Vec2<S> {
    fn sub_assign(&mut self, other: Self) {
        self.0[0] -= other.0[0];
        self.0[1] -= other.0[1];
    }
}

impl<S: Scalar> Mul for Vec2<S> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.0[0] * other.0[0],
            self.0[1] * other.0[1],
        )
    }
}

impl<S: Scalar> MulAssign for Vec2<S> {
    fn mul_assign(&mut self, other: Self) {
        self.0[0] *= other.0[0];
        self.0[1] *= other.0[1];
    }
}

impl<S: Scalar> Mul<S> for Vec2<S> {
    type Output = Self;

    fn mul(self, other: S) -> Self {
        Self::new(self.0[0] * other, self.0[1] * other)
    }
}

impl<S: Scalar> MulAssign<S> for Vec2<S> {
    fn mul_assign(&mut self, other: S) {
        self.0[0] *= other;
        self.0[1] *= other;
    }
}

impl<S: Scalar> Div for Vec2<S> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(
            self.0[0] / other.0[0],
            self.0[1] / other.0[1],
        )
    }
}

impl<S: Scalar> DivAssign for Vec2<S> {
    fn div_assign(&mut self, other: Self) {
        self.0[0] /= other.0[0];
        self.0[1] /= other.0[1];
    }
}

impl<S: Scalar> Div<S> for Vec2<S> {
    type Output = Self;

    fn div(self, other: S) -> Self {
        Self::new(self.0[0] / other, self.0[1] / other)
    }
}

impl<S: Scalar> DivAssign<S> for Vec2<S> {
    fn div_assign(&mut self, other: S) {
        self.0[0] /= other;
        self.0[1] /= other;
    }
}

impl<S: Scalar> Neg for Vec2<S> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.0[0], -self.0[1])
    }
}

impl<S: Scalar> Index<usize> for Vec2<S> {
    type Output = S;

    #[inline(always)]
    fn index(&self, idx: usize) -> &S {
        &self.0[idx]
    }
}

impl<S: Scalar> IndexMut<usize> for Vec2<S> {
    fn index_mut(&mut self, idx: usize) -> &mut S {
        &mut self.0[idx]
    }
}

impl<S: Scalar> From<(S, S)> for Vec2<S> {
    fn from((x, y): (S, S)) -> Self {
        Self::new(x, y)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3<S: Scalar>(pub [S; 3]);

#[rustfmt::skip]
impl<S: Scalar> Vec3<S> {
    pub fn new(x: S, y: S, z: S) -> Vec3<S> {
        Vec3([x, y, z])
    }

    pub fn x(&self) -> S { self.0[0] }
    pub fn y(&self) -> S { self.0[1] }
    pub fn z(&self) -> S { self.0[2] }
    pub fn ivert(&self) -> S { self.0[0] }
    pub fn iuv(&self) -> S { self.0[1] }
    pub fn inorm(&self) -> S { self.0[2] }

    pub fn length(&self) -> S {
        (self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2]).sqrti()
    }

    pub fn squared_length(&self) -> S {
        self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2]
    }

    pub fn normalize(&mut self) {
        let k = self.length().invert();
        self.0[0] *= k;
        self.0[1] *= k;
        self.0[2] *= k;
    }

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }

    pub fn dot(&self, other: Self) -> S {
        self.0[0] * other.0[0] + self.0[1] * other.0[1] + self.0[2] * other.0[2]
    }

    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self.0[1] * other.0[2] - self.0[2] * other.0[1],
            self.0[2] * other.0[0] - self.0[0] * other.0[2],
            self.0[0] * other.0[1] - self.0[1] * other.0[0],
        )
    }
}

impl<S: Scalar> Add for Vec3<S> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
        )
    }
}

impl<S: Scalar> AddAssign for Vec3<S> {
    fn add_assign(&mut self, other: Self) {
        self.0[0] += other.0[0];
        self.0[1] += other.0[1];
        self.0[2] += other.0[2];
    }
}

impl<S: Scalar> Sub for Vec3<S> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
        )
    }
}

impl<S: Scalar> SubAssign for Vec3<S> {
    fn sub_assign(&mut self, other: Self) {
        self.0[0] -= other.0[0];
        self.0[1] -= other.0[1];
        self.0[2] -= other.0[2];
    }
}

impl<S: Scalar> Mul for Vec3<S> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.0[0] * other.0[0],
            self.0[1] * other.0[1],
            self.0[2] * other.0[2],
        )
    }
}

impl<S: Scalar> MulAssign for Vec3<S> {
    fn mul_assign(&mut self, other: Self) {
        self.0[0] *= other.0[0];
        self.0[1] *= other.0[1];
        self.0[2] *= other.0[2];
    }
}

impl<S: Scalar> Mul<S> for Vec3<S> {
    type Output = Self;

    fn mul(self, other: S) -> Self {
        Self::new(self.0[0] * other, self.0[1] * other, self.0[2] * other)
    }
}

impl<S: Scalar> MulAssign<S> for Vec3<S> {
    fn mul_assign(&mut self, other: S) {
        self.0[0] *= other;
        self.0[1] *= other;
        self.0[2] *= other;
    }
}

impl<S: Scalar> Div for Vec3<S> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(
            self.0[0] / other.0[0],
            self.0[1] / other.0[1],
            self.0[2] / other.0[2],
        )
    }
}

impl<S: Scalar> DivAssign for Vec3<S> {
    fn div_assign(&mut self, other: Self) {
        self.0[0] /= other.0[0];
        self.0[1] /= other.0[1];
        self.0[2] /= other.0[2];
    }
}

impl<S: Scalar> Div<S> for Vec3<S> {
    type Output = Self;

    fn div(self, other: S) -> Self {
        Self::new(self.0[0] / other, self.0[1] / other, self.0[2] / other)
    }
}

impl<S: Scalar> DivAssign<S> for Vec3<S> {
    fn div_assign(&mut self, other: S) {
        self.0[0] /= other;
        self.0[1] /= other;
        self.0[2] /= other;
    }
}

impl<S: Scalar> Neg for Vec3<S> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.0[0], -self.0[1], -self.0[2])
    }
}

impl<S: Scalar> Index<usize> for Vec3<S> {
    type Output = S;

    #[inline(always)]
    fn index(&self, idx: usize) -> &S {
        &self.0[idx]
    }
}

impl<S: Scalar> IndexMut<usize> for Vec3<S> {
    fn index_mut(&mut self, idx: usize) -> &mut S {
        &mut self.0[idx]
    }
}

impl<S: Scalar> From<(S, S, S)> for Vec3<S> {
    fn from((x, y, z): (S, S, S)) -> Self {
        Self::new(x, y, z)
    }
}
