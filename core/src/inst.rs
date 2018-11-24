use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::alu;

/// nop
fn op_0000(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    (4, 1)
}

/// ld bc,d16
fn op_0001(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get16(cpu.get_pc().wrapping_add(1));
    cpu.set_bc(v);

    (12, 3)
}

/// ld (bc),a
fn op_0002(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    mmu.set8(cpu.get_bc(), v);

    (8, 1)
}

/// inc bc
fn op_0003(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_bc().wrapping_add(1);
    cpu.set_bc(v);

    (8, 1)
}

/// inc b
fn op_0004(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

/// dec b
fn op_0005(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

/// ld b,d8
fn op_0006(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_pc().wrapping_add(1));
    cpu.set_b(v);

    (8, 2)
}

/// rlca
fn op_0007(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(false);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (4, 1)
}

/// ld (a16),sp
fn op_0008(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_sp();
    mmu.set16(mmu.get16(cpu.get_pc().wrapping_add(1)), v);

    (20, 3)
}

/// add hl,bc
fn op_0009(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_hl();
    let q = cpu.get_bc();
    let (v, h, c, z) = alu::add16(p, q, false);
    cpu.set_hl(v);

    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

/// ld a,(bc)
fn op_000a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_bc());
    cpu.set_a(v);

    (8, 1)
}

/// dec bc
fn op_000b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_bc().wrapping_sub(1);
    cpu.set_bc(v);

    (8, 1)
}

/// inc c
fn op_000c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

/// dec c
fn op_000d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

/// ld c,d8
fn op_000e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_pc().wrapping_add(1));
    cpu.set_c(v);

    (8, 2)
}

/// rrca
fn op_000f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 1 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(false);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (4, 1)
}

/// stop 0
fn op_0010(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.stop();

    (4, 2)
}

/// ld de,d16
fn op_0011(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get16(cpu.get_pc().wrapping_add(1));
    cpu.set_de(v);

    (12, 3)
}

/// ld (de),a
fn op_0012(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    mmu.set8(cpu.get_de(), v);

    (8, 1)
}

/// inc de
fn op_0013(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_de().wrapping_add(1);
    cpu.set_de(v);

    (8, 1)
}

/// inc d
fn op_0014(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

/// dec d
fn op_0015(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

/// ld d,d8
fn op_0016(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_pc().wrapping_add(1));
    cpu.set_d(v);

    (8, 2)
}

/// rla
fn op_0017(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    cpu.set_a(v);
    cpu.set_zf(false);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (4, 1)
}

/// jr r8
fn op_0018(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = mmu.get8(cpu.get_pc().wrapping_add(1));
    let pc = cpu.get_pc().wrapping_add(alu::signed(p)).wrapping_add(2);
    cpu.set_pc(pc);

    (12, 2)
}

/// add hl,de
fn op_0019(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_hl();
    let q = cpu.get_de();
    let (v, h, c, z) = alu::add16(p, q, false);
    cpu.set_hl(v);

    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

/// ld a,(de)
fn op_001a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_de());
    cpu.set_a(v);

    (8, 1)
}

/// dec de
fn op_001b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_de().wrapping_sub(1);
    cpu.set_de(v);

    (8, 1)
}

/// inc e
fn op_001c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

/// dec e
fn op_001d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

/// ld e,d8
fn op_001e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_pc().wrapping_add(1));
    cpu.set_e(v);

    (8, 2)
}

/// rra
fn op_001f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    cpu.set_a(v);
    cpu.set_zf(false);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (4, 1)
}

/// jr nz,r8
fn op_0020(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let flg = !cpu.get_zf();
    if flg {
        let p = mmu.get8(cpu.get_pc().wrapping_add(1));
        let pc = cpu.get_pc().wrapping_add(alu::signed(p)).wrapping_add(2);
        cpu.set_pc(pc);
        return (12, 2);
    }

    (8, 2)
}

/// ld hl,d16
fn op_0021(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get16(cpu.get_pc().wrapping_add(1));
    cpu.set_hl(v);

    (12, 3)
}

/// ldi (hl),a
fn op_0022(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    mmu.set8(cpu.get_hl(), v);

    cpu.set_hl(cpu.get_hl().wrapping_add(1));

    (8, 1)
}

/// inc hl
fn op_0023(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_hl().wrapping_add(1);
    cpu.set_hl(v);

    (8, 1)
}

