use crate::message::Message;
use crate::types::T;
use color_print::cprintln;

pub fn log_in_message(message: &Message) {
    match message {
        Message::End => cprintln!("End"),
        Message::Start => cprintln!("Start"),
        Message::Generation => cprintln!("Trajectory Generated!"),
        Message::Goodbye => cprintln!("recv: Goodbye"),
        Message::TruePosition(v) => {
            cprintln!("<bright-yellow>True Pos: {:.4} {:.4} {:.4}</>", v.x, v.y, v.z)
        }
        Message::Position(v) => cprintln!("<magenta>Pos: {:.4} {:.4} {:.4}</>", v.x, v.y, v.z),
        Message::Direction(v) => cprintln!("<cyan>Dir: {:.4} {:.4} {:.4}</>", v.x, v.y, v.z),
        Message::Acceleration(v) => cprintln!("<green>Acc: {:.4} {:.4} {:.4}</>", v.x, v.y, v.z),
        Message::Speed(s) => cprintln!("Speed: {:.4}", s),
    }
}
pub fn log_filer_pos(state: &[T]) {
    cprintln!(
        "<blue>Kalman pos: {:.4} {:.4} {:.4}</>",
        state[0],
        state[1],
        state[2]
    );
}
