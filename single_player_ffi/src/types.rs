use perigee_single_player::level_0::perigee_core::rapier3d::na::{
    geometry::Translation, ArrayStorage, Const, Isometry, Matrix, Quaternion, Unit,
};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct CVector {
    x: f32,
    y: f32,
    z: f32,
}

// Vector<f32>
impl From<Matrix<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>>> for CVector {
    fn from(nalgebgra_vec: Matrix<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>>) -> Self {
        CVector {
            x: nalgebgra_vec.x,
            y: nalgebgra_vec.y,
            z: nalgebgra_vec.z,
        }
    }
}

impl From<Translation<f32, 3>> for CVector {
    fn from(nalgebgra_trans: Translation<f32, 3>) -> Self {
        CVector {
            x: nalgebgra_trans.x,
            y: nalgebgra_trans.y,
            z: nalgebgra_trans.z,
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct CQuaternion {
    w: f32,
    x: f32,
    y: f32,
    z: f32,
}

// Unit<Quaternion<f32>>
impl From<Unit<Quaternion<f32>>> for CQuaternion {
    fn from(nalgebgra_unit_quat: Unit<Quaternion<f32>>) -> Self {
        CQuaternion {
            w: nalgebgra_unit_quat.coords.w,
            x: nalgebgra_unit_quat.coords.x,
            y: nalgebgra_unit_quat.coords.y,
            z: nalgebgra_unit_quat.coords.z,
        }
    }
}

// Quaternion<f32>
impl From<Quaternion<f32>> for CQuaternion {
    fn from(nalgebgra_unit_quat: Quaternion<f32>) -> Self {
        CQuaternion {
            w: nalgebgra_unit_quat.coords.w,
            x: nalgebgra_unit_quat.coords.x,
            y: nalgebgra_unit_quat.coords.y,
            z: nalgebgra_unit_quat.coords.z,
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct CIsometry {
    translation: CVector,
    rotation: CQuaternion,
}

// Isometry<f32, Unit<Quaternion<f32>>, 3>
impl From<Isometry<f32, Unit<Quaternion<f32>>, 3>> for CIsometry {
    fn from(nalgebgra_iso: Isometry<f32, Unit<Quaternion<f32>>, 3>) -> Self {
        CIsometry {
            translation: CVector::from(nalgebgra_iso.translation),
            rotation: CQuaternion::from(nalgebgra_iso.rotation),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    isometry: CIsometry,
    scale: CVector,
}