/// inc h
fn op_0024(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

/// dec h
fn op_0025(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

/// ld h,d8
fn op_0026(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_pc().wrapping_add(1));
    cpu.set_h(v);

    (8, 2)
}

/// daa
fn op_0027(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let l = cpu.get_a() & 0xf;
    let h = cpu.get_a() >> 4;

    let lc = if l > 9 || cpu.get_hf() { 0x06 } else { 0x00 };
    let hc = if h > 9 || cpu.get_cf() { 0x60 } else { 0x00 };

    let v = cpu.get_a();
    let v = if cpu.get_nf() {
        v.wrapping_add(lc + hc)
    } else {
        v.wrapping_sub(lc + hc)
    };

    let z = v == 0;
    let c = hc > 0;

    cpu.set_a(v);
    cpu.set_zf(z);

    cpu.set_hf(false);
    cpu.set_cf(c);

    (4, 1)
}

/// jr z,r8
fn op_0028(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let flg = cpu.get_zf();
    if flg {
        let p = mmu.get8(cpu.get_pc().wrapping_add(1));
        let pc = cpu.get_pc().wrapping_add(alu::signed(p)).wrapping_add(2);
        cpu.set_pc(pc);
        return (12, 2);
    }

    (8, 2)
}

/// add hl,hl
fn op_0029(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_hl();
    let q = cpu.get_hl();
    let (v, h, c, z) = alu::add16(p, q, false);
    cpu.set_hl(v);

    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

/// ldi a,(hl)
fn op_002a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_a(v);

    cpu.set_hl(cpu.get_hl().wrapping_add(1));

    (8, 1)
}

/// dec hl
fn op_002b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_hl().wrapping_sub(1);
    cpu.set_hl(v);

    (8, 1)
}

/// inc l
fn op_002c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

/// dec l
fn op_002d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

/// ld l,d8
fn op_002e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_pc().wrapping_add(1));
    cpu.set_l(v);

    (8, 2)
}

/// cpl
fn op_002f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ 0xff);

    cpu.set_nf(true);
    cpu.set_hf(true);

    (4, 1)
}

/// jr nc,r8
fn op_0030(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let flg = !cpu.get_cf();
    if flg {
        let p = mmu.get8(cpu.get_pc().wrapping_add(1));
        let pc = cpu.get_pc().wrapping_add(alu::signed(p)).wrapping_add(2);
        cpu.set_pc(pc);
        return (12, 2);
    }

    (8, 2)
}

/// ld sp,d16
fn op_0031(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get16(cpu.get_pc().wrapping_add(1));
    cpu.set_sp(v);

    (12, 3)
}

/// ldd (hl),a
fn op_0032(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    mmu.set8(cpu.get_hl(), v);

    cpu.set_hl(cpu.get_hl().wrapping_sub(1));

    (8, 1)
}

/// inc sp
fn op_0033(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_sp().wrapping_add(1);
    cpu.set_sp(v);

    (8, 1)
}

/// inc (hl)
fn op_0034(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let (v, h, c, z) = alu::add8(v, 1, false);
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (12, 1)
}

/// dec (hl)
fn op_0035(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let (v, h, c, z) = alu::sub8(v, 1, false);
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (12, 1)
}

/// ld (hl),d8
fn op_0036(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_pc().wrapping_add(1));
    mmu.set8(cpu.get_hl(), v);

    (12, 2)
}

/// scf
fn op_0037(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_cf(true);

    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(true);

    (4, 1)
}

/// jr cf,r8
fn op_0038(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let flg = cpu.get_cf();
    if flg {
        let p = mmu.get8(cpu.get_pc().wrapping_add(1));
        let pc = cpu.get_pc().wrapping_add(alu::signed(p)).wrapping_add(2);
        cpu.set_pc(pc);
        return (12, 2);
    }

    (8, 2)
}

/// add hl,sp
fn op_0039(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_hl();
    let q = cpu.get_sp();
    let (v, h, c, z) = alu::add16(p, q, false);
    cpu.set_hl(v);

    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

/// ldd a,(hl)
fn op_003a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_a(v);

    cpu.set_hl(cpu.get_hl().wrapping_sub(1));

    (8, 1)
}

/// dec sp
fn op_003b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_sp().wrapping_sub(1);
    cpu.set_sp(v);

    (8, 1)
}

/// inc a
fn op_003c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

/// dec a
fn op_003d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

/// ld a,d8
fn op_003e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_pc().wrapping_add(1));
    cpu.set_a(v);

    (8, 2)
}

/// ccf
fn op_003f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let c = !cpu.get_cf();

    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (4, 1)
}

/// ld b,b
fn op_0040(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_b(v);

    (4, 1)
}

/// ld b,c
fn op_0041(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_b(v);

    (4, 1)
}

/// ld b,d
fn op_0042(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_b(v);

    (4, 1)
}

/// ld b,e
fn op_0043(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_b(v);

    (4, 1)
}

/// ld b,h
fn op_0044(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_b(v);

    (4, 1)
}

