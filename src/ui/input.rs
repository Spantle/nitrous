use egui::Key;

use super::NitrousGUI;

impl NitrousGUI {
    pub fn handle_input(&mut self, ctx: &egui::Context) {
        let keyinput = &mut self.emulator.bus.keyinput;
        ctx.input(|i| {
            keyinput.set_button_select(i.modifiers.shift);
            keyinput.set_button_start(i.key_down(Key::Enter));
            keyinput.set_button_up(i.key_down(Key::ArrowUp));
            keyinput.set_button_down(i.key_down(Key::ArrowDown));
        });
    }
}
