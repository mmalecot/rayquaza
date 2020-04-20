//! Mathematics module.

use crate::ffi;
use std::{
    f32::consts::PI,
    mem,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// Convenient macro to create a vector.
macro_rules! vector {
    ($x:expr, $y: expr) => {
        Vector2::new($x, $y)
    };
    ($x:expr, $y: expr, $z: expr) => {
        Vector3::new($x, $y, $z)
    };
}

/// Vector2 type.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const ZERO: Vector2 = vector!(0.0, 0.0);
    pub const ONE: Vector2 = vector!(1.0, 1.0);

    /// Creates a `Vector2`.
    #[inline]
    pub const fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    /// Calculates the vector length.
    pub fn length(self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    /// Calculates the dot product with `vector`.
    pub fn dot(self, vector: Vector2) -> f32 {
        self.x * vector.x + self.y * vector.y
    }

    /// Calculates the distance towards `vector`.
    pub fn distance_to(self, vector: Vector2) -> f32 {
        ((self.x - vector.x) * (self.x - vector.x) + (self.y - vector.y) * (self.y - vector.y))
            .sqrt()
    }

    /// Scales the vector by multiplying both components by `scale`.
    pub fn scale(&mut self, scale: f32) {
        *self *= scale;
    }

    /// Returns a new `Vector2` with components scaled by `scale`.
    pub fn scale_by(self, scale: f32) -> Vector2 {
        self * scale
    }

    /// Normalizes the vector.
    pub fn normalize(&mut self) {
        *self /= self.length();
    }

    /// Returns a new `Vector2` with normalized components.
    pub fn normalized(self) -> Vector2 {
        self / self.length()
    }

    /// Returns a new `Vector2` with components linearly interpolated by `amount` towards `vector`.
    pub fn lerp(self, vector: Vector2, amount: f32) -> Vector2 {
        vector!(
            self.x + amount * (vector.x - self.x),
            self.y + amount * (vector.y - self.y)
        )
    }

    /// Returns a new `Vector2` with rotation by `degrees`.
    pub fn rotate(self, degrees: f32) -> Vector2 {
        let radians = degrees.to_radians();
        vector!(
            self.x * radians.cos() - self.y * radians.sin(),
            self.x * radians.sin() + self.y * radians.cos()
        )
    }

    /// Calculates the angle towards `vector` in radians.
    pub fn angle_to(self, vector: Vector2) -> f32 {
        let result = (vector.y - self.y).atan2(vector.x - self.x);
        if result < 0.0 {
            result + 2.0 * PI
        } else {
            result
        }
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, vector: Vector2) -> Vector2 {
        vector!(self.x + vector.x, self.y + vector.y)
    }
}

impl Add<f32> for Vector2 {
    type Output = Vector2;
    fn add(self, value: f32) -> Self {
        vector!(self.x + value, self.y + value)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, vector: Vector2) {
        *self = *self + vector;
    }
}

impl AddAssign<f32> for Vector2 {
    fn add_assign(&mut self, value: f32) {
        *self = *self + value;
    }
}

impl Div for Vector2 {
    type Output = Vector2;
    fn div(self, vector: Vector2) -> Self {
        vector!(self.x / vector.x, self.y / vector.y)
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, value: f32) -> Self {
        vector!(self.x / value, self.y / value)
    }
}

impl DivAssign for Vector2 {
    fn div_assign(&mut self, vector: Vector2) {
        *self = *self / vector;
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, value: f32) {
        *self = *self / value;
    }
}

impl From<&Vector2> for Vector2 {
    #[inline]
    fn from(vector: &Vector2) -> Vector2 {
        *vector
    }
}

impl From<[f32; 2]> for Vector2 {
    #[inline]
    fn from(value: [f32; 2]) -> Vector2 {
        vector!(value[0], value[1])
    }
}

impl From<(f32, f32)> for Vector2 {
    #[inline]
    fn from((x, y): (f32, f32)) -> Vector2 {
        vector!(x, y)
    }
}

impl From<ffi::Vector2> for Vector2 {
    fn from(vector: ffi::Vector2) -> Vector2 {
        unsafe { mem::transmute(vector) }
    }
}

impl Into<[f32; 2]> for Vector2 {
    #[inline]
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl Into<(f32, f32)> for Vector2 {
    #[inline]
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl Into<ffi::Vector2> for Vector2 {
    fn into(self) -> ffi::Vector2 {
        unsafe { mem::transmute(self) }
    }
}

impl Into<ffi::Vector2> for &Vector2 {
    fn into(self) -> ffi::Vector2 {
        self.clone().into()
    }
}

impl Mul for Vector2 {
    type Output = Vector2;
    fn mul(self, vector: Vector2) -> Self {
        vector!(self.x * vector.x, self.y * vector.y)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, value: f32) -> Self {
        vector!(self.x * value, self.y * value)
    }
}

impl MulAssign for Vector2 {
    fn mul_assign(&mut self, vector: Vector2) {
        *self = *self * vector;
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, value: f32) {
        *self = *self * value;
    }
}

impl Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Self {
        vector!(-self.x, -self.y)
    }
}

impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, vector: Vector2) -> Self {
        vector!(self.x - vector.x, self.y - vector.y)
    }
}