/// ld b,l
fn op_0045(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_b(v);

    (4, 1)
}

/// ld b,(hl)
fn op_0046(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_b(v);

    (8, 1)
}

/// ld b,a
fn op_0047(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_b(v);

    (4, 1)
}

/// ld c,b
fn op_0048(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_c(v);

    (4, 1)
}

/// ld c,c
fn op_0049(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_c(v);

    (4, 1)
}

/// ld c,d
fn op_004a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_c(v);

    (4, 1)
}

/// ld c,e
fn op_004b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_c(v);

    (4, 1)
}

/// ld c,h
fn op_004c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_c(v);

    (4, 1)
}

/// ld c,l
fn op_004d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_c(v);

    (4, 1)
}

/// ld c,(hl)
fn op_004e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_c(v);

    (8, 1)
}

/// ld c,a
fn op_004f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_c(v);

    (4, 1)
}

/// ld d,b
fn op_0050(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_d(v);

    (4, 1)
}

/// ld d,c
fn op_0051(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_d(v);

    (4, 1)
}

/// ld d,d
fn op_0052(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_d(v);

    (4, 1)
}

/// ld d,e
fn op_0053(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_d(v);

    (4, 1)
}

/// ld d,h
fn op_0054(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_d(v);

    (4, 1)
}

/// ld d,l
fn op_0055(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_d(v);

    (4, 1)
}

/// ld d,(hl)
fn op_0056(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_d(v);

    (8, 1)
}

/// ld d,a
fn op_0057(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_d(v);

    (4, 1)
}

/// ld e,b
fn op_0058(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_e(v);

    (4, 1)
}

/// ld e,c
fn op_0059(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_e(v);

    (4, 1)
}

/// ld e,d
fn op_005a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_e(v);

    (4, 1)
}

/// ld e,e
fn op_005b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_e(v);

    (4, 1)
}

/// ld e,h
fn op_005c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_e(v);

    (4, 1)
}

/// ld e,l
fn op_005d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_e(v);

    (4, 1)
}

/// ld e,(hl)
fn op_005e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_e(v);

    (8, 1)
}

/// ld e,a
fn op_005f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_e(v);

    (4, 1)
}

/// ld h,b
fn op_0060(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_h(v);

    (4, 1)
}

/// ld h,c
fn op_0061(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_h(v);

    (4, 1)
}

/// ld h,d
fn op_0062(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_h(v);

    (4, 1)
}

/// ld h,e
fn op_0063(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_h(v);

    (4, 1)
}

/// ld h,h
fn op_0064(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_h(v);

    (4, 1)
}

/// ld h,l
fn op_0065(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_h(v);

    (4, 1)
}

/// ld h,(hl)
fn op_0066(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_h(v);

    (8, 1)
}

/// ld h,a
fn op_0067(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_h(v);

    (4, 1)
}

/// ld l,b
fn op_0068(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_l(v);

    (4, 1)
}

/// ld l,c
fn op_0069(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_l(v);

    (4, 1)
}

/// ld l,d
fn op_006a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_l(v);

    (4, 1)
}

/// ld l,e
fn op_006b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_l(v);

    (4, 1)
}

/// ld l,h
fn op_006c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_l(v);

    (4, 1)
}

/// ld l,l
fn op_006d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_l(v);

    (4, 1)
}

/// ld l,(hl)
fn op_006e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_l(v);

    (8, 1)
}

/// ld l,a
fn op_006f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_l(v);

    (4, 1)
}

