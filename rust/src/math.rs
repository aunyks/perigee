use rapier3d::na::{one, Isometry3, RealField, Scalar, SimdValue, Vector3};
use std::ops::{Add, Div, Mul, Sub};

#[inline]
pub fn lerp<T>(start: T, end: T, t: T) -> T
where
    T: Mul<Output = T> + Add<Output = T> + From<f32> + Sub<T, Output = T> + Copy,
{
    (start * (T::from(1.0) - t)) + (end * t)
}

// https://marcospereira.me/2022/08/24/lerp-how-to-frame-rate-independent/
/// A framerate-independent interpolation factor designed specifically for linear interpolation.
#[inline]
pub fn framerate_independent_interp_t<T>(factor_per_second: T, delta_seconds: f32) -> f32
where
    T: From<f32>,
    f32: From<T>,
{
    1.0 - (1.0 - f32::from(factor_per_second)).powf(delta_seconds)
}

// Adapted from the Unity codebase
// https://github.com/Unity-Technologies/UnityCsReference/blob/0a2eeb7a72710d89cccdb6aeee8431d27ee99cd1/Runtime/Export/Math/Vector3.cs#L61
#[inline]
pub fn move_towards(
    current: &Vector3<f32>,
    target: &Vector3<f32>,
    max_distance_delta: f32,
) -> Vector3<f32> {
    let difference_vector = target - current;
    let squared_distance = difference_vector.component_mul(&difference_vector).sum();
    if squared_distance == 0.0
        || (max_distance_delta >= 0.0 && squared_distance <= max_distance_delta.powi(2))
    {
        return *target;
    }
    // The normalized difference
    // between the current and target vectors.
    let normalized_vector_distance = difference_vector.normalize();

    current + normalized_vector_distance * max_distance_delta
}

// Adapted from: https://github.com/Unity-Technologies/UnityCsReference/blob/c84064be69f20dcf21ebe4a7bbc176d48e2f289c/Runtime/Export/Math/Vector3.cs#L308-L320
#[inline]
pub fn project_on_plane(vector: &Vector3<f32>, plane_normal: &Vector3<f32>) -> Vector3<f32> {
    let squared_magnitude = plane_normal.dot(plane_normal);
    if squared_magnitude < std::f32::EPSILON {
        return *vector;
    } else {
        let dot = vector.dot(plane_normal);
        return vector - plane_normal * dot / squared_magnitude;
    }
}

#[inline]
pub fn remap<T>(num: T, start_min: T, start_max: T, end_min: T, end_max: T) -> T
where
    T: Mul<Output = T>
        + Add<Output = T>
        + From<f32>
        + Sub<T, Output = T>
        + Div<T, Output = T>
        + Copy,
{
    (num - start_min) * (end_max - end_min) / (start_max - start_min) + end_min
}

#[derive(Clone, Copy)]
pub struct Transform3<T>
where
    T: Scalar + SimdValue + RealField + Copy,
{
    pub(crate) isometry: Isometry3<T>,
    pub(crate) scale: Vector3<T>,
}

impl<T> Transform3<T>
where
    T: Scalar + SimdValue + RealField + Copy,
{
    pub fn identity() -> Self {
        Self {
            isometry: Isometry3::identity(),
            scale: Vector3::new(one(), one(), one()),
        }
    }

    pub fn from_parts(isometry: Isometry3<T>, scale: Vector3<T>) -> Self {
        Self { isometry, scale }
    }

    pub fn scale(&self) -> &Vector3<T> {
        &self.scale
    }

    pub fn isometry(&self) -> &Isometry3<T> {
        &self.isometry
    }
}

impl<T> From<Isometry3<T>> for Transform3<T>
where
    T: Scalar + SimdValue + RealField + Copy,
{
    fn from(isometry: Isometry3<T>) -> Self {
        Self::from_parts(isometry, Vector3::new(one(), one(), one()))
    }
}