impl Sub<f32> for Vector2 {
    type Output = Vector2;
    fn sub(self, value: f32) -> Self {
        vector!(self.x - value, self.y - value)
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, vector: Vector2) {
        *self = *self - vector;
    }
}

impl SubAssign<f32> for Vector2 {
    fn sub_assign(&mut self, value: f32) {
        *self = *self - value;
    }
}

/// Vector3 type.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const ZERO: Vector3 = vector!(0.0, 0.0, 0.0);
    pub const ONE: Vector3 = vector!(1.0, 1.0, 1.0);
    pub const UP: Vector3 = vector!(0.0, 1.0, 0.0);
    pub const FORWARD: Vector3 = vector!(0.0, 0.0, 1.0);
    pub const RIGHT: Vector3 = vector!(1.0, 0.0, 0.0);
    pub const LEFT: Vector3 = vector!(-1.0, 0.0, 0.0);

    /// Creates a `Vector3`.
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Calculates the vector length.
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Calculates the dot product with `vector`.
    pub fn dot(self, vector: Vector3) -> f32 {
        self.x * vector.x + self.y * vector.y + self.z * vector.z
    }

    /// Calculates the distance towards `vector`.
    pub fn distance_to(self, vector: Vector3) -> f32 {
        let dx = vector.x - self.x;
        let dy = vector.y - self.y;
        let dz = vector.z - self.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Scales the vector by multiplying both components by `scale`.
    pub fn scale(&mut self, scale: f32) {
        *self *= scale;
    }

    /// Returns a new `Vector3` with components scaled by `scale`.
    pub fn scale_by(self, scale: f32) -> Vector3 {
        self * scale
    }

    /// Normalizes the vector.
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    /// Returns a new `Vector3` with normalized components.
    pub fn normalized(self) -> Vector3 {
        let mut length = self.length();
        if length == 0.0 {
            length = 1.0;
        }
        let length = 1.0 / length;
        vector!(self.x * length, self.x * length, self.x * length)
    }

    /// Returns a new `Vector3` with components linearly interpolated by `amount` towards `vector`.
    pub fn lerp(&self, vector: Vector3, amount: f32) -> Vector3 {
        vector!(
            self.x + amount * (vector.x - self.x),
            self.y + amount * (vector.y - self.y),
            self.z + amount * (vector.z - self.z)
        )
    }

    /// Returns a new `Vector3` containing the cross product between `self` and `vector`.
    pub fn cross(&self, vector: Vector3) -> Vector3 {
        vector!(
            self.y * vector.z - self.z * vector.y,
            self.z * vector.x - self.x * vector.z,
            self.x * vector.y - self.y * vector.x
        )
    }

    /// Returns a new `Vector3` perpendicular to `self`.
    pub fn perpendicular(&self) -> Vector3 {
        let x = self.x.abs();
        let y = self.y.abs();
        let z = self.z.abs();
        self.cross(if x < y && x < z {
            vector!(1.0, 0.0, 0.0)
        } else if y < z {
            vector!(0.0, 1.0, 0.0)
        } else {
            vector!(0.0, 0.0, 1.0)
        })
    }

    /// Normalizes and changes both `self` and `vector` to be orthogonal to each other.
    pub fn ortho_normalize(&mut self, vector: &mut Vector3) {
        *self = self.normalized();
        *vector = self.cross(*vector).normalized().cross(*self);
    }

    /// Reflects from `normal`.
    pub fn reflect(&mut self, normal: Vector3) {
        *self = self.reflect_from(normal);
    }

    /// Returns a new `Vector3` reflected from `normal`.
    pub fn reflect_from(&self, normal: Vector3) -> Vector3 {
        let dot_product = self.dot(normal);
        vector!(
            self.x - (2.0 * normal.x) * dot_product,
            self.y - (2.0 * normal.y) * dot_product,
            self.z - (2.0 * normal.z) * dot_product
        )
    }

    /// Returns a new `Vector3` containing the minimum of each corresponding component.
    pub fn min(&self, vector: Vector3) -> Vector3 {
        vector!(
            self.x.min(vector.x),
            self.y.min(vector.y),
            self.z.min(vector.z)
        )
    }

    /// Returns a new `Vector3` containing the maximum of each corresponding component.
    pub fn max(&self, vector: Vector3) -> Vector3 {
        vector!(
            self.x.max(vector.x),
            self.y.max(vector.y),
            self.z.max(vector.z)
        )
    }

    /// Returns barycenter coordinates from point with respect to triangle.
    pub fn barycenter(&self, vector1: Vector3, vector2: Vector3, vector3: Vector3) -> Vector3 {
        let v0 = vector2 - vector1;
        let v1 = vector3 - vector1;
        let v2 = *self - vector1;
        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);
        let d20 = v2.dot(v0);
        let d21 = v2.dot(v1);
        let denominator = d00 * d11 - d01 * d01;
        let y = (d11 * d20 - d01 * d21) / denominator;
        let z = (d00 * d21 - d01 * d20) / denominator;
        vector!(1.0 - (z + y), y, z)
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, vector: Vector3) -> Self {
        vector!(self.x + vector.x, self.y + vector.y, self.z + vector.z)
    }
}

