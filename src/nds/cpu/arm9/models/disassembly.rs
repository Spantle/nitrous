pub struct Chunk {
    pub kind: ChunkKind,
    pub value: String,
}

pub enum ChunkKind {
    Register,
    Immediate,
    Modifier,
    Punctuation,
}

impl Chunk {
    pub fn new(kind: ChunkKind, value: String) -> Self {
        Self { kind, value }
    }
}

pub trait DisassemblyTrait {
    fn is_real(&self) -> bool;

    fn set_cond(&mut self, cond: [char; 2]);
    fn set_inst(&mut self, inst: &str);
    fn set_inst_suffix(&mut self, inst_suffix: &str);

    fn push_reg_arg(&mut self, reg: u8, suffix: Option<&str>);
    fn push_word_arg(&mut self, arg: u32);
    fn push_str_arg(&mut self, arg: &str);

    fn push_reg_end_arg(&mut self, reg: u8, prefix: Option<&str>);
    fn push_word_end_arg(&mut self, arg: u32, prefix: Option<&str>);
    fn push_str_end_arg(&mut self, arg: &str, prefix: Option<&str>);
}

pub struct Disassembly {
    pub cond: Option<[char; 2]>,
    pub inst: String,
    pub inst_suffix: String,
    pub args: Vec<Chunk>,
    pub end_args: Vec<Chunk>,
}

impl DisassemblyTrait for Disassembly {
    fn is_real(&self) -> bool {
        true
    }

    fn set_cond(&mut self, cond: [char; 2]) {
        self.cond = Some(cond);
    }

    fn set_inst(&mut self, inst: &str) {
        self.inst = inst.to_string();
    }

    fn set_inst_suffix(&mut self, inst_suffix: &str) {
        self.inst_suffix = inst_suffix.to_string();
    }

    fn push_reg_arg(&mut self, reg: u8, suffix: Option<&str>) {
        self.args
            .push(Chunk::new(ChunkKind::Register, format!("r{}", reg)));

        if let Some(suffix) = suffix {
            self.args
                .push(Chunk::new(ChunkKind::Modifier, suffix.to_string()));
        }
    }

    fn push_word_arg(&mut self, arg: u32) {
        self.args
            .push(Chunk::new(ChunkKind::Immediate, format!("#0x{:0X}", arg)));
    }

    fn push_str_arg(&mut self, arg: &str) {
        self.args
            .push(Chunk::new(ChunkKind::Punctuation, arg.to_string()));
    }

    fn push_reg_end_arg(&mut self, reg: u8, prefix: Option<&str>) {
        if let Some(prefix) = prefix {
            self.end_args
                .push(Chunk::new(ChunkKind::Punctuation, prefix.to_string()));
        }

        self.end_args
            .push(Chunk::new(ChunkKind::Register, format!("r{}", reg)));
    }

    fn push_word_end_arg(&mut self, arg: u32, prefix: Option<&str>) {
        if let Some(prefix) = prefix {
            self.end_args
                .push(Chunk::new(ChunkKind::Punctuation, prefix.to_string()));
        }

        self.end_args
            .push(Chunk::new(ChunkKind::Immediate, format!("#0x{:0X}", arg)));
    }

    fn push_str_end_arg(&mut self, arg: &str, prefix: Option<&str>) {
        if let Some(prefix) = prefix {
            self.end_args
                .push(Chunk::new(ChunkKind::Punctuation, prefix.to_string()));
        }

        self.end_args
            .push(Chunk::new(ChunkKind::Modifier, arg.to_string()));
    }
}

// impl From<Disassembly> for String {
//     fn from(disassembly: Disassembly) -> String {
//         let mut result = String::new();

//         result.push_str(&disassembly.inst);
//         if let Some(cond) = disassembly.cond {
//             result.push_str(&cond.iter().collect::<String>());
//         }
//         result.push_str(&disassembly.inst_suffix);

//         if !disassembly.args.is_empty() {
//             result.push(' ');
//             result.push_str(&disassembly.args.join(", "));
//         }

//         if !disassembly.end_args.is_empty() {
//             result.push_str(", ");
//             result.push_str(&disassembly.end_args);
//         }

//         result
//     }
// }

impl Default for Disassembly {
    fn default() -> Self {
        Self {
            cond: None,
            inst: "?????".to_string(),
            inst_suffix: "".to_string(),
            args: Vec::new(),
            end_args: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct FakeDisassembly;

impl DisassemblyTrait for FakeDisassembly {
    fn is_real(&self) -> bool {
        false
    }

    fn set_cond(&mut self, _cond: [char; 2]) {}
    fn set_inst(&mut self, _inst: &str) {}
    fn set_inst_suffix(&mut self, _inst_suffix: &str) {}
    fn push_reg_arg(&mut self, _reg: u8, _suffix: Option<&str>) {}
    fn push_word_arg(&mut self, _arg: u32) {}
    fn push_str_arg(&mut self, _arg: &str) {}
    fn push_reg_end_arg(&mut self, _reg: u8, _prefix: Option<&str>) {}
    fn push_word_end_arg(&mut self, _arg: u32, _prefix: Option<&str>) {}
    fn push_str_end_arg(&mut self, _arg: &str, _prefix: Option<&str>) {}
}
