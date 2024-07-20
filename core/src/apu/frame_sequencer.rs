use super::clock_divider::ClockDivider;

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
}

impl FrameSequencer {
    pub fn new(source_clock_rate: usize) -> Self {
        Self {
            divider: ClockDivider::new(source_clock_rate, 512),
            step: 0,
        }
    }

    pub fn set_source_clock_rate(&mut self, source_clock_rate: usize) {
        self.divider.set_source_clock_rate(source_clock_rate);
    }

    pub fn step(&mut self, cycles: usize) -> Option<usize> {
        if !self.divider.step(cycles) {
            return None;
        }
        let current_step = self.step;
        self.step = (self.step + 1) % 8;
        Some(current_step)
    }
}

#[test]
fn test_frame_sequencer() {
    let mut seq = FrameSequencer::new(4194304);

    for i in 1..=(8192 * 10) {
        match i {
            8192 => assert_eq!(seq.step(1), Some(0)),
            16384 => assert_eq!(seq.step(1), Some(1)),
            24576 => assert_eq!(seq.step(1), Some(2)),
            32768 => assert_eq!(seq.step(1), Some(3)),
            40960 => assert_eq!(seq.step(1), Some(4)),
            49152 => assert_eq!(seq.step(1), Some(5)),
            57344 => assert_eq!(seq.step(1), Some(6)),
            65536 => assert_eq!(seq.step(1), Some(7)),
            73728 => assert_eq!(seq.step(1), Some(0)),
            81920 => assert_eq!(seq.step(1), Some(1)),
            _ => assert_eq!(seq.step(1), None),
        }
    }
}
