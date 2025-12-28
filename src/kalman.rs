use crate::{
    error::KalmanError,
    types::{Matrix3, Matrix3x6, Matrix6, Matrix6x3, T, Vector3, Vector6},
};
use nalgebra::{UnitQuaternion, matrix, vector};

const DT: T = 0.01;
const S_ACC: T = 1e-3;
const S_ACC2: T = S_ACC * S_ACC;
const S_GYR: T = 1e-2;
const S_GYR2: T = S_GYR * S_GYR;
const S_GPS: T = 1e-1;
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
    S_GPS2, 0., 0.; 0., S_GPS2, 0.;
    0., 0., S_GPS2
];

#[derive(Debug)]
pub struct Kalman {
    x: Vector6,
    p: Matrix6,
    a: Matrix6,
    a_t: Matrix6,
    h: Matrix3x6,
    h_t: Matrix6x3,
    r: Matrix3,
    q: Matrix6,
    k: Matrix6x3,
    b: Matrix6x3,
}

impl Kalman {
    pub fn new() -> Kalman {
        let q = B * B.transpose() * (S_ACC2 + S_GYR2);

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
        }
    }

    pub fn init(&mut self, pos: Vector3, speed: T, dir: Vector3) {
        let rot_q = UnitQuaternion::from_euler_angles(dir.z, dir.y, dir.x);
        let v0 = speed * KMH_TO_MS * rot_q.transform_vector(&Vector3::new(1., 0.0, 0.0));
        self.x = vector!(pos.x, pos.y, pos.z, v0.x, v0.y, v0.z,);
    }

    pub fn prediction(&mut self, acc: Vector3) -> Result<(), KalmanError> {
        self.x = self.a * self.x + self.b * acc;
        self.p = self.a * self.p * self.a_t + self.q;
        Ok(())
    }

    pub fn correction(&mut self, z: Vector3) -> Result<(), KalmanError> {
        let mut tmp = self.h * self.p * self.h_t + self.r;
        tmp = tmp.pseudo_inverse(1e-3)?;
        self.k = self.p * self.h_t * tmp;
        self.x += self.k * (z - self.h * self.x);
        self.p -= self.k * self.h * self.p;
        Ok(())
    }

    pub fn get_pos(&self) -> &Vector6 {
        &self.x
    }
}
