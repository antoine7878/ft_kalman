mod client;
mod error;
mod kalman;
mod kalman_view;
mod orchestrator;
mod types;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::kalman_view::CLIView;
use crate::orchestrator::Orchestrator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut orchestrator = Orchestrator::new("127.0.0.1:4242", CLIView {})?;

    orchestrator.start()?;

    let start = SystemTime::now().duration_since(UNIX_EPOCH)?;
    orchestrator.run()?;
    let end = SystemTime::now().duration_since(UNIX_EPOCH)?;
    println!("Finished in {}ms", (end - start).as_millis());
    println!("GOODBYE.");
    Ok(())
}

//
// use dear_app::{AddOnsConfig, RunnerConfig, run};
// use dear_imgui_rs::*;
// use dear_implot3d as implot3d;
// use implot3d::*;
// use rand::random_range;
// use std::{cell::RefCell, collections::VecDeque, thread, time::Duration};
//
// use crate::types::T;
//
// fn main() {
//     let runner = RunnerConfig {
//         window_title: "ft_kalman".to_string(),
//         window_size: (760.0, 900.0),
//         clear_color: [0.06, 0.08, 0.1, 1.0],
//         ..Default::default()
//     };
//     let addons = AddOnsConfig::auto();
//
//     run(runner, addons, move |ui, addons| {
//         let Some(plot_ctx) = addons.implot3d else {
//             ui.text("ImPlot3D add-on not enabled");
//             return;
//         };
//         let plot_ui = plot_ctx.get_plot_ui(ui);
//
//         ui.window("ImPlot3D Demo (Rust)")
//             .position([0., 0.], Condition::Always)
//             .flags(
//                 WindowFlags::NO_TITLE_BAR
//                     | WindowFlags::NO_DOCKING
//                     | WindowFlags::NO_RESIZE
//                     | WindowFlags::NO_COLLAPSE,
//             )
//             .size([760.0, 450.0], Condition::Always)
//             .build(|| demo_realtime_plots(ui, &plot_ui));
//     })
//     .unwrap();
// }
//
// struct ScrollingBuffer1D {
//     max_size: usize,
//     x: VecDeque<T>,
//     y: VecDeque<T>,
//     z: VecDeque<T>,
// }
//
// impl ScrollingBuffer1D {
//     fn new(max_size: usize) -> Self {
//         Self {
//             max_size,
//             x: VecDeque::new(),
//             y: VecDeque::new(),
//             z: VecDeque::new(),
//         }
//     }
//
//     fn add(&mut self, x: T, y: T, z: T) {
//         if self.x.len() > self.max_size {
//             self.x.pop_back();
//             self.y.pop_back();
//             self.z.pop_back();
//         }
//         self.x
//             .push_front(if !self.x.is_empty() { self.x[0] + x } else { x });
//         self.y
//             .push_front(if !self.y.is_empty() { self.y[0] + y } else { y });
//         self.z
//             .push_front(if !self.z.is_empty() { self.z[0] + z } else { z });
//     }
// }
//
// fn demo_realtime_plots(_: &Ui, plot_ui: &Plot3DUi) {
//     thread_local! {
//         static DATA: RefCell<ScrollingBuffer1D> = RefCell::new(ScrollingBuffer1D::new(3000));
//     }
//
//     let Some(_tok) = plot_ui
//         .begin_plot("Trajectory plot")
//         .size([-1.0, 400.0])
//         .build()
//     else {
//         return;
//     };
//
//     let flags = Axis3DFlags::NONE;
//
//     plot_ui.setup_axes("X", "Y", "Z", flags, flags, flags);
//     plot_ui.setup_axis_limits(Axis3D::X, -5_000., 5_000., Plot3DCond::Once);
//     plot_ui.setup_axis_limits(Axis3D::Y, -5_000., 5_000., Plot3DCond::Once);
//     plot_ui.setup_axis_limits(Axis3D::Z, -5_000., 5_000., Plot3DCond::Once);
//
//     DATA.with(|data| {
//         data.borrow_mut().add(
//             random_range(-100.0..100.0),
//             random_range(-100.0..100.0),
//             random_range(-100.0..100.0),
//         );
//         plot(plot_ui, data);
//     });
//
//     // thread::sleep(Duration::from_millis(16));
// }
//
// fn plot(plot_ui: &Plot3DUi, data: &RefCell<ScrollingBuffer1D>) {
//     let data = data.borrow();
//     let x = &data.x.as_slices();
//     let y = &data.y.as_slices();
//     let z = &data.z.as_slices();
//
//     let name = "Vehicule";
//     plot_ui.plot_line_f32(name, x.0, y.0, z.0, Line3DFlags::NONE);
//     plot_ui.plot_line_f32(name, x.1, y.1, z.1, Line3DFlags::NONE);
//
//     if x.1.is_empty() {
//         return;
//     }
//
//     plot_ui.plot_line_f32(
//         name,
//         &[x.0[x.0.len() - 1], x.1[0]],
//         &[y.0[y.0.len() - 1], y.1[0]],
//         &[z.0[z.0.len() - 1], z.1[0]],
//         Line3DFlags::NONE,
//     );
// }
