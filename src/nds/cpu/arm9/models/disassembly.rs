pub trait DisassemblyTrait {
    fn set_cond(&mut self, cond: [char; 2]);
    fn set_inst(&mut self, inst: &str);
    fn set_inst_suffix(&mut self, inst_suffix: &str);

    fn push_reg_arg(&mut self, reg: u8, suffix: &str);
    fn push_word_arg(&mut self, arg: u32);
    fn push_str_arg(&mut self, arg: &str);

    fn push_reg_end_arg(&mut self, reg: u8, prefix: &str);
    fn push_word_end_arg(&mut self, arg: u32, prefix: &str);
    fn push_str_end_arg(&mut self, arg: &str, prefix: &str);
}

pub struct Disassembly {
    cond: Option<[char; 2]>,
    inst: String,
    inst_suffix: String,
    args: Vec<String>,
    end_args: String,
}

impl DisassemblyTrait for Disassembly {
    fn set_cond(&mut self, cond: [char; 2]) {
        self.cond = Some(cond);
    }

    fn set_inst(&mut self, inst: &str) {
        self.inst = inst.to_string();
    }

    fn set_inst_suffix(&mut self, inst_suffix: &str) {
        self.inst_suffix = inst_suffix.to_string();
    }

    fn push_reg_arg(&mut self, reg: u8, suffix: &str) {
        self.args.push(format!("r{}{}", reg, suffix));
    }

    fn push_word_arg(&mut self, arg: u32) {
        self.args.push(format!("#0x{:0X}", arg));
    }

    fn push_str_arg(&mut self, arg: &str) {
        self.args.push(arg.to_string());
    }

    fn push_reg_end_arg(&mut self, reg: u8, prefix: &str) {
        self.end_args.push_str(&format!("{}r{}", prefix, reg));
    }

    fn push_word_end_arg(&mut self, arg: u32, prefix: &str) {
        self.end_args.push_str(&format!("{}#0x{:0X}", prefix, arg));
    }

    fn push_str_end_arg(&mut self, arg: &str, prefix: &str) {
        self.end_args.push_str(&format!("{}{}", prefix, arg));
    }
}

impl From<Disassembly> for String {
    fn from(disassembly: Disassembly) -> String {
        let mut result = String::new();

        result.push_str(&disassembly.inst);
        if let Some(cond) = disassembly.cond {
            result.push_str(&cond.iter().collect::<String>());
        }
        result.push_str(&disassembly.inst_suffix);

        if !disassembly.args.is_empty() {
            result.push(' ');
            result.push_str(&disassembly.args.join(", "));
        }

        if !disassembly.end_args.is_empty() {
            result.push_str(", ");
            result.push_str(&disassembly.end_args);
        }

        result
    }
}

impl Default for Disassembly {
    fn default() -> Self {
        Self {
            cond: None,
            inst: "?????".to_string(),
            inst_suffix: "".to_string(),
            args: Vec::new(),
            end_args: "".to_string(),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct FakeDisassembly;

impl DisassemblyTrait for FakeDisassembly {
    fn set_cond(&mut self, _cond: [char; 2]) {}
    fn set_inst(&mut self, _inst: &str) {}
    fn set_inst_suffix(&mut self, _inst_suffix: &str) {}
    fn push_reg_arg(&mut self, _reg: u8, _suffix: &str) {}
    fn push_word_arg(&mut self, _arg: u32) {}
    fn push_str_arg(&mut self, _arg: &str) {}
    fn push_reg_end_arg(&mut self, _reg: u8, _prefix: &str) {}
    fn push_word_end_arg(&mut self, _arg: u32, _prefix: &str) {}
    fn push_str_end_arg(&mut self, _arg: &str, _prefix: &str) {}
}
