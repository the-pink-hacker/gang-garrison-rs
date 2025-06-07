use egui::Context;
use egui_wgpu::{Renderer, ScreenDescriptor};
use wgpu::*;

use crate::prelude::*;

pub struct GuiRenderer {
    renderer: Renderer,
    screen_descriptor: ScreenDescriptor,
    egui_context: Context,
}

impl GuiRenderer {
    pub fn new(device: &Device, game_size: UVec2) -> Self {
        let renderer = Renderer::new(
            device,
            super::SCREEN_FORMAT,
            Some(super::DEPTH_FORMAT),
            1,
            false,
        );
        let screen_descriptor = Self::create_screen_descriptor(game_size);

        let egui_context = egui::Context::default();

        Self {
            renderer,
            screen_descriptor,
            egui_context,
        }
    }

    #[inline]
    fn create_screen_descriptor(game_size: UVec2) -> ScreenDescriptor {
        ScreenDescriptor {
            size_in_pixels: game_size.into(),
            pixels_per_point: 1.0,
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

        self.render(device, queue, encoder, &mut render_pass);
    }

    fn render(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        render_pass: &mut RenderPass<'static>,
    ) {
        let raw_input = egui::RawInput::default();

        let full_output = self.egui_context.run(raw_input, |ctx| {
            egui::SidePanel::left("left_panel").show(ctx, |ui| {
                ui.label("Hello, World!");

                if ui.button("This is a button").clicked() {
                    panic!("HL3 Confirmed!!!")
                }
            });
        });

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
