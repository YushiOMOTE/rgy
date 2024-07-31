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
    pub last: Option<usize>,
    pub next: Option<usize>,
    pub cycles: usize,
}

impl Frame {
    fn new() -> Self {
        Self {
            last: None,
            next: Some(0),
            cycles: 0,
        }
    }

    fn partial() -> Self {
        Self {
            last: None,
            next: None,
            cycles: 0,
        }
    }

    pub fn switched(&self) -> Option<usize> {
        if self.cycles == 0 {
            self.last
        } else {
            None
        }
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
            self.frame = Frame::partial();
        }

        self.frame.cycles = self.frame.cycles.wrapping_add(cycles);

        if self.bit_down(div) && self.resetting == 0 {
            self.update();
        }

        self.resetting = self.resetting.saturating_sub(cycles);

        self.frame
    }

    fn update(&mut self) {
        self.frame.last = self.frame.next;
        self.frame.next = self.frame.next.map(|index| (index + 1) % 8).or(Some(0));
        self.frame.cycles = 0;
    }
}

fn bit4(value: usize) -> bool {
    value & 0x10 != 0
}

#[test]
fn test_frame_sequencer() {
    let mut seq = FrameSequencer::new();

    let f0 = seq.step(1, 0x10);

    assert_eq!(f0.last, None);
    assert_eq!(f0.next, Some(0));
    assert_eq!(f0.cycles, 1);
    assert_eq!(f0.switched(), None);

    let f1 = seq.step(2, 0x10);

    assert_eq!(f1.last, None);
    assert_eq!(f1.next, Some(0));
    assert_eq!(f1.cycles, 3);
    assert_eq!(f1.switched(), None);

    let f2 = seq.step(3, 0x10);

    assert_eq!(f2.last, None);
    assert_eq!(f2.next, Some(0));
    assert_eq!(f2.cycles, 6);
    assert_eq!(f2.switched(), None);

    let f3 = seq.step(4, 0x00);

    assert_eq!(f3.last, Some(0));
    assert_eq!(f3.next, Some(1));
    assert_eq!(f3.cycles, 0);
    assert_eq!(f3.switched(), Some(0));

    let f4 = seq.step(5, 0x10);

    assert_eq!(f4.last, Some(0));
    assert_eq!(f4.next, Some(1));
    assert_eq!(f4.cycles, 5);
    assert_eq!(f4.switched(), None);
}

#[test]
fn test_frame_sequencer_loop() {
    let mut seq = FrameSequencer::new();

    assert_eq!(seq.step(1, 0x10).next, Some(0));
    assert_eq!(seq.step(1, 0x00).next, Some(1));
    assert_eq!(seq.step(1, 0x10).next, Some(1));
    assert_eq!(seq.step(1, 0x00).next, Some(2));
    assert_eq!(seq.step(1, 0x10).next, Some(2));
    assert_eq!(seq.step(1, 0x00).next, Some(3));
    assert_eq!(seq.step(1, 0x10).next, Some(3));
    assert_eq!(seq.step(1, 0x00).next, Some(4));
    assert_eq!(seq.step(1, 0x10).next, Some(4));
    assert_eq!(seq.step(1, 0x00).next, Some(5));
    assert_eq!(seq.step(1, 0x10).next, Some(5));
    assert_eq!(seq.step(1, 0x00).next, Some(6));
    assert_eq!(seq.step(1, 0x10).next, Some(6));
    assert_eq!(seq.step(1, 0x00).next, Some(7));
    assert_eq!(seq.step(1, 0x10).next, Some(7));
    assert_eq!(seq.step(1, 0x00).next, Some(0));
    assert_eq!(seq.step(1, 0x10).next, Some(0));
    assert_eq!(seq.step(1, 0x00).next, Some(1));
    assert_eq!(seq.step(1, 0x10).next, Some(1));
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

#[test]
fn test_frame_sequencer_reset_div10() {
    let mut seq = FrameSequencer::new();

    seq.reset_step();

    let f0 = seq.step(4, 0x10);

    assert_eq!(f0.last, None);
    assert_eq!(f0.next, None);
    assert_eq!(f0.cycles, 4);
    assert_eq!(f0.switched(), None);

    let f1 = seq.step(4, 0x10);

    assert_eq!(f1.last, None);
    assert_eq!(f1.next, None);
    assert_eq!(f1.cycles, 8);
    assert_eq!(f1.switched(), None);

    let f2 = seq.step(4, 0x00);

    assert_eq!(f2.last, None);
    assert_eq!(f2.next, Some(0));
    assert_eq!(f2.cycles, 0);
    assert_eq!(f2.switched(), None); // First flip is ignored
}

#[test]
fn test_frame_sequencer_reset_div00() {
    let mut seq = FrameSequencer::new();

    seq.reset_step();

    let f0 = seq.step(4, 0x00);

    assert_eq!(f0.last, None);
    assert_eq!(f0.next, Some(0));
    assert_eq!(f0.cycles, 4);
    assert_eq!(f0.switched(), None);

    let f1 = seq.step(4, 0x10);

    assert_eq!(f1.last, None);
    assert_eq!(f1.next, Some(0));
    assert_eq!(f1.cycles, 8);
    assert_eq!(f1.switched(), None);

    let f2 = seq.step(4, 0x00);

    assert_eq!(f2.last, Some(0));
    assert_eq!(f2.next, Some(1));
    assert_eq!(f2.cycles, 0);
    assert_eq!(f2.switched(), Some(0));
}
