use rapier3d::na::{Isometry, UnitQuaternion, Vector3};
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
