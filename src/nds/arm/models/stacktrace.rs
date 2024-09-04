use limited_queue::LimitedQueue;

const ARR_SIZE: usize = 32;

pub struct StackTrace {
    pub enabled: bool,
    branch_history: LimitedQueue<u32>,
    stack: Vec<u32>,
}

impl Default for StackTrace {
    fn default() -> StackTrace {
        StackTrace {
            enabled: true,
            branch_history: LimitedQueue::with_capacity(ARR_SIZE),
            stack: Vec::new(),
        }
    }
}

impl StackTrace {
    pub fn branch(&mut self, inst_addr: u32) {
        if self.enabled {
            self.branch_history.push(inst_addr);
        }
    }

    pub fn branch_link(&mut self, inst_addr: u32) {
        if self.enabled {
            self.branch_history.push(inst_addr);
            self.stack.push(inst_addr);

            if self.stack.len() > ARR_SIZE {
                self.stack.remove(0);
            }
        }
    }

    // used to return from subroutine
    pub fn branch_exchange(&mut self, inst_addr: u32, dest_addr: u32) {
        if self.enabled {
            self.branch_history.push(inst_addr);

            // remove dest_addr from stack
            if let Some(pos) = self.stack.iter().position(|&x| x == dest_addr) {
                self.stack.truncate(pos);
            }
        }
    }

    pub fn generate(&self, reason: String) -> String {
        let mut result = (reason + "\nStacktrace:\n").to_string();
        for (i, addr) in self.stack.iter().enumerate() {
            result.push_str(&format!("{:02}: {:#08X}\n", i, addr));
        }

        result += "Branch History:\n";
        for (i, addr) in self.branch_history.iter().enumerate() {
            result.push_str(&format!("{:02}: {:#08X}\n", i, addr));
        }

        result
    }
}
