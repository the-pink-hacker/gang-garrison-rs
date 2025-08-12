use crate::prelude::*;

pub trait ClientGamemodeState: GamemodeState {
    fn deserialize(
        &mut self,
        payload: &mut dyn Iterator<Item = u8>,
        kind: PacketKind,
    ) -> Result<(), CommonError>;

    fn render_hud(&self, ctx: &egui::Context);
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

    fn render_hud(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("gamemode_ctf_hud").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                let seconds_left = self.match_timer.left.as_secs();
                let minutes = seconds_left / 60;
                let seconds = seconds_left % 60;
                ui.label(format!("CTF: {minutes}:{seconds:.1}"));
                let percent = (self.match_timer.left.as_secs_f32()
                    / self.match_timer.total.as_secs_f32())
                    * 100.0;
                ui.label(format!("{percent:03.2}%"));
            });
        });
    }
}
