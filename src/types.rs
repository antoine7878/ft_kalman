use nalgebra::{SMatrix, SVector};

pub type T = f64;

pub type Vector3 = SVector<T, 3>;
pub type Vector6 = SVector<T, 6>;
pub type Matrix6 = SMatrix<T, 6, 6>;
pub type Matrix3x6 = SMatrix<T, 3, 6>;
pub type Matrix6x3 = SMatrix<T, 6, 3>;
pub type Matrix3 = SMatrix<T, 3, 3>;

pub fn string_of_vector3(vec: &Vector3) -> String {
    format!("{}, {}, {}", vec.x, vec.y, vec.z)
}
