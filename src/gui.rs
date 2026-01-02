use std::sync::{Arc, Mutex};

use dear_app::{run, AddOns, AddOnsConfig, RedrawMode, RunnerConfig};
use dear_imgui_rs::*;
use dear_implot::PlotUi;
use dear_implot3d::Plot3DContext;

use crate::plot_data::PlotData;

const TRAJECTORY_LABEL: &str = "Trajectory";
const POSITION_LABEL: &str = "Position";
const SPEED_LABEL: &str = "Speed";
const UNCERTAINTIES_LABEL: &str = "Uncertainty";
const INNOVATION_LABEL: &str = "Innovation";

pub struct GuiView {
    plot_data: Arc<Mutex<PlotData>>,
}

impl GuiView {
    pub fn new(plot_data: Arc<Mutex<PlotData>>) -> Self {
        Self { plot_data }
    }

    pub fn render(&self) {
        let runner = RunnerConfig {
            window_title: "kalman".to_string(),
            window_size: (1500.0, 1500.0),
            clear_color: [0.06, 0.08, 0.1, 1.0],
            redraw: RedrawMode::WaitUntil { fps: 30. },
            ..Default::default()
        };
        let addons = AddOnsConfig::auto();

        let plot_data = self.plot_data.clone();
        run(runner, addons, move |ui, addons| {
            ui.set_next_window_viewport(ui.main_viewport().id().into());
            let _padding = ui.push_style_var(StyleVar::WindowPadding([0.0, 0.0]));
            let _border = ui.push_style_var(StyleVar::WindowBorderSize(0.0));
            let _rounding = ui.push_style_var(StyleVar::WindowRounding(0.0));

            Self::render_main(ui, addons, &plot_data);
        })
        .unwrap();
    }

    fn render_main(ui: &Ui, addons: &mut AddOns, plot_data: &Arc<Mutex<PlotData>>) {
        let mut first = true;
        let vp = ui.main_viewport();

        ui.window("DockSpaceHost")
            .position(vp.work_pos(), Condition::Always)
            .size(vp.work_size(), Condition::Always)
            .flags(
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_COLLAPSE
                    | WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS
                    | WindowFlags::NO_NAV,
            )
            .build(|| {
                let dock_id_struct = ui.get_id("DockSpaceHost");
                if first {
                    Self::setup_initial_docking_layout(
                        dock_id_struct,
                        vp.work_pos(),
                        vp.work_size(),
                    );
                    first = false;
                }

                ui.dock_space_with_class(
                    dock_id_struct,
                    [0.0, 0.0],
                    DockNodeFlags::AUTO_HIDE_TAB_BAR,
                    None,
                );

                if let Some(plot3d_ctx) = addons.implot3d {
                    Self::draw_trajectory_panel(ui, plot3d_ctx, plot_data);
                };

                if let Some(plot_ctx) = addons.implot {
                    Self::render_position_panel(ui, &plot_ctx.get_plot_ui(ui), plot_data);
                    Self::render_speed_panel(ui, &plot_ctx.get_plot_ui(ui), plot_data);
                    Self::render_variance_panel(ui, &plot_ctx.get_plot_ui(ui), plot_data);
                    Self::render_innov_panel(ui, &plot_ctx.get_plot_ui(ui), plot_data);
                };
            });
    }
    fn setup_initial_docking_layout(
        dockspace_id: Id,
        viewport_work_pos: [f32; 2],
        viewport_work_size: [f32; 2],
    ) {
        // Clear any existing layout and create fresh dockspace (size comes from main viewport)
        DockBuilder::remove_node_docked_windows(dockspace_id, true);
        DockBuilder::remove_node(dockspace_id);
        DockBuilder::add_node(dockspace_id, DockNodeFlags::NO_UNDOCKING);
        // Match node pos/size to main viewport work area (exclude menu bars) before splitting
        {
            DockBuilder::set_node_pos(dockspace_id, viewport_work_pos);
            DockBuilder::set_node_size(dockspace_id, viewport_work_size);
        }

        let (left_id, right_1_id) =
            DockBuilder::split_node(dockspace_id, SplitDirection::Left, 1. / 2.);
        let (right_1_id, right_2_id) =
            DockBuilder::split_node(right_1_id, SplitDirection::Up, 1. / 4.);
        let (right_2_id, right_3_id) =
            DockBuilder::split_node(right_2_id, SplitDirection::Up, 1. / 3.);
        let (right_3_id, right_4_id) =
            DockBuilder::split_node(right_3_id, SplitDirection::Up, 1. / 2.);

        DockBuilder::dock_window(TRAJECTORY_LABEL, left_id);
        DockBuilder::dock_window(POSITION_LABEL, right_1_id);
        DockBuilder::dock_window(SPEED_LABEL, right_2_id);
        DockBuilder::dock_window(UNCERTAINTIES_LABEL, right_3_id);
        DockBuilder::dock_window(INNOVATION_LABEL, right_4_id);

        DockBuilder::finish(dockspace_id);
    }

