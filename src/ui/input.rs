use egui::Key;

use super::NitrousGUI;

impl NitrousGUI {
    pub fn handle_input(&mut self, ctx: &egui::Context) {
        let keyinput = &mut self.emulator.shared.keyinput;
        ctx.input(|i| {
            keyinput.set_button_a(!i.key_down(Key::X));
            keyinput.set_button_b(!i.key_down(Key::Z));
            keyinput.set_button_select(!i.modifiers.shift);
            keyinput.set_button_start(!i.key_down(Key::Enter));
            keyinput.set_button_right(!i.key_down(Key::ArrowRight));
            keyinput.set_button_left(!i.key_down(Key::ArrowLeft));
            keyinput.set_button_up(!i.key_down(Key::ArrowUp));
            keyinput.set_button_down(!i.key_down(Key::ArrowDown));
            keyinput.set_button_r(!i.key_down(Key::C));
            keyinput.set_button_l(!i.key_down(Key::D));
        });
    }
}
