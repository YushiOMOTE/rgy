/// The frame sequencer generates low frequency clocks for the modulation units. It is clocked by a 512 Hz timer.
///
/// Step   Length Ctr  Vol Env     Sweep
/// ---------------------------------------
/// 0      Clock       -           -
/// 1      -           -           -
/// 2      Clock       -           Clock
/// 3      -           -           -
/// 4      Clock       -           -
/// 5      -           -           -
/// 6      Clock       -           Clock
/// 7      -           Clock       -
/// ---------------------------------------
/// Rate   256 Hz      64 Hz       128 Hz
///
#[derive(Debug, Clone)]
pub struct FrameSequencer {
    frame: Frame,
    resetting: usize,
    last_div: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Frame {
    pub step: usize,
    pub cycles: usize,
}

impl Frame {
    pub fn switched(&self) -> Option<usize> {
        if self.cycles == 0 {
            Some(self.step)
        } else {
            None
        }
    }
}

impl Frame {
    fn new() -> Self {
        Self { step: 7, cycles: 0 }
    }
}

impl FrameSequencer {
    pub fn new() -> Self {
        Self {
            frame: Frame::new(),
            resetting: 0,
            last_div: 0,
        }
    }

    pub fn reset_step(&mut self) {
        self.frame = Frame::new();

        // This is to prevent updaating step immediately to 1 after reset
        // when reset and div-apu happens in the same machine cycle,
        self.resetting = 4;
    }

    fn bit_down(&mut self, div: usize) -> bool {
        let bit4_old = bit4(self.last_div);
        let bit4_new = bit4(div);

        self.last_div = div;

        bit4_old && !bit4_new
    }

    pub fn step(&mut self, cycles: usize, div: usize) -> Frame {
        if self.resetting > 0 && div & 0x10 > 0 {
            // If reset with bit 4 set, skip the first frame
            self.update();
        }

        self.frame.cycles = self.frame.cycles.wrapping_add(cycles);

        if self.bit_down(div) && self.resetting == 0 {
            self.update();
        }

        self.resetting = self.resetting.saturating_sub(cycles);

        self.frame
    }

    fn update(&mut self) {
        let new_step = (self.frame.step + 1) % 8;
        self.frame.step = new_step;
        self.frame.cycles = 0;
    }
}

fn bit4(value: usize) -> bool {
    value & 0x10 != 0
}

#[test]
fn test_frame_sequencer_step() {
    let mut seq = FrameSequencer::new();

    assert_eq!(seq.step(1, 0x10).step, 7);
    assert_eq!(seq.step(1, 0x00).step, 0);
    assert_eq!(seq.step(1, 0x10).step, 0);
    assert_eq!(seq.step(1, 0x00).step, 1);
    assert_eq!(seq.step(1, 0x10).step, 1);
    assert_eq!(seq.step(1, 0x00).step, 2);
    assert_eq!(seq.step(1, 0x10).step, 2);
    assert_eq!(seq.step(1, 0x00).step, 3);
    assert_eq!(seq.step(1, 0x10).step, 3);
    assert_eq!(seq.step(1, 0x00).step, 4);
    assert_eq!(seq.step(1, 0x10).step, 4);
    assert_eq!(seq.step(1, 0x00).step, 5);
    assert_eq!(seq.step(1, 0x10).step, 5);
    assert_eq!(seq.step(1, 0x00).step, 6);
    assert_eq!(seq.step(1, 0x10).step, 6);
    assert_eq!(seq.step(1, 0x00).step, 7);
    assert_eq!(seq.step(1, 0x10).step, 7);
    assert_eq!(seq.step(1, 0x00).step, 0);
    assert_eq!(seq.step(1, 0x10).step, 0);
    assert_eq!(seq.step(1, 0x00).step, 1);
    assert_eq!(seq.step(1, 0x10).step, 1);
    assert_eq!(seq.step(1, 0x00).step, 2);
}

#[test]
fn test_frame_sequencer_cycles() {
    let mut seq = FrameSequencer::new();

    assert_eq!(seq.step(1, 0x10).cycles, 1);
    assert_eq!(seq.step(2, 0x10).cycles, 3);
    assert_eq!(seq.step(3, 0x10).cycles, 6);
    assert_eq!(seq.step(4, 0x10).cycles, 10);
    assert_eq!(seq.step(5, 0x10).cycles, 15);
    assert_eq!(seq.step(6, 0x00).step, 0);
    assert_eq!(seq.step(7, 0x10).cycles, 7);
    assert_eq!(seq.step(8, 0x10).cycles, 15);
    assert_eq!(seq.step(9, 0x10).cycles, 24);
    assert_eq!(seq.step(10, 0x10).cycles, 34);
    assert_eq!(seq.step(11, 0x10).cycles, 45);
    assert_eq!(seq.step(12, 0x00).step, 1);
    assert_eq!(seq.step(13, 0x10).cycles, 13);
    assert_eq!(seq.step(14, 0x10).cycles, 27);
    assert_eq!(seq.step(15, 0x10).cycles, 42);
    assert_eq!(seq.step(16, 0x10).cycles, 58);
    assert_eq!(seq.step(17, 0x10).cycles, 75);
}

#[test]
fn test_frame_sequencer_switch() {
    let mut seq = FrameSequencer::new();

    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x00).switched(), Some(0));
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x00).switched(), Some(1));
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x00).switched(), Some(2));
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x00).switched(), Some(3));
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x00).switched(), Some(4));
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x00).switched(), Some(5));
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x00).switched(), Some(6));
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x00).switched(), Some(7));
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x00).switched(), Some(0));
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x00).switched(), Some(1));
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x10).switched(), None);
    assert_eq!(seq.step(1, 0x00).switched(), Some(2));
}
