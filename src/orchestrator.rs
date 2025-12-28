use crate::error::KalmanError;
use crate::types::{T, Vector3};
use nalgebra::vector;

use crate::client::Client;
use crate::kalman::Kalman;
use crate::kalman_view::KalmanView;

pub struct Orchestrator<V>
where
    V: KalmanView,
{
    client: Client,
    filter: Kalman,
    view: V,
}

enum Step {
    Break,
    Continue,
}

impl<V> Orchestrator<V>
where
    V: KalmanView,
{
    pub fn new(server_addr: &'static str, logger: V) -> Result<Orchestrator<V>, KalmanError> {
        Ok(Orchestrator {
            client: Client::new(server_addr)?,
            filter: Kalman::new(),
            view: logger,
        })
    }

    pub fn start(&mut self) -> Result<(), KalmanError> {
        self.client.start()
    }

    pub fn run(&mut self) -> Result<(), KalmanError> {
        self.process_init_msg()?;
        self.view.start();
        loop {
            match self.step()? {
                Step::Break => break,
                Step::Continue => continue,
            }
        }
        self.view.end();
        Ok(())
    }

    fn step(&mut self) -> Result<Step, KalmanError> {
        match self.client.recv_into_buf()?.to_string().as_str() {
            "MSG_START" => Ok(Step::Continue),
            "MSG_END" => {
                self.send_pos()?;
                Ok(Step::Continue)
            }
            "GOODBYE." => Ok(Step::Break),
            message => {
                self.handle_message(message)?;
                Ok(Step::Continue)
            }
        }
    }

    fn handle_message(&mut self, message: &str) -> Result<(), KalmanError> {
        match Self::vec_of_iter(message)? {
            ("POSITION", pos) => {
                self.view.print_message(message);
                self.filter.correction(pos)
            }
            ("ACCELERATION", acc) => self.filter.prediction(acc),
            _ => Ok(()),
        }
    }

    fn process_init_msg(&mut self) -> Result<(), KalmanError> {
        let _start = self.client.recv_into_buf()?;
        let (_, pos) = Self::vec_of_iter(self.client.recv_into_buf()?)?;
        let (_, speed) = Self::scal_of_iter(self.client.recv_into_buf()?)?;
        let _ = self.client.recv_into_buf()?;
        let (_, dir) = Self::vec_of_iter(self.client.recv_into_buf()?)?;
        let _end = self.client.recv_into_buf()?;
        self.filter.init(pos, speed, dir);
        self.send_pos()
    }

    fn vec_of_iter(message: &str) -> Result<(&str, Vector3), KalmanError> {
        let mut it = message.split_at(14).1.split('\n');
        match (it.next(), it.next(), it.next(), it.next()) {
            (Some(a), Some(x), Some(y), Some(z)) => Ok((
                a,
                vector![x.parse::<T>()?, y.parse::<T>()?, z.parse::<T>()?],
            )),
            _ => Err(KalmanError::Parsing),
        }
    }

    fn scal_of_iter(message: &str) -> Result<(&str, T), KalmanError> {
        let mut it = message.split_at(14).1.split('\n');
        match (it.next(), it.next()) {
            (Some(a), Some(x)) => Ok((a, x.parse::<T>()?)),
            _ => Err(KalmanError::Parsing),
        }
    }
    fn send_pos(&mut self) -> Result<(), KalmanError> {
        let x = self.filter.get_pos();
        self.client.send_position(x.x, x.y, x.z)
    }
}
