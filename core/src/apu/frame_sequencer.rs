use crate::clock::ClockDivider;

const FRAME_SEQUENCER_FREQ_HZ: usize = 512;

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
    divider: ClockDivider,
    step: usize,
    resetting: usize,
}

impl FrameSequencer {
    pub fn new() -> Self {
        Self {
            divider: ClockDivider::new(FRAME_SEQUENCER_FREQ_HZ),
            step: 0,
            resetting: 0,
        }
    }

    pub fn reset_step(&mut self) {
        self.step = 0;
        // This is to prevent updaating step immediately to 1 after reset
        // when reset and div-apu happens in the same machine cycle,
        self.resetting = 4;
        self.divider.reset();
    }

    pub fn step(&mut self, cycles: usize, div_apu: bool) -> Option<usize> {
        if div_apu && self.resetting == 0 {
            return Some(self.update());
        }
        self.resetting = self.resetting.saturating_sub(cycles);
        None
    }

    fn update(&mut self) -> usize {
        let current_step = self.step;
        self.step = (self.step + 1) % 8;
        current_step
    }
}

#[test]
fn test_frame_sequencer() {
    let mut seq = FrameSequencer::new();

    assert_eq!(seq.step(1, true), Some(0));
    assert_eq!(seq.step(1, true), Some(1));
    assert_eq!(seq.step(1, true), Some(2));
    assert_eq!(seq.step(1, true), Some(3));
    assert_eq!(seq.step(1, true), Some(4));
    assert_eq!(seq.step(1, true), Some(5));
    assert_eq!(seq.step(1, true), Some(6));
    assert_eq!(seq.step(1, true), Some(7));
    assert_eq!(seq.step(1, true), Some(0));
    assert_eq!(seq.step(1, true), Some(1));
    assert_eq!(seq.step(1, true), Some(2));
}
