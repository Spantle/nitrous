use std::collections::VecDeque;

use super::Registers;

const ARR_SIZE: usize = 16;

pub struct StackTrace {
    pub enabled: bool,
    branch_history: VecDeque<u32>,
    stack: VecDeque<u32>,
}

impl Default for StackTrace {
    fn default() -> StackTrace {
        StackTrace {
            enabled: true,
            branch_history: VecDeque::new(),
            stack: VecDeque::new(),
        }
    }
}

impl StackTrace {
    pub fn branch(&mut self, inst_addr: u32) {
        if self.enabled {
            self.branch_history.push_front(inst_addr);
            self.branch_history.limit();
            println!("Branching to {:#010X}", inst_addr);
        }
    }

    pub fn branch_link(&mut self, inst_addr: u32) {
        if self.enabled {
            self.branch_history.push_front(inst_addr);
            self.branch_history.limit();
            self.stack.push_front(inst_addr);
            self.stack.limit();
        }
    }

    // used to return from subroutine
    pub fn branch_exchange(&mut self, inst_addr: u32, dest_addr: u32) {
        if self.enabled {
            self.branch_history.push_front(inst_addr);
            self.branch_history.limit();

            // remove dest_addr from stack
            if let Some(pos) = self.stack.iter().position(|&x| x == dest_addr) {
                self.stack.truncate(pos);
            }
        }
    }

    pub fn generate(&self, registers: &Registers, reason: String) -> String {
        let mut result = (reason + "\nIn Stacktrace Branch Hst Reg Values\n").to_string();
        for i in 0..=(ARR_SIZE - 1) {
            let stack = if let Some(stack) = self.stack.get(i) {
                format!("{:#010X}", stack)
            } else {
                "          ".to_string()
            };
            let branch = if let Some(branch) = self.branch_history.get(i) {
                format!("{:#010X}", branch)
            } else {
                "          ".to_string()
            };
            let register = if i <= 15 {
                format!("{:#010X}", registers[i as u8])
            } else {
                "          ".to_string()
            };
            result.push_str(&format!("{:02} {} {} {}\n", i, stack, branch, register));
        }

        result
    }
}

trait Limit {
    fn limit(&mut self);
}

impl Limit for VecDeque<u32> {
    fn limit(&mut self) {
        if self.len() > ARR_SIZE {
            self.pop_back();
        }
    }
}
