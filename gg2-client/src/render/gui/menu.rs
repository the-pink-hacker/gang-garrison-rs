use std::{ops::DerefMut, time::Duration};

use bevy::{core::FrameCount, prelude::*, time::common_conditions::on_real_timer};
use bevy_egui::{egui, EguiContexts};
use bevy_rapier2d::render::DebugRenderContext;
use elements::*;
use gg2_common::player::{class::ClassGeneric, team::Team};

use crate::{
    config::ClientConfig,
    player::ClientPlayer,
    state::{ClientState, InGameDebugState, InGamePauseState},
};

mod elements;

fn main_system(
    mut contexts: EguiContexts,
    mut exit_event: EventWriter<AppExit>,
    mut client_state: ResMut<NextState<ClientState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Gang Garrison 2: Rust");

        ui.add_enabled(false, egui::Button::new("Host Game"));

        if ui.button("Join").clicked() {
            info!("Joining server...");
            client_state.set(ClientState::InGame);
        }

        ui.add_enabled_ui(false, |ui| {
            let _ = ui.button("Options");
            let _ = ui.button("Credits");
        });

        if ui.button("Visit The Forums").clicked() {
            info!("Opening forum url in browser: https://www.ganggarrison.com/forums/index.php");
            ctx.open_url(egui::OpenUrl::new_tab(
                "https://www.ganggarrison.com/forums/index.php",
            ));
        }

        create_quit_button(ui, &mut exit_event);
    });
}

fn pause_system(
    mut contexts: EguiContexts,
    mut exit_event: EventWriter<AppExit>,
    mut client_state: ResMut<NextState<ClientState>>,
    mut pause_state: ResMut<NextState<InGamePauseState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button("Resume").clicked() {
            pause_state.set(InGamePauseState::None);
        }

        if create_confirm_button(ui, "Leave Game", "Are you sure?", "Leave Game") {
            client_state.set(ClientState::Menus);
        }

        create_quit_button(ui, &mut exit_event);
    });
}

#[allow(clippy::too_many_arguments)]
fn in_game_debug_sytem(
    mut contexts: EguiContexts,
    config: Res<ClientConfig>,
    client_player_query: Query<(Option<&Team>, Option<&ClassGeneric>), With<ClientPlayer>>,
    mut team_selection: Local<Team>,
    mut class_selection: Local<ClassGeneric>,
    mut debug_render_context: ResMut<DebugRenderContext>,
    time: Res<Time<Real>>,
    delta_time_one_percent: Res<DeltaTimeOnePercent>,
    delta_time_average: Res<DeltaTimeAverage>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("In Game Debugging")
        .default_width(250.0)
        .show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show_viewport(ui, |ui, _viewport| {
                    ui.collapsing("Physics", |ui| {
                        ui.checkbox(&mut debug_render_context.enabled, "Debug Renderer");
                    });

                    ui.collapsing("Player", |ui| {
                        ui.label(format!("Player Name: {}", config.game.player_name));

                        let (team, class) = client_player_query.get_single().unwrap_or_default();
                        let team = team
                            .map(Team::to_string)
                            .unwrap_or_else(|| "UNKNOWN".to_string());
                        let class = class
                            .map(ClassGeneric::to_string)
                            .unwrap_or_else(|| "UNKNOWN".to_string());
                        ui.label(format!("Player Team: {}", team));
                        ui.label(format!("Player Class: {}", class));

                        let current_team = team_selection.deref_mut();

                        egui::containers::ComboBox::from_label("Select Player Team")
                            .selected_text(format!("{}", current_team))
                            .show_ui(ui, |ui| {
                                for team in enum_iterator::all::<Team>() {
                                    ui.selectable_value(
                                        current_team,
                                        team,
                                        format!("{}", current_team),
                                    );
                                }
                            });

                        if ui.button("Update Team").clicked() {}

                        let current_class = class_selection.deref_mut();

                        egui::containers::ComboBox::from_label("Select Player Class")
                            .selected_text(format!("{}", current_class))
                            .show_ui(ui, |ui| {
                                for class in enum_iterator::all::<ClassGeneric>() {
                                    ui.selectable_value(current_class, class, format!("{}", class));
                                }
                            });

                        if ui.button("Update Class").clicked() {}
                    });

                    ui.collapsing("Rendering", |ui| {
                        let delta_time_seconds = time.delta_seconds();
                        let delta_time_one_percent = delta_time_one_percent.last_frame;
                        let delta_time_average = delta_time_average.last_delta;

                        ui.heading("Frame Timings");

                        let fps = 1.0 / delta_time_seconds;
                        let fps_average = 1.0 / delta_time_average;
                        let fps_one_percent = 1.0 / delta_time_one_percent;

                        egui::Grid::new("fps").show(ui, |ui| {
                            ui.label("");
                            ui.label("FPS");
                            ui.label("MS");
                            ui.end_row();

                            [
                                ("Current", fps, delta_time_seconds * 1_000.0),
                                ("Average", fps_average, delta_time_average * 1_000.0),
                                ("1% Lows", fps_one_percent, delta_time_one_percent * 1_000.0),
                            ]
                            .into_iter()
                            .for_each(
                                |(row_label, fps, delta_time_mili)| {
                                    ui.label(row_label);
                                    ui.label(format!("{:.2}", fps));
                                    ui.label(format!("{:.2}", delta_time_mili));
                                    ui.end_row();
                                },
                            );
                        });
                    });
                });
        });
}

