use nalgebra::{
    Matrix3 as _Matrix3, Matrix3x6 as _Matrix3x6, Matrix6 as _Matrix6, Matrix6x3 as _Matrix6x3,
    Vector3 as _Vector3, Vector6 as _Vector6,
};

pub type T = f32;
pub type Vector3 = _Vector3<T>;
pub type Vector6 = _Vector6<T>;
pub type Matrix6 = _Matrix6<T>;
pub type Matrix3x6 = _Matrix3x6<T>;
pub type Matrix6x3 = _Matrix6x3<T>;
pub type Matrix3 = _Matrix3<T>;
