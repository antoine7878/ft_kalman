use kalman::{GuiView, Orchestrator, PlotData};

use std::sync::{Arc, Mutex};

use std::thread::{self, JoinHandle};
use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;
use color_print::cprintln;

#[derive(Parser)]
#[command(version, about, long_about = None, name="ft_kalman")]
pub(crate) struct Args {
    /// Activate GUI
    #[arg(short, long)]
    gui: bool,

    /// Log every send and reveived messages
    #[arg(short, long)]
    verbose: bool,

    /// Wait in between positions sending
    #[arg(short, long, default_value_t = 0)]
    throttle: u64,

    /// GUI with only print the last 20 min of the trajectory
    #[arg(short, long)]
    follow: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let start = SystemTime::now().duration_since(UNIX_EPOCH)?;

    let plot_data = if args.gui {
        Some(Arc::new(Mutex::new(PlotData::new(args.follow))))
    } else {
        None
    };

    let mut orchestrator = Orchestrator::new(
        "127.0.0.1:4242",
        plot_data.clone(),
        args.throttle,
        args.verbose,
        args.follow,
    )?;

    let thread_join_handle: JoinHandle<()> = thread::spawn(move || {
        if let Err(err) = orchestrator.run() {
            cprintln!("<red>{err}</>");
        }
    });

    if let Some(plot_data) = plot_data {
        GuiView::new(plot_data.clone()).render();
    }

    let _ = thread_join_handle.join();

    let end = SystemTime::now().duration_since(UNIX_EPOCH)?;
    println!("Finished in {}ms", (end - start).as_millis());
    Ok(())
}
