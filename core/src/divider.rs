use crate::clock::PrescaledTimer;

pub struct Divider {
    timer: PrescaledTimer,
    last_counter: usize,
}

impl Divider {
    pub fn new() -> Self {
        Self {
            timer: PrescaledTimer::builder()
                .enable()
                .frequency(16384)
                .interval(256)
                .build(),
            last_counter: 0,
        }
    }

    pub fn step(&mut self, cycles: usize) -> bool {
        self.timer.step(cycles);

        self.check_div_apu()
    }

    fn check_div_apu(&mut self) -> bool {
        let bit4_old = self.last_counter & 0x10 > 0;
        let bit4_new = self.timer.counter() & 0x10 > 0;

        self.last_counter = self.timer.counter();

        bit4_old && !bit4_new
    }

    // TODO: To be used for STOP emulation where DIV doesn't ticks
    #[allow(dead_code)]
    fn enable(&mut self) {
        self.timer.enable();
    }

    // TODO: To be used for STOP emulation where DIV doesn't ticks
    #[allow(dead_code)]
    fn diable(&mut self) {
        self.timer.disable();
    }

    pub fn on_read(&self) -> u8 {
        self.timer.counter() as u8
    }

    pub fn on_write(&mut self, _value: u8) {
        self.timer.set_counter(0);
    }
}
