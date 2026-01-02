use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use crate::client::Client;
use crate::error::KalmanError;
use crate::kalman::Kalman;
use crate::log::{log_filer_pos, log_in_message};
use crate::message::Message;
use crate::plot_data::PlotData;
use crate::types::T;

pub struct Orchestrator {
    client: Client,
    filter: Kalman,
    plot_data: Option<Arc<Mutex<PlotData>>>,
    throttle: u64,
    verbose: bool,
    follow: bool,
}

impl Orchestrator {
    pub fn new(
        server_addr: &'static str,
        plot_data: Option<Arc<Mutex<PlotData>>>,
        throttle: u64,
        verbose: bool,
        follow: bool,
    ) -> Result<Orchestrator, KalmanError> {
        Ok(Orchestrator {
            client: Client::new(server_addr)?,
            filter: Kalman::new(),
            plot_data,
            throttle,
            verbose,
            follow,
        })
    }

    pub fn run(&mut self) -> Result<(), KalmanError> {
        self.client.start()?;
        self.process_init_msg()?;
        loop {
            let message = self.client.recv_into_buf()?;
            if self.verbose {
                log_in_message(&message);
            }
            match &message {
                Message::End => self.send_pos()?,
                Message::Goodbye => break,
                Message::TruePosition(pos) | Message::Position(pos) => {
                    self.update_plot_data(Some(pos.as_slice()));
                    self.filter.correction(pos)?;
                }
                Message::Acceleration(acc) => {
                    self.filter.prediction(acc)?;
                    if self.follow {
                        self.update_plot_data(None);
                    }
                }
                Message::Direction(_)
                | Message::Start
                | Message::Speed(_)
                | Message::Generation => continue,
            };
        }
        self.set_done();
        Ok(())
    }
    fn process_init_msg(&mut self) -> Result<(), KalmanError> {
        let _start = self.client.recv_into_buf()?;
        let pos = self.client.recv_into_buf()?;
        let speed = self.client.recv_into_buf()?;
        let _acc = self.client.recv_into_buf()?;
        let dir = self.client.recv_into_buf()?;
        let _end = self.client.recv_into_buf()?;

        match (pos, speed, dir) {
            (Message::TruePosition(pos), Message::Speed(speed), Message::Direction(dir)) => {
                self.filter.init(pos, speed, dir);
                self.send_pos()
            }
            _ => Err(KalmanError::Parsing("Bad inital messsage".into())),
        }
    }

    fn send_pos(&mut self) -> Result<(), KalmanError> {
        sleep(Duration::from_micros(self.throttle));
        let a = self.filter.get_state();
        if self.verbose {
            log_filer_pos(a);
        }
        self.client.send_position(a)
    }

    fn update_plot_data(&self, gps: Option<&[T]>) {
        if let Some(plot_data) = &self.plot_data
            && let Ok(mut plot_data) = plot_data.lock()
        {
            plot_data.push(
                self.filter.get_state(),
                self.filter.get_state_variance(),
                self.filter.get_innovation(),
                gps,
                self.filter.get_nis(),
            );
        };
    }

    fn set_done(&self) {
        if let Some(plot_data) = &self.plot_data
            && let Ok(mut plot_data) = plot_data.lock()
        {
            plot_data.done = true;
        }
    }
}
