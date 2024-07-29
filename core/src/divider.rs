use crate::clock::PrescaledTimer;

pub struct Divider {
    timer: PrescaledTimer,
}

impl Divider {
    pub fn new() -> Self {
        Self {
            timer: PrescaledTimer::builder()
                .enable()
                .frequency(16384)
                .interval(256)
                .build(),
        }
    }

    pub fn step(&mut self, cycles: usize) -> usize {
        self.timer.step(cycles);
        self.timer.counter()
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
