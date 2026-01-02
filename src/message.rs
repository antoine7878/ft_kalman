use crate::{
    error::KalmanError,
    types::{string_of_vector3, Vector3, T},
};
use nalgebra::vector;
use std::{fmt, str::Split};

pub enum Message {
    Start,
    End,
    Generation,
    Goodbye,
    TruePosition(Vector3),
    Speed(T),
    Position(Vector3),
    Direction(Vector3),
    Acceleration(Vector3),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Message::End => write!(f, "End"),
            Message::Start => write!(f, "Start"),
            Message::Generation => write!(f, "Trajectory Generated!"),
            Message::Goodbye => write!(f, "Goodbye"),
            Message::TruePosition(v) => write!(f, "True Pos: {}", string_of_vector3(v)),
            Message::Position(v) => write!(f, "Pos: {}", string_of_vector3(v)),
            Message::Direction(v) => write!(f, "Dir: {}", string_of_vector3(v)),
            Message::Acceleration(v) => write!(f, "Acc: {}", string_of_vector3(v)),
            Message::Speed(s) => write!(f, "Speed: {}", s),
        }
    }
}
impl TryFrom<&str> for Message {
    type Error = KalmanError;

    fn try_from(msg: &str) -> Result<Message, Self::Error> {
        fn vec_of_it(it: &mut Split<'_, char>) -> Result<Vector3, KalmanError> {
            match (it.next(), it.next(), it.next()) {
                (Some(x), Some(y), Some(z)) => {
                    Ok(vector![x.parse::<T>()?, y.parse::<T>()?, z.parse::<T>()?])
                }
                _ => Err(KalmanError::Parsing("in vec_of_it".into())),
            }
        }
        match msg {
            "MSG_START" => Ok(Message::Start),
            "MSG_END" => Ok(Message::End),
            "GOODBYE." => Ok(Message::Goodbye),
            "Trajectory Generated!\nSending Info. . .\n" => Ok(Message::Generation),
            message => {
                let mut it = message.split_at(14).1.split('\n');
                match it.next() {
                    Some("POSITION") => Ok(Message::Position(vec_of_it(&mut it)?)),
                    Some("TRUE POSITION") => Ok(Message::TruePosition(vec_of_it(&mut it)?)),
                    Some("ACCELERATION") => Ok(Message::Acceleration(vec_of_it(&mut it)?)),
                    Some("DIRECTION") => Ok(Message::Direction(vec_of_it(&mut it)?)),
                    Some("SPEED") => Ok(Message::Speed(
                        it.next()
                            .ok_or(KalmanError::Parsing(message.into()))?
                            .parse::<T>()?,
                    )),
                    _ => Err(KalmanError::Parsing(message.into())),
                }
            }
        }
    }
}