    fn render_position_panel(ui: &Ui, plot_ui: &PlotUi, plot_data: &Arc<Mutex<PlotData>>) {
        use dear_implot::*;
        let flags = WindowFlags::NO_DECORATION | WindowFlags::NO_NAV | WindowFlags::NO_MOVE;
        ui.window(POSITION_LABEL).flags(flags).build(|| {
            let Some(plot) = plot_ui.begin_plot_with_size(POSITION_LABEL, [-1., -1.]) else {
                return;
            };
            let flags = AxisFlags::AUTO_FIT;
            plot_ui.setup_x_axis(XAxis::X1, Some("time"), flags);
            plot_ui.setup_y_axis(YAxis::Y1, Some("x (m)"), flags);
            plot_ui.setup_y_axis(YAxis::Y2, Some("y (m)"), flags);
            plot_ui.setup_y_axis(YAxis::Y3, Some("z (m)"), flags);
            if let Ok(plot_data) = plot_data.lock() {
                plot_ui.set_axes(XAxis::X1, YAxis::Y1);
                SimpleLinePlot::new("X", &plot_data.x).plot();
                plot_ui.set_axes(XAxis::X1, YAxis::Y2);
                SimpleLinePlot::new("Y", &plot_data.y).plot();
                plot_ui.set_axes(XAxis::X1, YAxis::Y3);
                SimpleLinePlot::new("Z", &plot_data.z).plot();
            }
            plot.end();
        });
    }

    fn render_speed_panel(ui: &Ui, plot_ui: &PlotUi, plot_data: &Arc<Mutex<PlotData>>) {
        use dear_implot::*;
        let flags = WindowFlags::NO_DECORATION | WindowFlags::NO_NAV | WindowFlags::NO_MOVE;
        ui.window(SPEED_LABEL).flags(flags).build(|| {
            let Some(plot) = plot_ui.begin_plot_with_size(SPEED_LABEL, [-1., -1.]) else {
                return;
            };
            let flags = AxisFlags::AUTO_FIT;
            plot_ui.setup_x_axis(XAxis::X1, Some("time"), flags);
            plot_ui.setup_y_axis(YAxis::Y1, Some("vx (m/s)"), flags);
            plot_ui.setup_y_axis(YAxis::Y2, Some("vy (m/s)"), flags);
            plot_ui.setup_y_axis(YAxis::Y3, Some("vz (m/s)"), flags);
            if let Ok(plot_data) = plot_data.lock() {
                plot_ui.set_axes(XAxis::X1, YAxis::Y1);
                SimpleLinePlot::new("X", &plot_data.vx).plot();
                plot_ui.set_axes(XAxis::X1, YAxis::Y2);
                SimpleLinePlot::new("Y", &plot_data.vy).plot();
                plot_ui.set_axes(XAxis::X1, YAxis::Y3);
                SimpleLinePlot::new("Z", &plot_data.vz).plot();
            }
            plot.end();
        });
    }

