use volatile::Volatile;
use BusId::*;

pub const DEVICE_MCU: Device = Device { id: 0, bus_id: Bus2, addr: 0x4a };
// const DEVICE_CAM0: Device = Device { id: 1, bus_id: 1, addr: 0x7a };
// const DEVICE_CAM1: Device = Device { id: 2, bus_id: 1, addr: 0x78 };

type Result<T = (), E = ()> = core::result::Result<T, E>;

/// Should only be called once
pub unsafe fn init() {
    Bus::init(Bus1);
    Bus::init(Bus2);
    Bus::init(Bus3);
}

pub fn write_reg(device: Device, reg: u8, data: u8) {
    write_reg_buf(device, reg, &[data]);
}

pub fn write_reg_buf(device: Device, reg: u8, data: &[u8]) -> Result {
    let bus = unsafe { Bus::from_id(device.bus_id) };

    bus.start_transfer(device.addr, reg, Mode::Write)?;

    for (i, &byte) in data.iter().enumerate() {
        bus.set_data(byte);

        let mut control = Control::BUSY | Control::INTERRUPT;
        
        if i == data.len() - 1 {
            control |= Control::STOP;
        }

        bus.set_control(control);
        bus.wait_while_busy();

        if !bus.received_ack() {
            return Err(())
        }
    }

    Ok(())
}

#[repr(C)]
struct Bus {
    data: Volatile<u8>,
    control: Volatile<u8>,
    cntex: Volatile<u16>,
    signal_clock: Volatile<u16>,
}

impl Bus {
    fn start_transfer(&mut self, addr: u8, reg: u8, mode: Mode) -> Result {
        for _ in 0..8 {
            self.wait_while_busy();

            // select device
            {
                self.set_data(addr);
                self.set_control(Control::BUSY | Control::INTERRUPT | Control::START);

                if !self.received_ack() {
                    continue
                }
            }

            // seelect register
            {
                self.set_data(reg);
                self.set_control(Control::BUSY | Control::INTERRUPT);

                if !self.received_ack() {
                    continue
                }
            }

            // select read mode
            if mode == Mode::Read {
                self.set_data(addr | 1);
                self.set_control(Control::BUSY | Control::INTERRUPT | Control::START);

                if !self.received_ack() {
                    continue
                }
            }

            return Ok(());
        }

        Err(())
    }

    fn abort_transfer(&mut self) {
        self.set_control(
              Control::BUSY
            | Control::INTERRUPT
            | Control::ERROR
            | Control::STOP
        );
    }

    fn received_ack(&mut self) -> bool {
        self.wait_while_busy();
        let received_ack = self.control().contains(Control::ACK);

        if !received_ack {
            self.abort_transfer();
        }

        received_ack
    }

    fn data(&self) -> u8 {
        self.data.read()
    }

    fn set_data(&mut self, data: u8) {
        self.data.write(data);
    }

    fn control(&self) -> Control {
        let control = self.control.read();
        Control::from_bits_truncate(control)
    }

    fn set_control(&mut self, control: Control) {
        let control = control.bits();
        self.control.write(control);
    }
    
    fn wait_while_busy(&self) {
        while self.control().contains(Control::BUSY) {
            // Busy waiting for busy flag to clear
        }
    }
}

impl Bus {
    /// Unsafe because at most one &mut reference to a bus must exist at a time
    unsafe fn from_id(id: BusId) -> &'static mut Bus {
        (id.addr() as *mut Bus).as_mut().unwrap()
    }

    /// Unsafe because the bus should only be inited at most once
    unsafe fn init(id: BusId) {
        let bus = Self::from_id(id);
        bus.wait_while_busy();
        bus.cntex.write(2); // unknown meaning
        bus.signal_clock.write(1280);
    }
}

#[derive(Copy,Clone,PartialEq)]
enum BusId {
    Bus1,
    Bus2,
    Bus3,
}

impl BusId {
    fn addr(self) -> usize {
        match self {
            BusId::Bus1 => 0x10161000,
            BusId::Bus2 => 0x10144000,
            BusId::Bus3 => 0x10148000,
        }
    }
}

#[repr(u8)]
bitflags! {
    struct Control: u8 {
        const STOP      = 1 << 0;
        const START     = 1 << 1;
        const ERROR     = 1 << 2;
        const ACK       = 1 << 4;
        const READ      = 1 << 5;
        const INTERRUPT = 1 << 6;
        const BUSY      = 1 << 7;
    }
}

#[repr(u8)]
bitflags! {
    struct ControlEx: u8 {
        const UNKNOWN = 2;
    }
}

#[repr(u8)]
bitflags! {
    struct SCL: u8 {
        const UNKNOWN = 5;
    }
}

#[derive(Copy,Clone,PartialEq)]
pub struct Device {
    id: u8,
    bus_id: BusId,
    addr: u8,
}

#[derive(Copy,Clone,PartialEq)]
enum Mode {
    Read,
    Write,
}

macro_rules! assert_size {
    ($name:ident, $ty:ty, $size:expr) => {
        #[allow(warnings)]
        fn $name() {
            panic!();
            unsafe {
                let x = core::mem::uninitialized();
                core::mem::transmute::<$ty, [u8; $size]>(x);
            }
        }
    };
}

assert_size!(Bus_, Bus, 6);