/// ld (hl),b
fn op_0070(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

/// ld (hl),c
fn op_0071(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

/// ld (hl),d
fn op_0072(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

/// ld (hl),e
fn op_0073(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

/// ld (hl),h
fn op_0074(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

/// ld (hl),l
fn op_0075(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

/// halt
fn op_0076(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.halt();

    (4, 1)
}

/// ld (hl),a
fn op_0077(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

/// ld a,b
fn op_0078(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_a(v);

    (4, 1)
}

/// ld a,c
fn op_0079(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_a(v);

    (4, 1)
}

/// ld a,d
fn op_007a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_a(v);

    (4, 1)
}

/// ld a,e
fn op_007b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_a(v);

    (4, 1)
}

/// ld a,h
fn op_007c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_a(v);

    (4, 1)
}

/// ld a,l
fn op_007d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_a(v);

    (4, 1)
}

/// ld a,(hl)
fn op_007e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_a(v);

    (8, 1)
}

/// ld a,a
fn op_007f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_a(v);

    (4, 1)
}

/// add a,b
fn op_0080(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_b();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// add a,c
fn op_0081(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_c();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// add a,d
fn op_0082(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_d();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// add a,e
fn op_0083(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_e();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// add a,h
fn op_0084(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_h();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// add a,l
fn op_0085(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_l();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// add a,(hl)
fn op_0086(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_hl());
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

/// add a,a
fn op_0087(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_a();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// adc a,b
fn op_0088(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_b();
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// adc a,c
fn op_0089(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_c();
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// adc a,d
fn op_008a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_d();
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// adc a,e
fn op_008b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_e();
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// adc a,h
fn op_008c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_h();
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// adc a,l
fn op_008d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_l();
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// adc a,(hl)
fn op_008e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_hl());
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

/// adc a,a
fn op_008f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_a();
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// sub b
fn op_0090(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_b();
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// sub c
fn op_0091(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_c();
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// sub d
fn op_0092(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_d();
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// sub e
fn op_0093(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_e();
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// sub h
fn op_0094(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_h();
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// sub l
fn op_0095(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_l();
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// sub (hl)
fn op_0096(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_hl());
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

/// sub a
fn op_0097(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_a();
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// sbc a,b
fn op_0098(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_b();
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// sbc a,c
fn op_0099(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_c();
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// sbc a,d
fn op_009a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_d();
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// sbc a,e
fn op_009b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_e();
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// sbc a,h
fn op_009c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_h();
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// sbc a,l
fn op_009d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_l();
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// sbc a,(hl)
fn op_009e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_hl());
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

/// sbc a,a
fn op_009f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_a();
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// and b
fn op_00a0(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_b());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

/// and c
fn op_00a1(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_c());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

/// and d
fn op_00a2(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_d());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

/// and e
fn op_00a3(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_e());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

/// and h
fn op_00a4(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_h());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

/// and l
fn op_00a5(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_l());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

/// and (hl)
fn op_00a6(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & mmu.get8(cpu.get_hl()));
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (8, 1)
}

/// and a
fn op_00a7(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_a());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

/// xor b
fn op_00a8(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ cpu.get_b());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// xor c
fn op_00a9(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ cpu.get_c());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// xor d
fn op_00aa(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ cpu.get_d());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// xor e
fn op_00ab(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ cpu.get_e());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// xor h
fn op_00ac(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ cpu.get_h());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// xor l
fn op_00ad(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ cpu.get_l());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// xor (hl)
fn op_00ae(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ mmu.get8(cpu.get_hl()));
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 1)
}

/// xor a
fn op_00af(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ cpu.get_a());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// or b
fn op_00b0(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | cpu.get_b());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// or c
fn op_00b1(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | cpu.get_c());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// or d
fn op_00b2(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | cpu.get_d());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// or e
fn op_00b3(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | cpu.get_e());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// or h
fn op_00b4(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | cpu.get_h());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// or l
fn op_00b5(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | cpu.get_l());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// or (hl)
fn op_00b6(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | mmu.get8(cpu.get_hl()));
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 1)
}

/// or a
fn op_00b7(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | cpu.get_a());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// cp b
fn op_00b8(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_b();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// cp c
fn op_00b9(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_c();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// cp d
fn op_00ba(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_d();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// cp e
fn op_00bb(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_e();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// cp h
fn op_00bc(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_h();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// cp l
fn op_00bd(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_l();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// cp (hl)
fn op_00be(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_hl());
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

/// cp a
fn op_00bf(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_a();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// ret nz
fn op_00c0(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let flg = !cpu.get_zf();
    if flg {
        cpu.set_pc(cpu.pop(mmu));
        return (20, 0);
    }

    (8, 1)
}

/// pop bc
fn op_00c1(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_bc(cpu.pop(mmu));

    (12, 1)
}

/// jp nz,a16
fn op_00c2(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    (12, 3)
}

/// jp a16
fn op_00c3(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    (16, 3)
}

/// call nz,a16
fn op_00c4(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let flg = !cpu.get_zf();
    if flg {
        cpu.push(mmu, cpu.get_pc().wrapping_add(3));
        cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(1)));
        return (24, 0);
    }

    (12, 3)
}

/// push bc
fn op_00c5(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_bc());

    (16, 1)
}

/// add a,d8
fn op_00c6(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(1));
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

/// rst 0x00
fn op_00c7(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_pc(0x00u16.wrapping_sub(1));

    (16, 1)
}

/// ret z
fn op_00c8(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let flg = cpu.get_zf();
    if flg {
        cpu.set_pc(cpu.pop(mmu));
        return (20, 0);
    }

    (8, 1)
}

/// ret
fn op_00c9(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_pc(cpu.pop(mmu).wrapping_sub(1));

    (16, 1)
}

/// jp z,a16
fn op_00ca(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    (12, 3)
}

/// prefix cb
fn op_00cb(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    (4, 1)
}

/// call z,a16
fn op_00cc(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let flg = cpu.get_zf();
    if flg {
        cpu.push(mmu, cpu.get_pc().wrapping_add(3));
        cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(1)));
        return (24, 0);
    }

    (12, 3)
}

/// call a16
fn op_00cd(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_pc().wrapping_add(3));
    cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(1)).wrapping_sub(3));

    (24, 3)
}

/// adc a,d8
fn op_00ce(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(1));
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

/// rst 0x08
fn op_00cf(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_pc(0x08u16.wrapping_sub(1));

    (16, 1)
}

/// ret nc
fn op_00d0(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let flg = !cpu.get_cf();
    if flg {
        cpu.set_pc(cpu.pop(mmu));
        return (20, 0);
    }

    (8, 1)
}

/// pop de
fn op_00d1(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_de(cpu.pop(mmu));

    (12, 1)
}

/// jp nc,a16
fn op_00d2(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    (12, 3)
}

/// call nc,a16
fn op_00d4(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let flg = !cpu.get_cf();
    if flg {
        cpu.push(mmu, cpu.get_pc().wrapping_add(3));
        cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(1)));
        return (24, 0);
    }

    (12, 3)
}

/// push de
fn op_00d5(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_de());

    (16, 1)
}

/// sub d8
fn op_00d6(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(1));
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

/// rst 0x10
fn op_00d7(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_pc(0x10u16.wrapping_sub(1));

    (16, 1)
}

/// ret cf
fn op_00d8(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let flg = cpu.get_cf();
    if flg {
        cpu.set_pc(cpu.pop(mmu));
        return (20, 0);
    }

    (8, 1)
}

/// reti
fn op_00d9(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_pc(cpu.pop(mmu).wrapping_sub(1));
    cpu.enable_interrupt_immediate();

    (16, 1)
}

/// jp cf,a16
fn op_00da(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    (12, 3)
}

/// call cf,a16
fn op_00dc(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let flg = cpu.get_cf();
    if flg {
        cpu.push(mmu, cpu.get_pc().wrapping_add(3));
        cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(1)));
        return (24, 0);
    }

    (12, 3)
}

/// sbc a,d8
fn op_00de(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(1));
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

/// rst 0x18
fn op_00df(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_pc(0x18u16.wrapping_sub(1));

    (16, 1)
}

/// ld (0xff00+a8),a
fn op_00e0(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    mmu.set8(0xff00 + mmu.get8(cpu.get_pc().wrapping_add(1)) as u16, v);

    (12, 2)
}

/// pop hl
fn op_00e1(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_hl(cpu.pop(mmu));

    (12, 1)
}

/// ld (0xff00+c),a
fn op_00e2(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    mmu.set8(0xff00 + cpu.get_c() as u16, v);

    (8, 1)
}

/// push hl
fn op_00e5(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_hl());

    (16, 1)
}

/// and d8
fn op_00e6(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & mmu.get8(cpu.get_pc().wrapping_add(1)));
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (8, 2)
}

/// rst 0x20
fn op_00e7(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_pc(0x20u16.wrapping_sub(1));

    (16, 1)
}

/// add sp,r8
fn op_00e8(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_sp();
    let q = mmu.get8(cpu.get_pc().wrapping_add(1));
    let (v, h, c, z) = alu::add16e(p, q, false);
    cpu.set_sp(v);
    cpu.set_zf(false);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (16, 2)
}

/// jp (hl)
fn op_00e9(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    (4, 1)
}

/// ld (a16),a
fn op_00ea(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    mmu.set8(mmu.get16(cpu.get_pc().wrapping_add(1)), v);

    (16, 3)
}

/// xor d8
fn op_00ee(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ mmu.get8(cpu.get_pc().wrapping_add(1)));
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// rst 0x28
fn op_00ef(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_pc(0x28u16.wrapping_sub(1));

    (16, 1)
}

/// ld a,(0xff00+a8)
fn op_00f0(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(0xff00 + mmu.get8(cpu.get_pc().wrapping_add(1)) as u16);
    cpu.set_a(v);

    (12, 2)
}

/// pop af
fn op_00f1(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_af(cpu.pop(mmu));

    (12, 1)
}

/// ld a,(0xff00+c)
fn op_00f2(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(0xff00 + cpu.get_c() as u16);
    cpu.set_a(v);

    (8, 1)
}

/// di
fn op_00f3(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.disable_interrupt();

    (4, 1)
}

/// push af
fn op_00f5(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_af());

    (16, 1)
}

/// or d8
fn op_00f6(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | mmu.get8(cpu.get_pc().wrapping_add(1)));
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// rst 0x30
fn op_00f7(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_pc(0x30u16.wrapping_sub(1));

    (16, 1)
}

/// ldhl sp,r8
fn op_00f8(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_sp();
    let q = mmu.get8(cpu.get_pc().wrapping_add(1));
    let (v, h, c, z) = alu::add16e(p, q, false);
    cpu.set_hl(v);
    cpu.set_zf(false);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (12, 2)
}

/// ld sp,hl
fn op_00f9(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_hl();
    cpu.set_sp(v);

    (8, 1)
}

/// ld a,(a16)
fn op_00fa(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(mmu.get16(cpu.get_pc().wrapping_add(1)));
    cpu.set_a(v);

    (16, 3)
}

/// ei
fn op_00fb(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.enable_interrupt();

    (4, 1)
}

/// cp d8
fn op_00fe(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(1));
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

/// rst 0x38
fn op_00ff(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    cpu.set_pc(0x38u16.wrapping_sub(1));

    (16, 1)
}

/// rlc b
fn op_cb00(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rlc c
fn op_cb01(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rlc d
fn op_cb02(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rlc e
fn op_cb03(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rlc h
fn op_cb04(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rlc l
fn op_cb05(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rlc (hl)
fn op_cb06(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (16, 2)
}

/// rlc a
fn op_cb07(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 0x80 != 0;
    let v = v.rotate_left(1);
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rrc b
fn op_cb08(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let c = v & 1 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rrc c
fn op_cb09(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let c = v & 1 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rrc d
fn op_cb0a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let c = v & 1 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rrc e
fn op_cb0b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let c = v & 1 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rrc h
fn op_cb0c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let c = v & 1 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rrc l
fn op_cb0d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let c = v & 1 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rrc (hl)
fn op_cb0e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let c = v & 1 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (16, 2)
}

/// rrc a
fn op_cb0f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 1 != 0;
    let v = v.rotate_right(1);
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rl b
fn op_cb10(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    let z = v == 0;
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rl c
fn op_cb11(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    let z = v == 0;
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rl d
fn op_cb12(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    let z = v == 0;
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rl e
fn op_cb13(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    let z = v == 0;
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rl h
fn op_cb14(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    let z = v == 0;
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rl l
fn op_cb15(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    let z = v == 0;
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rl (hl)
fn op_cb16(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    let z = v == 0;
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (16, 2)
}

/// rl a
fn op_cb17(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let v = v | if cpu.get_cf() { 1 } else { 0 };
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rr b
fn op_cb18(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    let z = v == 0;
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rr c
fn op_cb19(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    let z = v == 0;
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rr d
fn op_cb1a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    let z = v == 0;
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rr e
fn op_cb1b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    let z = v == 0;
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rr h
fn op_cb1c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    let z = v == 0;
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rr l
fn op_cb1d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    let z = v == 0;
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// rr (hl)
fn op_cb1e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    let z = v == 0;
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (16, 2)
}

/// rr a
fn op_cb1f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let v = v | if cpu.get_cf() { 0x80 } else { 0 };
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// sla b
fn op_cb20(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let z = v == 0;
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// sla c
fn op_cb21(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let z = v == 0;
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// sla d
fn op_cb22(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let z = v == 0;
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// sla e
fn op_cb23(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let z = v == 0;
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// sla h
fn op_cb24(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let z = v == 0;
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// sla l
fn op_cb25(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let z = v == 0;
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// sla (hl)
fn op_cb26(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let z = v == 0;
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (16, 2)
}

/// sla a
fn op_cb27(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 0x80 != 0;
    let v = v.wrapping_shl(1);
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// sra b
fn op_cb28(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let c = v & 1 != 0;
    let msb = v & 0x80;
    let v = v.wrapping_shr(1);
    let v = v | msb;
    let z = v == 0;
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// sra c
fn op_cb29(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let c = v & 1 != 0;
    let msb = v & 0x80;
    let v = v.wrapping_shr(1);
    let v = v | msb;
    let z = v == 0;
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// sra d
fn op_cb2a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let c = v & 1 != 0;
    let msb = v & 0x80;
    let v = v.wrapping_shr(1);
    let v = v | msb;
    let z = v == 0;
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// sra e
fn op_cb2b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let c = v & 1 != 0;
    let msb = v & 0x80;
    let v = v.wrapping_shr(1);
    let v = v | msb;
    let z = v == 0;
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// sra h
fn op_cb2c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let c = v & 1 != 0;
    let msb = v & 0x80;
    let v = v.wrapping_shr(1);
    let v = v | msb;
    let z = v == 0;
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// sra l
fn op_cb2d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let c = v & 1 != 0;
    let msb = v & 0x80;
    let v = v.wrapping_shr(1);
    let v = v | msb;
    let z = v == 0;
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// sra (hl)
fn op_cb2e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let c = v & 1 != 0;
    let msb = v & 0x80;
    let v = v.wrapping_shr(1);
    let v = v | msb;
    let z = v == 0;
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (16, 2)
}

/// sra a
fn op_cb2f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 1 != 0;
    let msb = v & 0x80;
    let v = v.wrapping_shr(1);
    let v = v | msb;
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// swap b
fn op_cb30(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let v = v.rotate_left(4);
    cpu.set_b(v);
    let z = v == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// swap c
fn op_cb31(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let v = v.rotate_left(4);
    cpu.set_c(v);
    let z = v == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// swap d
fn op_cb32(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let v = v.rotate_left(4);
    cpu.set_d(v);
    let z = v == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// swap e
fn op_cb33(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let v = v.rotate_left(4);
    cpu.set_e(v);
    let z = v == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// swap h
fn op_cb34(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let v = v.rotate_left(4);
    cpu.set_h(v);
    let z = v == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// swap l
fn op_cb35(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let v = v.rotate_left(4);
    cpu.set_l(v);
    let z = v == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// swap (hl)
fn op_cb36(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let v = v.rotate_left(4);
    mmu.set8(cpu.get_hl(), v);
    let z = v == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (16, 2)
}

/// swap a
fn op_cb37(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let v = v.rotate_left(4);
    cpu.set_a(v);
    let z = v == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// srl b
fn op_cb38(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let z = v == 0;
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// srl c
fn op_cb39(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let z = v == 0;
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// srl d
fn op_cb3a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let z = v == 0;
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// srl e
fn op_cb3b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let z = v == 0;
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// srl h
fn op_cb3c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let z = v == 0;
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// srl l
fn op_cb3d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let z = v == 0;
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// srl (hl)
fn op_cb3e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let z = v == 0;
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (16, 2)
}

/// srl a
fn op_cb3f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let c = v & 1 != 0;
    let v = v.wrapping_shr(1);
    let z = v == 0;
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (8, 2)
}

/// bit 0,b
fn op_cb40(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 0,c
fn op_cb41(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 0,d
fn op_cb42(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 0,e
fn op_cb43(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 0,h
fn op_cb44(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 0,l
fn op_cb45(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 0,(hl)
fn op_cb46(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (16, 2)
}

/// bit 0,a
fn op_cb47(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 1,b
fn op_cb48(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 1,c
fn op_cb49(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 1,d
fn op_cb4a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 1,e
fn op_cb4b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 1,h
fn op_cb4c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 1,l
fn op_cb4d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 1,(hl)
fn op_cb4e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (16, 2)
}

/// bit 1,a
fn op_cb4f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 2,b
fn op_cb50(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 2,c
fn op_cb51(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 2,d
fn op_cb52(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 2,e
fn op_cb53(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 2,h
fn op_cb54(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 2,l
fn op_cb55(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 2,(hl)
fn op_cb56(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (16, 2)
}

/// bit 2,a
fn op_cb57(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 3,b
fn op_cb58(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 3,c
fn op_cb59(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 3,d
fn op_cb5a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 3,e
fn op_cb5b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 3,h
fn op_cb5c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 3,l
fn op_cb5d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 3,(hl)
fn op_cb5e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (16, 2)
}

/// bit 3,a
fn op_cb5f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 4,b
fn op_cb60(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 4,c
fn op_cb61(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 4,d
fn op_cb62(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 4,e
fn op_cb63(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 4,h
fn op_cb64(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 4,l
fn op_cb65(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 4,(hl)
fn op_cb66(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (16, 2)
}

/// bit 4,a
fn op_cb67(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 5,b
fn op_cb68(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 5,c
fn op_cb69(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 5,d
fn op_cb6a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 5,e
fn op_cb6b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 5,h
fn op_cb6c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 5,l
fn op_cb6d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 5,(hl)
fn op_cb6e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (16, 2)
}

/// bit 5,a
fn op_cb6f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 6,b
fn op_cb70(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 6,c
fn op_cb71(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 6,d
fn op_cb72(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 6,e
fn op_cb73(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 6,h
fn op_cb74(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 6,l
fn op_cb75(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 6,(hl)
fn op_cb76(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (16, 2)
}

/// bit 6,a
fn op_cb77(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 7,b
fn op_cb78(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 7,c
fn op_cb79(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 7,d
fn op_cb7a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 7,e
fn op_cb7b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 7,h
fn op_cb7c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 7,l
fn op_cb7d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 7,(hl)
fn op_cb7e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (16, 2)
}

/// bit 7,a
fn op_cb7f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// res 0,b
fn op_cb80(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 0,c
fn op_cb81(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 0,d
fn op_cb82(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 0,e
fn op_cb83(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 0,h
fn op_cb84(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 0,l
fn op_cb85(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 0,(hl)
fn op_cb86(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 0,a
fn op_cb87(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 1,b
fn op_cb88(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 1,c
fn op_cb89(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 1,d
fn op_cb8a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 1,e
fn op_cb8b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 1,h
fn op_cb8c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 1,l
fn op_cb8d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 1,(hl)
fn op_cb8e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 1,a
fn op_cb8f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 2,b
fn op_cb90(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 2,c
fn op_cb91(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 2,d
fn op_cb92(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 2,e
fn op_cb93(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 2,h
fn op_cb94(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 2,l
fn op_cb95(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 2,(hl)
fn op_cb96(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 2,a
fn op_cb97(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 3,b
fn op_cb98(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 3,c
fn op_cb99(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 3,d
fn op_cb9a(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 3,e
fn op_cb9b(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 3,h
fn op_cb9c(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 3,l
fn op_cb9d(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 3,(hl)
fn op_cb9e(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 3,a
fn op_cb9f(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 4,b
fn op_cba0(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 4,c
fn op_cba1(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 4,d
fn op_cba2(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 4,e
fn op_cba3(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 4,h
fn op_cba4(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 4,l
fn op_cba5(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 4,(hl)
fn op_cba6(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 4,a
fn op_cba7(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 5,b
fn op_cba8(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 5,c
fn op_cba9(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 5,d
fn op_cbaa(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 5,e
fn op_cbab(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 5,h
fn op_cbac(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 5,l
fn op_cbad(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 5,(hl)
fn op_cbae(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 5,a
fn op_cbaf(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 6,b
fn op_cbb0(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 6,c
fn op_cbb1(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 6,d
fn op_cbb2(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 6,e
fn op_cbb3(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 6,h
fn op_cbb4(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 6,l
fn op_cbb5(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 6,(hl)
fn op_cbb6(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 6,a
fn op_cbb7(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 7,b
fn op_cbb8(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 7,c
fn op_cbb9(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 7,d
fn op_cbba(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 7,e
fn op_cbbb(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 7,h
fn op_cbbc(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 7,l
fn op_cbbd(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 7,(hl)
fn op_cbbe(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 7,a
fn op_cbbf(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// set 0,b
fn op_cbc0(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 0,c
fn op_cbc1(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 0,d
fn op_cbc2(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 0,e
fn op_cbc3(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 0,h
fn op_cbc4(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 0,l
fn op_cbc5(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 0,(hl)
fn op_cbc6(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 0,a
fn op_cbc7(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 1,b
fn op_cbc8(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 1,c
fn op_cbc9(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 1,d
fn op_cbca(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 1,e
fn op_cbcb(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 1,h
fn op_cbcc(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 1,l
fn op_cbcd(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 1,(hl)
fn op_cbce(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 1,a
fn op_cbcf(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 2,b
fn op_cbd0(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 2,c
fn op_cbd1(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 2,d
fn op_cbd2(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 2,e
fn op_cbd3(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 2,h
fn op_cbd4(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 2,l
fn op_cbd5(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 2,(hl)
fn op_cbd6(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 2,a
fn op_cbd7(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 3,b
fn op_cbd8(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 3,c
fn op_cbd9(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 3,d
fn op_cbda(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 3,e
fn op_cbdb(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 3,h
fn op_cbdc(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 3,l
fn op_cbdd(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 3,(hl)
fn op_cbde(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 3,a
fn op_cbdf(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 4,b
fn op_cbe0(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 4,c
fn op_cbe1(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 4,d
fn op_cbe2(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 4,e
fn op_cbe3(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 4,h
fn op_cbe4(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 4,l
fn op_cbe5(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 4,(hl)
fn op_cbe6(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 4,a
fn op_cbe7(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 5,b
fn op_cbe8(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 5,c
fn op_cbe9(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 5,d
fn op_cbea(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 5,e
fn op_cbeb(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 5,h
fn op_cbec(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 5,l
fn op_cbed(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 5,(hl)
fn op_cbee(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 5,a
fn op_cbef(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 6,b
fn op_cbf0(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 6,c
fn op_cbf1(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 6,d
fn op_cbf2(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 6,e
fn op_cbf3(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 6,h
fn op_cbf4(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 6,l
fn op_cbf5(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 6,(hl)
fn op_cbf6(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 6,a
fn op_cbf7(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 7,b
fn op_cbf8(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 7,c
fn op_cbf9(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 7,d
fn op_cbfa(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 7,e
fn op_cbfb(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 7,h
fn op_cbfc(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 7,l
fn op_cbfd(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 7,(hl)
fn op_cbfe(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 7,a
fn op_cbff(cpu: &Cpu, mmu: &Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}