    fn render_variance_panel(ui: &Ui, plot_ui: &PlotUi, plot_data: &Arc<Mutex<PlotData>>) {
        use dear_implot::*;
        let flags = WindowFlags::NO_DECORATION | WindowFlags::NO_NAV | WindowFlags::NO_MOVE;
        ui.window(UNCERTAINTIES_LABEL).flags(flags).build(|| {
            let Some(plot) = plot_ui.begin_plot_with_size(UNCERTAINTIES_LABEL, [-1., -1.]) else {
                return;
            };
            let flags = AxisFlags::AUTO_FIT;
            plot_ui.setup_x_axis(XAxis::X1, Some("time"), flags);
            plot_ui.setup_y_axis(YAxis::Y1, Some("pos var (m)"), flags);
            plot_ui.setup_y_axis(YAxis::Y2, Some("speed var (m/s)"), flags);
            if let Ok(plot_data) = plot_data.lock() {
                plot_ui.set_axes(XAxis::X1, YAxis::Y1);
                SimpleLinePlot::new("X", &plot_data.x_unc).plot();
                SimpleLinePlot::new("Y", &plot_data.y_unc).plot();
                SimpleLinePlot::new("Z", &plot_data.z_unc).plot();

                plot_ui.set_axes(XAxis::X1, YAxis::Y2);
                SimpleLinePlot::new("X", &plot_data.vx_unc).plot();
                SimpleLinePlot::new("Y", &plot_data.vy_unc).plot();
                SimpleLinePlot::new("Z", &plot_data.vz_unc).plot();
            }
            plot.end();
        });
    }

    fn render_innov_panel(ui: &Ui, plot_ui: &PlotUi, plot_data: &Arc<Mutex<PlotData>>) {
        use dear_implot::*;
        let flags = WindowFlags::NO_DECORATION | WindowFlags::NO_NAV | WindowFlags::NO_MOVE;
        ui.window(INNOVATION_LABEL).flags(flags).build(|| {
            let Some(plot) = plot_ui.begin_plot_with_size(INNOVATION_LABEL, [-1., -1.]) else {
                return;
            };
            let flags = AxisFlags::AUTO_FIT;
            plot_ui.setup_x_axis(XAxis::X1, Some("time"), flags);
            plot_ui.setup_y_axis(YAxis::Y1, Some("Innovation (m)"), flags);
            plot_ui.setup_y_axis(YAxis::Y2, Some("NIS"), flags);
            if let Ok(plot_data) = plot_data.lock() {
                plot_ui.set_axes(XAxis::X1, YAxis::Y1);
                SimpleLinePlot::new("X", &plot_data.x_innov).plot();
                SimpleLinePlot::new("Y", &plot_data.y_innov).plot();
                SimpleLinePlot::new("Z", &plot_data.z_innov).plot();
                plot_ui.set_axes(XAxis::X1, YAxis::Y2);
                SimpleLinePlot::new("NIS", &plot_data.nis).plot();
            }
            plot.end();
        });
    }

    fn draw_trajectory_panel(
        ui: &Ui,
        plot3d_ctx: &Plot3DContext,
        plot_data: &Arc<Mutex<PlotData>>,
    ) {
        use dear_implot3d::*;
        let plot3d_ui = &plot3d_ctx.get_plot_ui(ui);
        let flags = WindowFlags::NO_DECORATION | WindowFlags::NO_NAV | WindowFlags::NO_MOVE;
        ui.window(TRAJECTORY_LABEL).flags(flags).build(|| {
            let Some(_plot) = plot3d_ui
                .begin_plot(TRAJECTORY_LABEL)
                .size([-1.0, -1.0])
                .flags(Plot3DFlags::NO_LEGEND | Plot3DFlags::NO_MENUS | Plot3DFlags::NO_TITLE)
                .build()
            else {
                return;
            };


            if let Ok(plot_data) = plot_data.lock() {
                let flags = if plot_data.done { Axis3DFlags::NONE} else {
                    Axis3DFlags::AUTO_FIT
                };
                // let flags = Axis3DFlags::AUTO_FIT;
                plot3d_ui.setup_axes("X", "Y", "Z", flags, flags, flags);

                set_next_line_style([0.282, 0.431, 0.671, 1.], 5.);
                plot3d_ui.plot_line_f64(
                    "KF",
                    &plot_data.x,
                    &plot_data.y,
                    &plot_data.z,
                    Line3DFlags::NONE,
                );
                set_next_marker_style(Marker3D::Cross, 4., [0.; 4], 4., [1., 0., 0., 0.3]);
                plot3d_ui.plot_scatter_f64(
                    "GPS",
                    &plot_data.x_gps,
                    &plot_data.y_gps,
                    &plot_data.z_gps,
                    Scatter3DFlags::NONE,
                );
            }
        });
    }
}
