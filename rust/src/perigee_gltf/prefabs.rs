use crate::math::Transform3;

#[derive(Clone)]
enum PrefabItem {}

#[derive(Clone)]
struct SceneTreeNode {
    transform: Transform3<f32>,
    item: PrefabItem,
    children: Vec<SceneTreeNode>,
}

#[derive(Clone)]
pub struct Prefab {
    root: SceneTreeNode,
}
