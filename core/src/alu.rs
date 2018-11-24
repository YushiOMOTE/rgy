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
