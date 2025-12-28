use crate::error::KalmanError;
use crate::types::T;
use std::fmt::Write;
use std::net::UdpSocket;
use std::time::Duration;

pub const MAX_LEN: usize = 1024;

#[derive(Debug)]
pub struct Client {
    server: &'static str,
    socket: UdpSocket,
    buf: [u8; MAX_LEN],
}

impl Client {
    pub fn new(server_ip: &'static str) -> Result<Client, KalmanError> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        Ok(Client {
            server: server_ip,
            socket,
            buf: [0; MAX_LEN],
        })
    }

    pub fn start(&mut self) -> Result<(), KalmanError> {
        self.socket
            .set_read_timeout(Some(Duration::from_millis(500)))?;
        self.socket.send_to(b"READY", self.server)?;
        loop {
            println!("Connection ...");
            match self.recv_into_buf() {
                Ok("Trajectory Generated!\nSending Info. . .\n") => break,
                Ok(msg) => println!("Received: {}", msg),
                Err(_) => continue,
            }
        }
        println!("Connected !");
        Ok(())
    }

    pub fn recv_into_buf(&mut self) -> Result<&str, KalmanError> {
        match self.socket.recv_from(&mut self.buf) {
            Ok((len, _)) if len >= MAX_LEN => Err(KalmanError::MessageTooLong(len)),
            Ok((len, _)) => Ok(str::from_utf8(&self.buf[..len])?),
            Err(e) => Err(KalmanError::Io(e)),
        }
    }

    pub fn send_position(&self, x: T, y: T, z: T) -> Result<(), KalmanError> {
        let mut msg = String::with_capacity(64);
        write!(&mut msg, "{} {} {}", x, y, z)?;
        self.socket.send_to(msg.as_bytes(), self.server)?;
        Ok(())
    }
}
