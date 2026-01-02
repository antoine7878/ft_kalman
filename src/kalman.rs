use crate::{
    error::KalmanError,
    types::{Matrix3, Matrix3x6, Matrix6, Matrix6x3, Vector3, Vector6, T},
};
use nalgebra::{matrix, vector, Rotation3};

const DT: T = 0.01;
const MULT: T = 50. * 2.5;
const S_ACC: T = 1e-3 * MULT;
const S_GYR: T = 1e-2 * MULT;
const S_GPS: T = 1e-1 * MULT;

const S_ACC2: T = S_ACC * S_ACC;
const S_GYR2: T = S_GYR * S_GYR;
const S_GPS2: T = S_GPS * S_GPS;

const DD: T = 0.5 * DT * DT;
const KMH_TO_MS: T = 1000. / 3600.;

const A: Matrix6 = matrix![
    1.,0.,0.,DT,0.,0.;
    0.,1.,0.,0.,DT,0.;
    0.,0.,1.,0.,0.,DT;
    0.,0.,0.,1.,0.,0.;
    0.,0.,0.,0.,1.,0.;
    0.,0.,0.,0.,0.,1.
];

const H: Matrix3x6 = matrix![
    1.,0.,0.,0.,0.,0.;
    0.,1.,0.,0.,0.,0.;
    0.,0.,1.,0.,0.,0.;
];

const B: Matrix6x3 = matrix![
    DD,0.,0.;
    0.,DD,0.;
    0.,0.,DD;
    DT,0.,0.;
    0.,DT,0.;
    0.,0.,DT;
];

const P: Matrix6 = matrix![
    S_GPS2, 0., 0., 0., 0., 0.;
    0., S_GPS2, 0., 0., 0., 0.;
    0., 0., S_GPS2, 0., 0., 0.;
    0., 0., 0., S_GYR2 + S_ACC2 * DT, 0., 0.;
    0., 0., 0., 0., S_GYR2 + S_ACC2 * DT, 0. ;
    0., 0., 0., 0., 0., S_GYR2 + S_ACC2 * DT
];

const R: Matrix3 = matrix![
    S_GPS2, 0., 0.;
    0., S_GPS2, 0.;
    0., 0., S_GPS2
];

#[derive(Default, Debug)]
pub struct Kalman {
    x: Vector6, // State
    p: Matrix6, // State covariance
    p_diag: [T; 6],
    a: Matrix6, // State transition
    a_t: Matrix6,
    h: Matrix3x6, // State to mesurment
    h_t: Matrix6x3,
    r: Matrix3,   // Mesurement covariance
    q: Matrix6,   // Process Noise Covariance
    k: Matrix6x3, // Kalaman gain
    b: Matrix6x3, // Control-input
    innovation: Vector3,
    nis: T,
}
impl Kalman {
    pub fn new() -> Kalman {
        let q = B * B.transpose() * S_ACC2;
        Kalman {
            x: Vector6::zeros(),
            p: P,
            k: Matrix6x3::zeros(),
            a: A,
            a_t: A.transpose(),
            h: H,
            h_t: H.transpose(),
            q,
            r: R,
            b: B,
            ..Default::default()
        }
    }

    pub fn init(&mut self, pos: Vector3, speed: T, dir: Vector3) {
        let rot = Rotation3::from_euler_angles(dir[0], dir[1], dir[2]);
        let v0 = rot * vector![speed * KMH_TO_MS, 0., 0.];
        self.x = vector!(pos.x, pos.y, pos.z, v0.x, v0.y, v0.z,);
    }

    pub fn prediction(&mut self, acc: &Vector3) -> Result<(), KalmanError> {
        self.x = self.a * self.x + self.b * acc;
        self.p = self.a * self.p * self.a_t + self.q;
        self.p_diag.copy_from_slice(self.p.diagonal().as_slice());
        Ok(())
    }

    pub fn correction(&mut self, z: &Vector3) -> Result<(), KalmanError> {
        let mut tmp = self.h * self.p * self.h_t + self.r;
        tmp = tmp.pseudo_inverse(1e-5)?;
        self.k = self.p * self.h_t * tmp;
        self.innovation = z - self.h * self.x;
        self.nis = (self.innovation.transpose()
            * (self.h * self.p * self.h_t + self.r).pseudo_inverse(1e-5)?
            * self.innovation)
            .x;
        self.x += self.k * self.innovation;
        self.p -= self.k * self.h * self.p;
        self.p_diag.copy_from_slice(self.p.diagonal().as_slice());
        Ok(())
    }

    pub fn get_state(&self) -> &[T] {
        self.x.as_slice()
    }

    pub fn get_state_variance(&self) -> &[T; 6] {
        &self.p_diag
    }

    pub fn get_innovation(&self) -> &[T] {
        self.innovation.as_slice()
    }
    pub fn get_nis(&self) -> T {
        self.nis
    }
}