/// The slowest delta time out of 100 frames.
#[derive(Resource, Default)]
struct DeltaTimeOnePercent {
    last_frame: f32,
    current_frame: f32,
}

fn update_delta_one_percent_system(
    mut delta_time: ResMut<DeltaTimeOnePercent>,
    time: Res<Time<Real>>,
    frame_count: Res<FrameCount>,
) {
    if frame_count.0 % 100 == 0 {
        delta_time.last_frame = delta_time.current_frame;
        delta_time.current_frame = 0.0;
    } else {
        let current_frame = time.delta_seconds();

        // Higher means slower
        if current_frame > delta_time.current_frame {
            delta_time.current_frame = current_frame;
        }
    }
}

#[derive(Resource, Default)]
struct DeltaTimeAverage {
    last_delta: f32,
    current_delta: f32,
    current_frames: u32,
}

fn update_delta_average_system(mut delta_time: ResMut<DeltaTimeAverage>, time: Res<Time<Real>>) {
    let total_delta = delta_time.current_delta * delta_time.current_frames as f32;
    delta_time.current_frames += 1;
    let average_delta = (total_delta + time.delta_seconds()) / delta_time.current_frames as f32;
    delta_time.current_delta = average_delta;
}

fn update_delta_average_commit_system(mut delta_time: ResMut<DeltaTimeAverage>) {
    delta_time.last_delta = delta_time.current_delta;
    delta_time.current_frames = 0;
    delta_time.current_delta = 0.0;
}

pub struct GuiMenuPlugin;

impl Plugin for GuiMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DeltaTimeOnePercent>()
            .init_resource::<DeltaTimeAverage>()
            .add_systems(
                Update,
                (
                    main_system.run_if(in_state(ClientState::Menus)),
                    pause_system.run_if(in_state(InGamePauseState::Paused)),
                    (
                        (
                            update_delta_one_percent_system,
                            update_delta_average_commit_system
                                .run_if(on_real_timer(Duration::from_secs(1)))
                                .before(update_delta_average_system),
                            update_delta_average_system,
                        )
                            .before(in_game_debug_sytem),
                        in_game_debug_sytem,
                    )
                        .run_if(
                            in_state(InGamePauseState::None)
                                .and_then(in_state(InGameDebugState::Enabled)),
                        ),
                ),
            );
    }
}
