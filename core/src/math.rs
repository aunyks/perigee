use rapier3d::na::Vector3;

// Adapted from the Unit codebase
// https://github.com/Unity-Technologies/UnityCsReference/blob/0a2eeb7a72710d89cccdb6aeee8431d27ee99cd1/Runtime/Export/Math/Vector3.cs#L61
#[inline]
pub fn move_towards(
    current: &Vector3<f32>,
    target: &Vector3<f32>,
    max_distance_delta: f32,
) -> Vector3<f32> {
    let intermediate_vector = target - current;
    let squared_distance = intermediate_vector
        .component_mul(&intermediate_vector)
        .sum();
    if squared_distance == 0.0
        || (max_distance_delta >= 0.0 && squared_distance <= max_distance_delta.powf(2.0))
    {
        return *target;
    }
    let distance = squared_distance.sqrt();

    current + intermediate_vector / distance * max_distance_delta
}

// Adapted from: https://github.com/Unity-Technologies/UnityCsReference/blob/c84064be69f20dcf21ebe4a7bbc176d48e2f289c/Runtime/Export/Math/Vector3.cs#L308-L320
#[inline]
pub fn project_on_plane(vector: &Vector3<f32>, plane_normal: &Vector3<f32>) -> Vector3<f32> {
    let squared_magnitude = plane_normal.dot(plane_normal);
    let dot = vector.dot(plane_normal);
    vector - plane_normal * dot / squared_magnitude
}
