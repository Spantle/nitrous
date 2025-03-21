use std::collections::VecDeque;

use crate::nds::{interrupts::Interrupts, Bits};

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct IpcFifo {
    cnt9: IpcFifoCnt,
    cnt7: IpcFifoCnt,
    send_queue9: VecDeque<u32>,
    send_queue7: VecDeque<u32>,
    recent9: u32,
    recent7: u32,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
struct IpcFifoCnt {
    send_fifo_empty_irq: bool,
    receive_fifo_not_empty_irq: bool,
    error: bool,
    enabled: bool,
}

impl IpcFifo {
    pub fn set_cnt<const ARM_BOOL: bool>(&mut self, interrupts: &mut Interrupts, value: u32) {
        let (cnt, send_queue, receive_queue) = if ARM_BOOL {
            (&mut self.cnt9, &mut self.send_queue9, &mut self.send_queue7)
        } else {
            (&mut self.cnt7, &mut self.send_queue7, &mut self.send_queue9)
        };

        interrupts.f.set_ipc_send_fifo_empty(
            !cnt.send_fifo_empty_irq && value.get_bit(2) && send_queue.is_empty(),
        );
        interrupts.f.set_ipc_receive_fifo_not_empty(
            !cnt.receive_fifo_not_empty_irq && value.get_bit(10) && !receive_queue.is_empty(),
        );

        if value.get_bit(3) {
            send_queue.clear();
        }

        cnt.send_fifo_empty_irq = value.get_bit(2);
        cnt.receive_fifo_not_empty_irq = value.get_bit(10);
        cnt.error &= !value.get_bit(14);
        cnt.enabled = value.get_bit(15);
    }

    pub fn get_cnt<const ARM_BOOL: bool>(&self) -> u32 {
        let (cnt, send_queue, receive_queue) = if ARM_BOOL {
            (&self.cnt9, &self.send_queue9, &self.send_queue7)
        } else {
            (&self.cnt7, &self.send_queue7, &self.send_queue9)
        };

        let mut value = 0;

        value.set_bit(0, send_queue.is_empty());
        value.set_bit(1, send_queue.len() == 16);
        value.set_bit(2, cnt.send_fifo_empty_irq);

        value.set_bit(8, receive_queue.is_empty());
        value.set_bit(9, receive_queue.len() == 16);
        value.set_bit(10, cnt.receive_fifo_not_empty_irq);

        value.set_bit(14, cnt.error);
        value.set_bit(15, cnt.enabled);

        value
    }

    pub fn send<const ARM_BOOL: bool>(&mut self, value: u32) {
        let (cnt, send_queue) = if ARM_BOOL {
            (&mut self.cnt9, &mut self.send_queue9)
        } else {
            (&mut self.cnt7, &mut self.send_queue7)
        };

        if !cnt.enabled {
            return;
        }

        if send_queue.len() == 16 {
            send_queue.pop_back();
        }
        send_queue.push_back(value);
        cnt.error = send_queue.len() == 16;
    }

    pub fn receive<const ARM_BOOL: bool>(&mut self) -> u32 {
        let (cnt, receive_queue, recent) = if ARM_BOOL {
            (&mut self.cnt9, &mut self.send_queue7, &mut self.recent9)
        } else {
            (&mut self.cnt7, &mut self.send_queue9, &mut self.recent7)
        };

        if !cnt.enabled {
            return *recent;
        }

        if receive_queue.is_empty() {
            cnt.error = true;
            return *recent;
        }

        let value = receive_queue.pop_front().unwrap();
        *recent = value;
        value
    }

    pub fn update_interrupts(
        &mut self,
        interrupts9: &mut Interrupts,
        interrupts7: &mut Interrupts,
    ) {
        interrupts9.f.falsy_set_ipc_send_fifo_empty(
            self.cnt9.send_fifo_empty_irq && self.send_queue9.is_empty(),
        );
        interrupts9.f.falsy_set_ipc_receive_fifo_not_empty(
            self.cnt9.receive_fifo_not_empty_irq && !self.send_queue7.is_empty(),
        );

        interrupts7.f.falsy_set_ipc_send_fifo_empty(
            self.cnt7.send_fifo_empty_irq && self.send_queue7.is_empty(),
        );
        interrupts7.f.falsy_set_ipc_receive_fifo_not_empty(
            self.cnt7.receive_fifo_not_empty_irq && !self.send_queue9.is_empty(),
        );
    }
}