impl Add<f32> for Vector3 {
    type Output = Vector3;
    fn add(self, value: f32) -> Self {
        vector!(self.x + value, self.y + value, self.z + value)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, vector: Vector3) {
        *self = *self + vector;
    }
}

impl AddAssign<f32> for Vector3 {
    fn add_assign(&mut self, value: f32) {
        *self = *self + value;
    }
}

impl Div for Vector3 {
    type Output = Vector3;
    fn div(self, vector: Vector3) -> Self {
        vector!(self.x / vector.x, self.y / vector.y, self.z / vector.z)
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, value: f32) -> Self {
        vector!(self.x / value, self.y / value, self.z / value)
    }
}

impl DivAssign for Vector3 {
    fn div_assign(&mut self, v: Vector3) {
        *self = *self / v;
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, value: f32) {
        *self = *self / value;
    }
}

impl From<&Vector3> for Vector3 {
    #[inline]
    fn from(vector: &Vector3) -> Vector3 {
        *vector
    }
}

impl From<[f32; 3]> for Vector3 {
    #[inline]
    fn from(value: [f32; 3]) -> Vector3 {
        vector!(value[0], value[1], value[2])
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    #[inline]
    fn from((x, y, z): (f32, f32, f32)) -> Vector3 {
        vector!(x, y, z)
    }
}

impl From<ffi::Vector3> for Vector3 {
    fn from(vector: ffi::Vector3) -> Vector3 {
        unsafe { mem::transmute(vector) }
    }
}

impl Into<[f32; 3]> for Vector3 {
    #[inline]
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl Into<(f32, f32, f32)> for Vector3 {
    #[inline]
    fn into(self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}

impl Into<ffi::Vector3> for Vector3 {
    fn into(self) -> ffi::Vector3 {
        unsafe { mem::transmute(self) }
    }
}

impl Into<ffi::Vector3> for &Vector3 {
    fn into(self) -> ffi::Vector3 {
        self.clone().into()
    }
}

impl Mul for Vector3 {
    type Output = Vector3;
    fn mul(self, vector: Vector3) -> Self {
        vector!(self.x * vector.x, self.y * vector.y, self.z * vector.z)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, value: f32) -> Self {
        vector!(self.x * value, self.y * value, self.z * value)
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, v: Vector3) {
        *self = *self * v;
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, value: f32) {
        *self = *self * value;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Self {
        vector!(-self.x, -self.y, -self.z)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, vector: Vector3) -> Self {
        vector!(self.x - vector.x, self.y - vector.y, self.z - vector.z)
    }
}

impl Sub<f32> for Vector3 {
    type Output = Vector3;
    fn sub(self, value: f32) -> Self {
        vector!(self.x - value, self.y - value, self.z - value)
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, v: Vector3) {
        *self = *self - v;
    }
}

impl SubAssign<f32> for Vector3 {
    fn sub_assign(&mut self, value: f32) {
        *self = *self - value;
    }
}

#[inline]
/// Clamps a value between the given minimum and maximum values.
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::core::math::{self, Vector2, Vector3};

    #[test]
    fn test_vector2_new() {
        assert_eq!(Vector2::new(0.0, 0.0), Vector2::ZERO);
        assert_eq!(Vector2::new(1.0, 1.0), Vector2::ONE);
    }

    #[test]
    fn test_vector2_from_ref() {
        assert_eq!(Vector2::from(&Vector2::ZERO), Vector2::ZERO);
        assert_eq!(Vector2::from(&Vector2::ONE), Vector2::ONE);
    }

    #[test]
    fn test_vector2_from_f32_array() {
        assert_eq!(Vector2::from([0.0, 0.0]), Vector2::ZERO);
        assert_eq!(Vector2::from([1.0, 1.0]), Vector2::ONE);
    }

    #[test]
    fn test_vector2_from_f32_slice() {
        assert_eq!(Vector2::from((0.0, 0.0)), Vector2::ZERO);
        assert_eq!(Vector2::from((1.0, 1.0)), Vector2::ONE);
    }

    #[test]
    fn test_vector2_into_f32_array() {
        let value: [f32; 2] = Vector2::ZERO.into();
        assert_eq!(value, [0.0, 0.0]);
        let value: [f32; 2] = Vector2::ONE.into();
        assert_eq!(value, [1.0, 1.0]);
    }

    #[test]
    fn test_vector2_into_f32_slice() {
        let value: (f32, f32) = Vector2::ZERO.into();
        assert_eq!(value, (0.0, 0.0));
        let value: (f32, f32) = Vector2::ONE.into();
        assert_eq!(value, (1.0, 1.0));
    }

    #[test]
    fn test_vector3_new() {
        assert_eq!(Vector3::new(0.0, 0.0, 0.0), Vector3::ZERO);
        assert_eq!(Vector3::new(1.0, 1.0, 1.0), Vector3::ONE);
    }

    #[test]
    fn test_vector3_from_ref() {
        assert_eq!(Vector3::from(&Vector3::ZERO), Vector3::ZERO);
        assert_eq!(Vector3::from(&Vector3::ONE), Vector3::ONE);
    }

    #[test]
    fn test_vector3_from_f32_array() {
        assert_eq!(Vector3::from([0.0, 0.0, 0.0]), Vector3::ZERO);
        assert_eq!(Vector3::from([1.0, 1.0, 1.0]), Vector3::ONE);
    }

    #[test]
    fn test_vector3_from_f32_slice() {
        assert_eq!(Vector3::from((0.0, 0.0, 0.0)), Vector3::ZERO);
        assert_eq!(Vector3::from((1.0, 1.0, 1.0)), Vector3::ONE);
    }

    #[test]
    fn test_vector3_into_f32_array() {
        let value: [f32; 3] = Vector3::ZERO.into();
        assert_eq!(value, [0.0, 0.0, 0.0]);
        let value: [f32; 3] = Vector3::ONE.into();
        assert_eq!(value, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_vector3_into_f32_slice() {
        let value: (f32, f32, f32) = Vector3::ZERO.into();
        assert_eq!(value, (0.0, 0.0, 0.0));
        let value: (f32, f32, f32) = Vector3::ONE.into();
        assert_eq!(value, (1.0, 1.0, 1.0));
    }

    #[test]
    fn test_clamp() {
        assert_eq!(math::clamp(200, 100, 300), 200);
        assert_eq!(math::clamp(400, 100, 300), 300);
        assert_eq!(math::clamp(0, 100, 300), 100);
    }
}
