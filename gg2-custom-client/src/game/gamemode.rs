use crate::prelude::*;

pub trait ClientGamemodeState: GamemodeState {
    fn deserialize(
        &mut self,
        payload: &mut dyn Iterator<Item = u8>,
        kind: PacketKind,
    ) -> Result<(), CommonError>;

    fn render_hud(&self, ctx: &egui::Context, world: &'static ClientWorld);
}

impl ClientGamemodeState for CaptureTheFlagState {
    fn deserialize(
        &mut self,
        payload: &mut dyn Iterator<Item = u8>,
        _kind: PacketKind,
    ) -> Result<(), CommonError> {
        self.match_timer = HudMatchTimer::deserialize(payload)?;
        debug!("{self:#?}");

        Ok(())
    }

    fn render_hud(&self, ctx: &egui::Context, world: &'static ClientWorld) {
        egui::TopBottomPanel::top("gamemode_ctf_hud").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                let seconds_left = self.match_timer.left.as_secs();
                let minutes = seconds_left / 60;
                let seconds = seconds_left % 60;
                ui.label(format!("CTF: {minutes:02}:{seconds:02.2}"));
                let percent = (self.match_timer.left.as_secs_f32()
                    / self.match_timer.total.as_secs_f32())
                    * 100.0;
                ui.label(format!("{percent:03.2}%"));

                ui.label("RED: UNIMPLEMENTED");
                ui.label("BLU: UNIMPLEMENTED");
            });
        });
    }
}
