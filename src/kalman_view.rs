pub trait KalmanView {
    fn start(&self);
    fn print_message(&self, message: &str);
    fn end(&self);
}

pub struct CLIView {}

impl KalmanView for CLIView {
    fn start(&self) {
        println!();
    }
    fn print_message(&self, message: &str) {
        println!("                                 ");
        println!("                                 ");
        println!("                                 ");
        println!("                                 ");
        println!("                                 ");
        print!("\x1b[1A\x1b[1A\x1b[1A\x1b[1A\x1b[1A");
        println!("{}", message,);
        print!("\x1b[1A\x1b[1A\x1b[1A\x1b[1A\x1b[1A");
    }

    fn end(&self) {
        print!("\n\n\n\n\n");
    }
}

// pub struct GuiLogger {
// }
// impl Logger for GuiLogger{}
