use egui::Context;
use egui_wgpu::{Renderer, ScreenDescriptor};
use wgpu::*;
use winit::{event::WindowEvent, window::Window};

use crate::prelude::*;

const PIXELS_PER_POINT: f32 = 1.0;

pub struct GuiRenderer {
    renderer: Renderer,
    screen_descriptor: ScreenDescriptor,
    egui_context: Context,
    egui_winit_state: egui_winit::State,
}

impl GuiRenderer {
    pub fn new(device: &Device, game_size: UVec2, window: &Window) -> Self {
        let renderer = Renderer::new(
            device,
            super::SCREEN_FORMAT,
            Some(super::DEPTH_FORMAT),
            1,
            false,
        );
        let screen_descriptor = Self::create_screen_descriptor(game_size);

        let egui_context = egui::Context::default();

        let egui_winit_state = egui_winit::State::new(
            egui_context.clone(),
            egui_context.viewport_id(),
            window,
            Some(PIXELS_PER_POINT),
            None,
            None,
        );

        Self {
            renderer,
            screen_descriptor,
            egui_context,
            egui_winit_state,
        }
    }

    #[inline]
    fn create_screen_descriptor(game_size: UVec2) -> ScreenDescriptor {
        ScreenDescriptor {
            size_in_pixels: game_size.into(),
            pixels_per_point: PIXELS_PER_POINT,
        }
    }

    pub fn resize(&mut self, game_size: UVec2) {
        self.screen_descriptor = Self::create_screen_descriptor(game_size);
    }

    pub fn render_pass(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        game_view: &TextureView,
        depth_texture: &Texture,
        window: &Window,
    ) {
        let depth_view = depth_texture.create_view(&TextureViewDescriptor {
            format: Some(super::DEPTH_FORMAT),
            ..Default::default()
        });

        let mut render_pass = encoder
            .begin_render_pass(&RenderPassDescriptor {
                label: Some("EGUI Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: game_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Load,
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            })
            .forget_lifetime();

        self.render(device, queue, encoder, &mut render_pass, window);
    }

    fn render(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        render_pass: &mut RenderPass<'static>,
        window: &Window,
    ) {
        // egui-winit wiki:
        //
        // You need to set egui::RawInput::viewports yourself though.
        // Use update_viewport_info to update the info for each viewport.
        let raw_input = self.egui_winit_state.take_egui_input(window);

        let full_output = self.egui_context.run(raw_input, |ctx| {
            egui::SidePanel::left("left_panel").show(ctx, |ui| {
                egui::ScrollArea::both()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.label("Hello, World!");

                        if ui.button("This is a button").clicked() {
                            panic!("HL3 Confirmed!!!")
                        }
                    });
            });
        });

        self.egui_winit_state
            .handle_platform_output(window, full_output.platform_output);

        full_output
            .textures_delta
            .set
            .iter()
            .for_each(|(id, image_delta)| {
                self.renderer
                    .update_texture(device, queue, *id, image_delta);
            });

        let paint_jobs = self
            .egui_context
            .tessellate(full_output.shapes, self.screen_descriptor.pixels_per_point);

        self.renderer
            .update_buffers(device, queue, encoder, &paint_jobs, &self.screen_descriptor);

        self.renderer
            .render(render_pass, &paint_jobs, &self.screen_descriptor);
    }
}

impl super::State {
    /// Returns true if event was consumed
    pub fn on_window_event(&mut self, event: &WindowEvent) -> bool {
        let response = self
            .gui
            .egui_winit_state
            .on_window_event(&self.window, event);

        if response.repaint {
            self.gui.egui_context.request_repaint();
        }

        response.consumed
    }

    pub fn on_mouse_motion(&mut self, delta: (f64, f64)) {
        self.gui.egui_winit_state.on_mouse_motion(delta);
    }

    /// When the screen is cropped, the mouse position is no longer correct.
    /// This fixes `WindowEvent::CursorMoved` to have the right position.
    pub fn correct_cursor_position(&self, event: &mut WindowEvent) {
        if let WindowEvent::CursorMoved { position, .. } = event {
            position.x -= self.size_difference_half.x as f64;
            position.y -= self.size_difference_half.y as f64;
        }
    }
}
