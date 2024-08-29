use limited_queue::LimitedQueue;

use crate::nds::{interrupts::Interrupts, Bits};

#[allow(clippy::upper_case_acronyms)]
pub struct IPCFIFO {
    cnt9: IPCFIFOCNT,
    cnt7: IPCFIFOCNT,
    send_queue9: LimitedQueue<u32>,
    send_queue7: LimitedQueue<u32>,
    recent9: u32,
    recent7: u32,

    pub request_send_fifo_empty_irq9: bool,
    pub request_send_fifo_empty_irq7: bool,
    pub request_receive_fifo_not_empty_irq9: bool,
    pub request_receive_fifo_not_empty_irq7: bool,
}

impl Default for IPCFIFO {
    fn default() -> Self {
        Self {
            cnt9: IPCFIFOCNT::default(),
            cnt7: IPCFIFOCNT::default(),
            send_queue9: LimitedQueue::with_capacity(16),
            send_queue7: LimitedQueue::with_capacity(16),
            recent9: 0,
            recent7: 0,

            request_send_fifo_empty_irq9: false,
            request_send_fifo_empty_irq7: false,
            request_receive_fifo_not_empty_irq9: false,
            request_receive_fifo_not_empty_irq7: false,
        }
    }
}

#[derive(Default)]
#[allow(clippy::upper_case_acronyms)]
struct IPCFIFOCNT {
    send_fifo_empty_irq: bool,
    receive_fifo_not_empty_irq: bool,
    error: bool,
    enabled: bool,
}

impl IPCFIFO {
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
        value.set_bit(1, send_queue.is_full());
        value.set_bit(2, cnt.send_fifo_empty_irq);

        value.set_bit(8, receive_queue.is_empty());
        value.set_bit(9, receive_queue.is_full());
        value.set_bit(10, cnt.receive_fifo_not_empty_irq);

        value.set_bit(14, cnt.error);
        value.set_bit(15, cnt.enabled);

        value
    }

    pub fn send<const ARM_BOOL: bool>(&mut self, value: u32) {
        let (cnt, send_queue, interrupt) = if ARM_BOOL {
            (
                &mut self.cnt9,
                &mut self.send_queue9,
                &mut self.request_receive_fifo_not_empty_irq7,
            )
        } else {
            (
                &mut self.cnt7,
                &mut self.send_queue7,
                &mut self.request_receive_fifo_not_empty_irq9,
            )
        };

        if !cnt.enabled {
            return;
        }

        cnt.error = send_queue.is_full();
        *interrupt = !send_queue.is_empty() && cnt.receive_fifo_not_empty_irq;
        send_queue.push(value);
    }

    pub fn receive<const ARM_BOOL: bool>(&mut self) -> u32 {
        let (cnt, receive_queue, recent, interrupt) = if ARM_BOOL {
            (
                &mut self.cnt9,
                &mut self.send_queue7,
                &mut self.recent9,
                &mut self.request_send_fifo_empty_irq7,
            )
        } else {
            (
                &mut self.cnt7,
                &mut self.send_queue9,
                &mut self.recent7,
                &mut self.request_send_fifo_empty_irq9,
            )
        };

        if !cnt.enabled {
            return *recent;
        }

        if receive_queue.is_empty() {
            cnt.error = true;
            return *recent;
        }

        let value = receive_queue.pop().unwrap();
        *interrupt = receive_queue.is_empty() && cnt.send_fifo_empty_irq;
        *recent = value;
        value
    }
}
