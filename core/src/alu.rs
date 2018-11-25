fn carry(b: usize, p: usize, q: usize, c: usize) -> bool {
    let c = c as usize;
    let m = (1 << b) - 1;
    (p & m) + (q & m) + (c & m) > m
}

fn borrow(b: usize, p: usize, q: usize, c: usize) -> bool {
    let m = (1 << b) - 1;
    (p & m) < (q & m) + (c & m)
}

fn add(b: usize, p: usize, q: usize, c: bool, hb: usize, cb: usize) -> (usize, bool, bool, bool) {
    let c = c as usize;
    let m = (1 << b) - 1;
    let s = (p + q + c) & m;
    let h = carry(hb, p, q, c);
    let c = carry(cb, p, q, c);
    let z = s == 0;
    (s, h, c, z)
}

fn sub(b: usize, p: usize, q: usize, c: bool, hb: usize, cb: usize) -> (usize, bool, bool, bool) {
    let c = c as usize;
    let m = (1 << b) - 1;
    let s = (p.wrapping_sub(q).wrapping_sub(c)) & m;
    let h = borrow(hb, p, q, c);
    let c = borrow(cb, p, q, c);
    let z = s == 0;
    (s, h, c, z)
}

pub fn signed(v: u8) -> u16 {
    if v & 0x80 != 0 {
        0xff00 | v as u16
    } else {
        v as u16
    }
}

pub fn add8(p: u8, q: u8, c: bool) -> (u8, bool, bool, bool) {
    let (v, h, c, z) = add(8, p as usize, q as usize, c, 4, 8);
    (v as u8, h, c, z)
}

pub fn sub8(p: u8, q: u8, c: bool) -> (u8, bool, bool, bool) {
    let (v, h, c, z) = sub(8, p as usize, q as usize, c, 4, 8);
    (v as u8, h, c, z)
}

pub fn add16(p: u16, q: u16, c: bool) -> (u16, bool, bool, bool) {
    let (v, h, c, z) = add(16, p as usize, q as usize, c, 12, 16);
    (v as u16, h, c, z)
}

pub fn add16e(p: u16, q: u8, c: bool) -> (u16, bool, bool, bool) {
    let (v, h, c, z) = add(16, p as usize, signed(q) as usize, c, 4, 8);
    (v as u16, h, c, z)
}

#[test]
fn test_add8() {
    assert_eq!(add8(0x12, 0x22, false), (0x34, false, false, false));
    assert_eq!(add8(0x12, 0x22, true), (0x35, false, false, false));
    assert_eq!(add8(0x12, 0x2f, false), (0x41, true, false, false));
    assert_eq!(add8(0x12, 0x2f, true), (0x42, true, false, false));
    assert_eq!(add8(0x12, 0xf0, false), (0x02, false, true, false));
    assert_eq!(add8(0x12, 0xf0, true), (0x03, false, true, false));
    assert_eq!(add8(0x0a, 0xfa, false), (0x04, true, true, false));
    assert_eq!(add8(0x0a, 0xfa, true), (0x05, true, true, false));
    assert_eq!(add8(0x00, 0x00, false), (0x00, false, false, true));
    assert_eq!(add8(0x20, 0xe0, false), (0x00, false, true, true));
    assert_eq!(add8(0x08, 0xf8, false), (0x00, true, true, true));
    assert_eq!(add8(0x07, 0xf8, true), (0x00, true, true, true));
}

#[test]
fn test_sub8() {
    assert_eq!(sub8(0x12, 0x10, false), (0x02, false, false, false));
    assert_eq!(sub8(0x34, 0x22, true), (0x11, false, false, false));
    assert_eq!(sub8(0x32, 0x2f, false), (0x03, true, false, false));
    assert_eq!(sub8(0x32, 0x2e, true), (0x03, true, false, false));
    assert_eq!(sub8(0x12, 0xf0, false), (0x22, false, true, false));
    assert_eq!(sub8(0x12, 0xe0, true), (0x31, false, true, false));
    assert_eq!(sub8(0x0a, 0xef, false), (0x1b, true, true, false));
    assert_eq!(sub8(0x20, 0x5a, true), (0xc5, true, true, false));
    assert_eq!(sub8(0x12, 0x12, false), (0x00, false, false, true));
    assert_eq!(sub8(0x88, 0x87, true), (0x00, false, false, true));
}

#[test]
fn test_add16() {
    assert_eq!(add16(0x1200, 0x1000, false), (0x2200, false, false, false));
    assert_eq!(add16(0x1134, 0x1222, true), (0x2357, false, false, false));
    assert_eq!(add16(0xf231, 0x2a13, false), (0x1c44, false, true, false));
    assert_eq!(add16(0xf231, 0x2a13, true), (0x1c45, false, true, false));
    assert_eq!(add16(0xf631, 0x2a03, false), (0x2034, true, true, false));
    assert_eq!(add16(0xf631, 0x2a03, true), (0x2035, true, true, false));
}

#[test]
fn test_signed() {
    assert_eq!(signed(0x0a), 0x000a);
    assert_eq!(signed(0x8a), 0xff8a);
}