impl<T> Mul<Isometry3<T>> for Transform3<T>
where
    T: Scalar + SimdValue + RealField + Copy,
{
    type Output = Transform3<T>;

    fn mul(self, rhs: Isometry3<T>) -> Self::Output {
        Transform3 {
            isometry: self.isometry
                * Isometry3::from_parts(
                    self.scale.component_mul(&rhs.translation.vector).into(),
                    rhs.rotation,
                ),
            scale: self.scale,
        }
    }
}

impl<T> Mul<&Isometry3<T>> for Transform3<T>
where
    T: Scalar + SimdValue + RealField + Copy,
{
    type Output = Transform3<T>;

    fn mul(self, rhs: &Isometry3<T>) -> Self::Output {
        self * (*rhs)
    }
}

impl<T> Mul<Isometry3<T>> for &Transform3<T>
where
    T: Scalar + SimdValue + RealField + Copy,
{
    type Output = Transform3<T>;

    fn mul(self, rhs: Isometry3<T>) -> Self::Output {
        (*self) * rhs
    }
}

impl<T> Mul<&Isometry3<T>> for &Transform3<T>
where
    T: Scalar + SimdValue + RealField + Copy,
{
    type Output = Transform3<T>;

    fn mul(self, rhs: &Isometry3<T>) -> Self::Output {
        (*self) * (*rhs)
    }
}

impl<T> Mul<Transform3<T>> for Transform3<T>
where
    T: Scalar + SimdValue + RealField + Copy,
{
    type Output = Transform3<T>;
    fn mul(self, rhs: Transform3<T>) -> Self::Output {
        Transform3 {
            isometry: (self * rhs.isometry).isometry,
            scale: self.scale.component_mul(&rhs.scale),
        }
    }
}

impl<T> Mul<&Transform3<T>> for Transform3<T>
where
    T: Scalar + SimdValue + RealField + Copy,
{
    type Output = Transform3<T>;
    fn mul(self, rhs: &Transform3<T>) -> Self::Output {
        self * (*rhs)
    }
}

impl<T> Mul<Transform3<T>> for &Transform3<T>
where
    T: Scalar + SimdValue + RealField + Copy,
{
    type Output = Transform3<T>;
    fn mul(self, rhs: Transform3<T>) -> Self::Output {
        (*self) * rhs
    }
}

impl<T> Mul<&Transform3<T>> for &Transform3<T>
where
    T: Scalar + SimdValue + RealField + Copy,
{
    type Output = Transform3<T>;
    fn mul(self, rhs: &Transform3<T>) -> Self::Output {
        (*self) * (*rhs)
    }
}

impl<T> Mul<Vector3<T>> for Transform3<T>
where
    T: Scalar + SimdValue + RealField + Copy,
{
    type Output = Vector3<T>;

    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        // T * R * S: Scale first, rotate second, translate third
        self.isometry * (self.scale.component_mul(&rhs))
    }
}

impl<T> Mul<&Vector3<T>> for Transform3<T>
where
    T: Scalar + SimdValue + RealField + Copy,
{
    type Output = Vector3<T>;

    fn mul(self, rhs: &Vector3<T>) -> Self::Output {
        // T * R * S: Scale first, rotate second, translate third
        self.isometry * (*rhs)
    }
}

impl<T> Mul<Vector3<T>> for &Transform3<T>
where
    T: Scalar + SimdValue + RealField + Copy,
{
    type Output = Vector3<T>;

    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        // T * R * S: Scale first, rotate second, translate third
        (*self).isometry * rhs
    }
}

impl<T> Mul<&Vector3<T>> for &Transform3<T>
where
    T: Scalar + SimdValue + RealField + Copy,
{
    type Output = Vector3<T>;

    fn mul(self, rhs: &Vector3<T>) -> Self::Output {
        // T * R * S: Scale first, rotate second, translate third
        (*self).isometry * (*rhs)
    }
}
