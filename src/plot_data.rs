use crate::types::T;

#[derive(Default)]
pub struct PlotData {
    pub x: Vec<T>,
    pub y: Vec<T>,
    pub z: Vec<T>,

    pub x_gps: Vec<T>,
    pub y_gps: Vec<T>,
    pub z_gps: Vec<T>,

    pub vx: Vec<T>,
    pub vy: Vec<T>,
    pub vz: Vec<T>,

    pub x_unc: Vec<T>,
    pub y_unc: Vec<T>,
    pub z_unc: Vec<T>,

    pub vx_unc: Vec<T>,
    pub vy_unc: Vec<T>,
    pub vz_unc: Vec<T>,

    pub x_innov: Vec<T>,
    pub y_innov: Vec<T>,
    pub z_innov: Vec<T>,

    pub nis: Vec<T>,

    pub done: bool,
    max_size: usize,
    max_size_gps: usize,
}

impl PlotData {
    pub fn new(follow: bool) -> Self {
        let max_size = 300 * 20;
        Self {
            max_size,
            max_size_gps: max_size / if follow { 300 } else { 1 },
            x: Vec::with_capacity(max_size),
            y: Vec::with_capacity(max_size),
            z: Vec::with_capacity(max_size),

            vx: Vec::with_capacity(max_size),
            vy: Vec::with_capacity(max_size),
            vz: Vec::with_capacity(max_size),

            x_gps: Vec::with_capacity(max_size),
            y_gps: Vec::with_capacity(max_size),
            z_gps: Vec::with_capacity(max_size),

            x_unc: Vec::with_capacity(max_size),
            y_unc: Vec::with_capacity(max_size),
            z_unc: Vec::with_capacity(max_size),

            vx_unc: Vec::with_capacity(max_size),
            vy_unc: Vec::with_capacity(max_size),
            vz_unc: Vec::with_capacity(max_size),

            x_innov: Vec::with_capacity(max_size),
            y_innov: Vec::with_capacity(max_size),
            z_innov: Vec::with_capacity(max_size),

            nis: Vec::with_capacity(max_size),
            ..Default::default()
        }
    }

    pub fn push(
        &mut self,
        state: &[T],
        state_unc: &[T],
        innovation: &[T],
        gps: Option<&[T]>,
        nis: T,
    ) {
        if self.x.len() > self.max_size {
            let excess = self.x.len() - self.max_size;
            self.x.drain(0..excess);
            self.y.drain(0..excess);
            self.z.drain(0..excess);

            self.vx.drain(0..excess);
            self.vy.drain(0..excess);
            self.vz.drain(0..excess);

            self.x_unc.drain(0..excess);
            self.y_unc.drain(0..excess);
            self.z_unc.drain(0..excess);

            self.vx_unc.drain(0..excess);
            self.vy_unc.drain(0..excess);
            self.vz_unc.drain(0..excess);

            self.x_innov.drain(0..excess);
            self.y_innov.drain(0..excess);
            self.z_innov.drain(0..excess);

            self.nis.drain(0..excess);
        }

        if !self.x_gps.is_empty() && self.x_gps.len() > self.max_size_gps {
            let excess = self.x_gps.len() - self.max_size_gps;
            self.x_gps.drain(0..excess);
            self.y_gps.drain(0..excess);
            self.z_gps.drain(0..excess);
        }

        self.x.push(state[0]);
        self.y.push(state[1]);
        self.z.push(state[2]);

        self.vx.push(state[3]);
        self.vy.push(state[4]);
        self.vz.push(state[5]);

        self.x_unc.push(state_unc[0]);
        self.y_unc.push(state_unc[1]);
        self.z_unc.push(state_unc[2]);

        self.vx_unc.push(state_unc[3]);
        self.vy_unc.push(state_unc[4]);
        self.vz_unc.push(state_unc[5]);

        self.x_innov.push(innovation[0]);
        self.y_innov.push(innovation[1]);
        self.z_innov.push(innovation[2]);

        if let Some(gps) = gps {
            self.x_gps.push(gps[0]);
            self.y_gps.push(gps[1]);
            self.z_gps.push(gps[2]);
        }

        self.nis.push(nis);
    }
}
