use crate::alu;
use crate::cpu::Cpu;
use crate::mmu::Mmu;
use lazy_static::lazy_static;
use log::*;
use std::collections::HashMap;

lazy_static! {
    static ref MNEMONICS: HashMap<u16, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0x0000, "nop ");
        m.insert(0x0001, "ld bc,d16");
        m.insert(0x0002, "ld (bc),a");
        m.insert(0x0003, "inc bc");
        m.insert(0x0004, "inc b");
        m.insert(0x0005, "dec b");
        m.insert(0x0006, "ld b,d8");
        m.insert(0x0007, "rlca ");
        m.insert(0x0008, "ld (a16),sp");
        m.insert(0x0009, "add hl,bc");
        m.insert(0x000a, "ld a,(bc)");
        m.insert(0x000b, "dec bc");
        m.insert(0x000c, "inc c");
        m.insert(0x000d, "dec c");
        m.insert(0x000e, "ld c,d8");
        m.insert(0x000f, "rrca ");
        m.insert(0x0010, "stop 0");
        m.insert(0x0011, "ld de,d16");
        m.insert(0x0012, "ld (de),a");
        m.insert(0x0013, "inc de");
        m.insert(0x0014, "inc d");
        m.insert(0x0015, "dec d");
        m.insert(0x0016, "ld d,d8");
        m.insert(0x0017, "rla ");
        m.insert(0x0018, "jr r8");
        m.insert(0x0019, "add hl,de");
        m.insert(0x001a, "ld a,(de)");
        m.insert(0x001b, "dec de");
        m.insert(0x001c, "inc e");
        m.insert(0x001d, "dec e");
        m.insert(0x001e, "ld e,d8");
        m.insert(0x001f, "rra ");
        m.insert(0x0020, "jr nz,r8");
        m.insert(0x0021, "ld hl,d16");
        m.insert(0x0022, "ldi (hl),a");
        m.insert(0x0023, "inc hl");
        m.insert(0x0024, "inc h");
        m.insert(0x0025, "dec h");
        m.insert(0x0026, "ld h,d8");
        m.insert(0x0027, "daa ");
        m.insert(0x0028, "jr z,r8");
        m.insert(0x0029, "add hl,hl");
        m.insert(0x002a, "ldi a,(hl)");
        m.insert(0x002b, "dec hl");
        m.insert(0x002c, "inc l");
        m.insert(0x002d, "dec l");
        m.insert(0x002e, "ld l,d8");
        m.insert(0x002f, "cpl ");
        m.insert(0x0030, "jr nc,r8");
        m.insert(0x0031, "ld sp,d16");
        m.insert(0x0032, "ldd (hl),a");
        m.insert(0x0033, "inc sp");
        m.insert(0x0034, "inc (hl)");
        m.insert(0x0035, "dec (hl)");
        m.insert(0x0036, "ld (hl),d8");
        m.insert(0x0037, "scf ");
        m.insert(0x0038, "jr cf,r8");
        m.insert(0x0039, "add hl,sp");
        m.insert(0x003a, "ldd a,(hl)");
        m.insert(0x003b, "dec sp");
        m.insert(0x003c, "inc a");
        m.insert(0x003d, "dec a");
        m.insert(0x003e, "ld a,d8");
        m.insert(0x003f, "ccf ");
        m.insert(0x0040, "ld b,b");
        m.insert(0x0041, "ld b,c");
        m.insert(0x0042, "ld b,d");
        m.insert(0x0043, "ld b,e");
        m.insert(0x0044, "ld b,h");
        m.insert(0x0045, "ld b,l");
        m.insert(0x0046, "ld b,(hl)");
        m.insert(0x0047, "ld b,a");
        m.insert(0x0048, "ld c,b");
        m.insert(0x0049, "ld c,c");
        m.insert(0x004a, "ld c,d");
        m.insert(0x004b, "ld c,e");
        m.insert(0x004c, "ld c,h");
        m.insert(0x004d, "ld c,l");
        m.insert(0x004e, "ld c,(hl)");
        m.insert(0x004f, "ld c,a");
        m.insert(0x0050, "ld d,b");
        m.insert(0x0051, "ld d,c");
        m.insert(0x0052, "ld d,d");
        m.insert(0x0053, "ld d,e");
        m.insert(0x0054, "ld d,h");
        m.insert(0x0055, "ld d,l");
        m.insert(0x0056, "ld d,(hl)");
        m.insert(0x0057, "ld d,a");
        m.insert(0x0058, "ld e,b");
        m.insert(0x0059, "ld e,c");
        m.insert(0x005a, "ld e,d");
        m.insert(0x005b, "ld e,e");
        m.insert(0x005c, "ld e,h");
        m.insert(0x005d, "ld e,l");
        m.insert(0x005e, "ld e,(hl)");
        m.insert(0x005f, "ld e,a");
        m.insert(0x0060, "ld h,b");
        m.insert(0x0061, "ld h,c");
        m.insert(0x0062, "ld h,d");
        m.insert(0x0063, "ld h,e");
        m.insert(0x0064, "ld h,h");
        m.insert(0x0065, "ld h,l");
        m.insert(0x0066, "ld h,(hl)");
        m.insert(0x0067, "ld h,a");
        m.insert(0x0068, "ld l,b");
        m.insert(0x0069, "ld l,c");
        m.insert(0x006a, "ld l,d");
        m.insert(0x006b, "ld l,e");
        m.insert(0x006c, "ld l,h");
        m.insert(0x006d, "ld l,l");
        m.insert(0x006e, "ld l,(hl)");
        m.insert(0x006f, "ld l,a");
        m.insert(0x0070, "ld (hl),b");
        m.insert(0x0071, "ld (hl),c");
        m.insert(0x0072, "ld (hl),d");
        m.insert(0x0073, "ld (hl),e");
        m.insert(0x0074, "ld (hl),h");
        m.insert(0x0075, "ld (hl),l");
        m.insert(0x0076, "halt ");
        m.insert(0x0077, "ld (hl),a");
        m.insert(0x0078, "ld a,b");
        m.insert(0x0079, "ld a,c");
        m.insert(0x007a, "ld a,d");
        m.insert(0x007b, "ld a,e");
        m.insert(0x007c, "ld a,h");
        m.insert(0x007d, "ld a,l");
        m.insert(0x007e, "ld a,(hl)");
        m.insert(0x007f, "ld a,a");
        m.insert(0x0080, "add a,b");
        m.insert(0x0081, "add a,c");
        m.insert(0x0082, "add a,d");
        m.insert(0x0083, "add a,e");
        m.insert(0x0084, "add a,h");
        m.insert(0x0085, "add a,l");
        m.insert(0x0086, "add a,(hl)");
        m.insert(0x0087, "add a,a");
        m.insert(0x0088, "adc a,b");
        m.insert(0x0089, "adc a,c");
        m.insert(0x008a, "adc a,d");
        m.insert(0x008b, "adc a,e");
        m.insert(0x008c, "adc a,h");
        m.insert(0x008d, "adc a,l");
        m.insert(0x008e, "adc a,(hl)");
        m.insert(0x008f, "adc a,a");
        m.insert(0x0090, "sub b");
        m.insert(0x0091, "sub c");
        m.insert(0x0092, "sub d");
        m.insert(0x0093, "sub e");
        m.insert(0x0094, "sub h");
        m.insert(0x0095, "sub l");
        m.insert(0x0096, "sub (hl)");
        m.insert(0x0097, "sub a");
        m.insert(0x0098, "sbc a,b");
        m.insert(0x0099, "sbc a,c");
        m.insert(0x009a, "sbc a,d");
        m.insert(0x009b, "sbc a,e");
        m.insert(0x009c, "sbc a,h");
        m.insert(0x009d, "sbc a,l");
        m.insert(0x009e, "sbc a,(hl)");
        m.insert(0x009f, "sbc a,a");
        m.insert(0x00a0, "and b");
        m.insert(0x00a1, "and c");
        m.insert(0x00a2, "and d");
        m.insert(0x00a3, "and e");
        m.insert(0x00a4, "and h");
        m.insert(0x00a5, "and l");
        m.insert(0x00a6, "and (hl)");
        m.insert(0x00a7, "and a");
        m.insert(0x00a8, "xor b");
        m.insert(0x00a9, "xor c");
        m.insert(0x00aa, "xor d");
        m.insert(0x00ab, "xor e");
        m.insert(0x00ac, "xor h");
        m.insert(0x00ad, "xor l");
        m.insert(0x00ae, "xor (hl)");
        m.insert(0x00af, "xor a");
        m.insert(0x00b0, "or b");
        m.insert(0x00b1, "or c");
        m.insert(0x00b2, "or d");
        m.insert(0x00b3, "or e");
        m.insert(0x00b4, "or h");
        m.insert(0x00b5, "or l");
        m.insert(0x00b6, "or (hl)");
        m.insert(0x00b7, "or a");
        m.insert(0x00b8, "cp b");
        m.insert(0x00b9, "cp c");
        m.insert(0x00ba, "cp d");
        m.insert(0x00bb, "cp e");
        m.insert(0x00bc, "cp h");
        m.insert(0x00bd, "cp l");
        m.insert(0x00be, "cp (hl)");
        m.insert(0x00bf, "cp a");
        m.insert(0x00c0, "ret nz");
        m.insert(0x00c1, "pop bc");
        m.insert(0x00c2, "jp nz,a16");
        m.insert(0x00c3, "jp a16");
        m.insert(0x00c4, "call nz,a16");
        m.insert(0x00c5, "push bc");
        m.insert(0x00c6, "add a,d8");
        m.insert(0x00c7, "rst 0x00");
        m.insert(0x00c8, "ret z");
        m.insert(0x00c9, "ret ");
        m.insert(0x00ca, "jp z,a16");
        m.insert(0x00cb, "prefix cb");
        m.insert(0x00cc, "call z,a16");
        m.insert(0x00cd, "call a16");
        m.insert(0x00ce, "adc a,d8");
        m.insert(0x00cf, "rst 0x08");
        m.insert(0x00d0, "ret nc");
        m.insert(0x00d1, "pop de");
        m.insert(0x00d2, "jp nc,a16");
        m.insert(0x00d4, "call nc,a16");
        m.insert(0x00d5, "push de");
        m.insert(0x00d6, "sub d8");
        m.insert(0x00d7, "rst 0x10");
        m.insert(0x00d8, "ret cf");
        m.insert(0x00d9, "reti ");
        m.insert(0x00da, "jp cf,a16");
        m.insert(0x00dc, "call cf,a16");
        m.insert(0x00de, "sbc a,d8");
        m.insert(0x00df, "rst 0x18");
        m.insert(0x00e0, "ld (0xff00+a8),a");
        m.insert(0x00e1, "pop hl");
        m.insert(0x00e2, "ld (0xff00+c),a");
        m.insert(0x00e5, "push hl");
        m.insert(0x00e6, "and d8");
        m.insert(0x00e7, "rst 0x20");
        m.insert(0x00e8, "add sp,r8");
        m.insert(0x00e9, "jp hl");
        m.insert(0x00ea, "ld (a16),a");
        m.insert(0x00ee, "xor d8");
        m.insert(0x00ef, "rst 0x28");
        m.insert(0x00f0, "ld a,(0xff00+a8)");
        m.insert(0x00f1, "pop af");
        m.insert(0x00f2, "ld a,(0xff00+c)");
        m.insert(0x00f3, "di ");
        m.insert(0x00f5, "push af");
        m.insert(0x00f6, "or d8");
        m.insert(0x00f7, "rst 0x30");
        m.insert(0x00f8, "ldhl sp,r8");
        m.insert(0x00f9, "ld sp,hl");
        m.insert(0x00fa, "ld a,(a16)");
        m.insert(0x00fb, "ei ");
        m.insert(0x00fe, "cp d8");
        m.insert(0x00ff, "rst 0x38");
        m.insert(0xcb00, "rlc b");
        m.insert(0xcb01, "rlc c");
        m.insert(0xcb02, "rlc d");
        m.insert(0xcb03, "rlc e");
        m.insert(0xcb04, "rlc h");
        m.insert(0xcb05, "rlc l");
        m.insert(0xcb06, "rlc (hl)");
        m.insert(0xcb07, "rlc a");
        m.insert(0xcb08, "rrc b");
        m.insert(0xcb09, "rrc c");
        m.insert(0xcb0a, "rrc d");
        m.insert(0xcb0b, "rrc e");
        m.insert(0xcb0c, "rrc h");
        m.insert(0xcb0d, "rrc l");
        m.insert(0xcb0e, "rrc (hl)");
        m.insert(0xcb0f, "rrc a");
        m.insert(0xcb10, "rl b");
        m.insert(0xcb11, "rl c");
        m.insert(0xcb12, "rl d");
        m.insert(0xcb13, "rl e");
        m.insert(0xcb14, "rl h");
        m.insert(0xcb15, "rl l");
        m.insert(0xcb16, "rl (hl)");
        m.insert(0xcb17, "rl a");
        m.insert(0xcb18, "rr b");
        m.insert(0xcb19, "rr c");
        m.insert(0xcb1a, "rr d");
        m.insert(0xcb1b, "rr e");
        m.insert(0xcb1c, "rr h");
        m.insert(0xcb1d, "rr l");
        m.insert(0xcb1e, "rr (hl)");
        m.insert(0xcb1f, "rr a");
        m.insert(0xcb20, "sla b");
        m.insert(0xcb21, "sla c");
        m.insert(0xcb22, "sla d");
        m.insert(0xcb23, "sla e");
        m.insert(0xcb24, "sla h");
        m.insert(0xcb25, "sla l");
        m.insert(0xcb26, "sla (hl)");
        m.insert(0xcb27, "sla a");
        m.insert(0xcb28, "sra b");
        m.insert(0xcb29, "sra c");
        m.insert(0xcb2a, "sra d");
        m.insert(0xcb2b, "sra e");
        m.insert(0xcb2c, "sra h");
        m.insert(0xcb2d, "sra l");
        m.insert(0xcb2e, "sra (hl)");
        m.insert(0xcb2f, "sra a");
        m.insert(0xcb30, "swap b");
        m.insert(0xcb31, "swap c");
        m.insert(0xcb32, "swap d");
        m.insert(0xcb33, "swap e");
        m.insert(0xcb34, "swap h");
        m.insert(0xcb35, "swap l");
        m.insert(0xcb36, "swap (hl)");
        m.insert(0xcb37, "swap a");
        m.insert(0xcb38, "srl b");
        m.insert(0xcb39, "srl c");
        m.insert(0xcb3a, "srl d");
        m.insert(0xcb3b, "srl e");
        m.insert(0xcb3c, "srl h");
        m.insert(0xcb3d, "srl l");
        m.insert(0xcb3e, "srl (hl)");
        m.insert(0xcb3f, "srl a");
        m.insert(0xcb40, "bit 0,b");
        m.insert(0xcb41, "bit 0,c");
        m.insert(0xcb42, "bit 0,d");
        m.insert(0xcb43, "bit 0,e");
        m.insert(0xcb44, "bit 0,h");
        m.insert(0xcb45, "bit 0,l");
        m.insert(0xcb46, "bit 0,(hl)");
        m.insert(0xcb47, "bit 0,a");
        m.insert(0xcb48, "bit 1,b");
        m.insert(0xcb49, "bit 1,c");
        m.insert(0xcb4a, "bit 1,d");
        m.insert(0xcb4b, "bit 1,e");
        m.insert(0xcb4c, "bit 1,h");
        m.insert(0xcb4d, "bit 1,l");
        m.insert(0xcb4e, "bit 1,(hl)");
        m.insert(0xcb4f, "bit 1,a");
        m.insert(0xcb50, "bit 2,b");
        m.insert(0xcb51, "bit 2,c");
        m.insert(0xcb52, "bit 2,d");
        m.insert(0xcb53, "bit 2,e");
        m.insert(0xcb54, "bit 2,h");
        m.insert(0xcb55, "bit 2,l");
        m.insert(0xcb56, "bit 2,(hl)");
        m.insert(0xcb57, "bit 2,a");
        m.insert(0xcb58, "bit 3,b");
        m.insert(0xcb59, "bit 3,c");
        m.insert(0xcb5a, "bit 3,d");
        m.insert(0xcb5b, "bit 3,e");
        m.insert(0xcb5c, "bit 3,h");
        m.insert(0xcb5d, "bit 3,l");
        m.insert(0xcb5e, "bit 3,(hl)");
        m.insert(0xcb5f, "bit 3,a");
        m.insert(0xcb60, "bit 4,b");
        m.insert(0xcb61, "bit 4,c");
        m.insert(0xcb62, "bit 4,d");
        m.insert(0xcb63, "bit 4,e");
        m.insert(0xcb64, "bit 4,h");
        m.insert(0xcb65, "bit 4,l");
        m.insert(0xcb66, "bit 4,(hl)");
        m.insert(0xcb67, "bit 4,a");
        m.insert(0xcb68, "bit 5,b");
        m.insert(0xcb69, "bit 5,c");
        m.insert(0xcb6a, "bit 5,d");
        m.insert(0xcb6b, "bit 5,e");
        m.insert(0xcb6c, "bit 5,h");
        m.insert(0xcb6d, "bit 5,l");
        m.insert(0xcb6e, "bit 5,(hl)");
        m.insert(0xcb6f, "bit 5,a");
        m.insert(0xcb70, "bit 6,b");
        m.insert(0xcb71, "bit 6,c");
        m.insert(0xcb72, "bit 6,d");
        m.insert(0xcb73, "bit 6,e");
        m.insert(0xcb74, "bit 6,h");
        m.insert(0xcb75, "bit 6,l");
        m.insert(0xcb76, "bit 6,(hl)");
        m.insert(0xcb77, "bit 6,a");
        m.insert(0xcb78, "bit 7,b");
        m.insert(0xcb79, "bit 7,c");
        m.insert(0xcb7a, "bit 7,d");
        m.insert(0xcb7b, "bit 7,e");
        m.insert(0xcb7c, "bit 7,h");
        m.insert(0xcb7d, "bit 7,l");
        m.insert(0xcb7e, "bit 7,(hl)");
        m.insert(0xcb7f, "bit 7,a");
        m.insert(0xcb80, "res 0,b");
        m.insert(0xcb81, "res 0,c");
        m.insert(0xcb82, "res 0,d");
        m.insert(0xcb83, "res 0,e");
        m.insert(0xcb84, "res 0,h");
        m.insert(0xcb85, "res 0,l");
        m.insert(0xcb86, "res 0,(hl)");
        m.insert(0xcb87, "res 0,a");
        m.insert(0xcb88, "res 1,b");
        m.insert(0xcb89, "res 1,c");
        m.insert(0xcb8a, "res 1,d");
        m.insert(0xcb8b, "res 1,e");
        m.insert(0xcb8c, "res 1,h");
        m.insert(0xcb8d, "res 1,l");
        m.insert(0xcb8e, "res 1,(hl)");
        m.insert(0xcb8f, "res 1,a");
        m.insert(0xcb90, "res 2,b");
        m.insert(0xcb91, "res 2,c");
        m.insert(0xcb92, "res 2,d");
        m.insert(0xcb93, "res 2,e");
        m.insert(0xcb94, "res 2,h");
        m.insert(0xcb95, "res 2,l");
        m.insert(0xcb96, "res 2,(hl)");
        m.insert(0xcb97, "res 2,a");
        m.insert(0xcb98, "res 3,b");
        m.insert(0xcb99, "res 3,c");
        m.insert(0xcb9a, "res 3,d");
        m.insert(0xcb9b, "res 3,e");
        m.insert(0xcb9c, "res 3,h");
        m.insert(0xcb9d, "res 3,l");
        m.insert(0xcb9e, "res 3,(hl)");
        m.insert(0xcb9f, "res 3,a");
        m.insert(0xcba0, "res 4,b");
        m.insert(0xcba1, "res 4,c");
        m.insert(0xcba2, "res 4,d");
        m.insert(0xcba3, "res 4,e");
        m.insert(0xcba4, "res 4,h");
        m.insert(0xcba5, "res 4,l");
        m.insert(0xcba6, "res 4,(hl)");
        m.insert(0xcba7, "res 4,a");
        m.insert(0xcba8, "res 5,b");
        m.insert(0xcba9, "res 5,c");
        m.insert(0xcbaa, "res 5,d");
        m.insert(0xcbab, "res 5,e");
        m.insert(0xcbac, "res 5,h");
        m.insert(0xcbad, "res 5,l");
        m.insert(0xcbae, "res 5,(hl)");
        m.insert(0xcbaf, "res 5,a");
        m.insert(0xcbb0, "res 6,b");
        m.insert(0xcbb1, "res 6,c");
        m.insert(0xcbb2, "res 6,d");
        m.insert(0xcbb3, "res 6,e");
        m.insert(0xcbb4, "res 6,h");
        m.insert(0xcbb5, "res 6,l");
        m.insert(0xcbb6, "res 6,(hl)");
        m.insert(0xcbb7, "res 6,a");
        m.insert(0xcbb8, "res 7,b");
        m.insert(0xcbb9, "res 7,c");
        m.insert(0xcbba, "res 7,d");
        m.insert(0xcbbb, "res 7,e");
        m.insert(0xcbbc, "res 7,h");
        m.insert(0xcbbd, "res 7,l");
        m.insert(0xcbbe, "res 7,(hl)");
        m.insert(0xcbbf, "res 7,a");
        m.insert(0xcbc0, "set 0,b");
        m.insert(0xcbc1, "set 0,c");
        m.insert(0xcbc2, "set 0,d");
        m.insert(0xcbc3, "set 0,e");
        m.insert(0xcbc4, "set 0,h");
        m.insert(0xcbc5, "set 0,l");
        m.insert(0xcbc6, "set 0,(hl)");
        m.insert(0xcbc7, "set 0,a");
        m.insert(0xcbc8, "set 1,b");
        m.insert(0xcbc9, "set 1,c");
        m.insert(0xcbca, "set 1,d");
        m.insert(0xcbcb, "set 1,e");
        m.insert(0xcbcc, "set 1,h");
        m.insert(0xcbcd, "set 1,l");
        m.insert(0xcbce, "set 1,(hl)");
        m.insert(0xcbcf, "set 1,a");
        m.insert(0xcbd0, "set 2,b");
        m.insert(0xcbd1, "set 2,c");
        m.insert(0xcbd2, "set 2,d");
        m.insert(0xcbd3, "set 2,e");
        m.insert(0xcbd4, "set 2,h");
        m.insert(0xcbd5, "set 2,l");
        m.insert(0xcbd6, "set 2,(hl)");
        m.insert(0xcbd7, "set 2,a");
        m.insert(0xcbd8, "set 3,b");
        m.insert(0xcbd9, "set 3,c");
        m.insert(0xcbda, "set 3,d");
        m.insert(0xcbdb, "set 3,e");
        m.insert(0xcbdc, "set 3,h");
        m.insert(0xcbdd, "set 3,l");
        m.insert(0xcbde, "set 3,(hl)");
        m.insert(0xcbdf, "set 3,a");
        m.insert(0xcbe0, "set 4,b");
        m.insert(0xcbe1, "set 4,c");
        m.insert(0xcbe2, "set 4,d");
        m.insert(0xcbe3, "set 4,e");
        m.insert(0xcbe4, "set 4,h");
        m.insert(0xcbe5, "set 4,l");
        m.insert(0xcbe6, "set 4,(hl)");
        m.insert(0xcbe7, "set 4,a");
        m.insert(0xcbe8, "set 5,b");
        m.insert(0xcbe9, "set 5,c");
        m.insert(0xcbea, "set 5,d");
        m.insert(0xcbeb, "set 5,e");
        m.insert(0xcbec, "set 5,h");
        m.insert(0xcbed, "set 5,l");
        m.insert(0xcbee, "set 5,(hl)");
        m.insert(0xcbef, "set 5,a");
        m.insert(0xcbf0, "set 6,b");
        m.insert(0xcbf1, "set 6,c");
        m.insert(0xcbf2, "set 6,d");
        m.insert(0xcbf3, "set 6,e");
        m.insert(0xcbf4, "set 6,h");
        m.insert(0xcbf5, "set 6,l");
        m.insert(0xcbf6, "set 6,(hl)");
        m.insert(0xcbf7, "set 6,a");
        m.insert(0xcbf8, "set 7,b");
        m.insert(0xcbf9, "set 7,c");
        m.insert(0xcbfa, "set 7,d");
        m.insert(0xcbfb, "set 7,e");
        m.insert(0xcbfc, "set 7,h");
        m.insert(0xcbfd, "set 7,l");
        m.insert(0xcbfe, "set 7,(hl)");
        m.insert(0xcbff, "set 7,a");
        m
    };
}

/// nop
#[allow(unused_variables)]
fn op_0000(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    (4, 1)
}

/// ld bc,d16
#[allow(unused_variables)]
fn op_0001(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get16(cpu.get_pc().wrapping_add(arg));
    cpu.set_bc(v);

    (12, 3)
}

/// ld (bc),a
#[allow(unused_variables)]
fn op_0002(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    mmu.set8(cpu.get_bc(), v);

    (8, 1)
}

/// inc bc
#[allow(unused_variables)]
fn op_0003(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_bc().wrapping_add(1);
    cpu.set_bc(v);

    (8, 1)
}

/// inc b
#[allow(unused_variables)]
fn op_0004(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

/// dec b
#[allow(unused_variables)]
fn op_0005(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

/// ld b,d8
#[allow(unused_variables)]
fn op_0006(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_b(v);

    (8, 2)
}

/// rlca
#[allow(unused_variables)]
fn op_0007(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0008(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_sp();
    mmu.set16(mmu.get16(cpu.get_pc().wrapping_add(arg)), v);

    (20, 3)
}

/// add hl,bc
#[allow(unused_variables)]
fn op_0009(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_000a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_bc());
    cpu.set_a(v);

    (8, 1)
}

/// dec bc
#[allow(unused_variables)]
fn op_000b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_bc().wrapping_sub(1);
    cpu.set_bc(v);

    (8, 1)
}

/// inc c
#[allow(unused_variables)]
fn op_000c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

/// dec c
#[allow(unused_variables)]
fn op_000d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_c(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

/// ld c,d8
#[allow(unused_variables)]
fn op_000e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_c(v);

    (8, 2)
}

/// rrca
#[allow(unused_variables)]
fn op_000f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0010(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.stop();

    (4, 2)
}

/// ld de,d16
#[allow(unused_variables)]
fn op_0011(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get16(cpu.get_pc().wrapping_add(arg));
    cpu.set_de(v);

    (12, 3)
}

/// ld (de),a
#[allow(unused_variables)]
fn op_0012(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    mmu.set8(cpu.get_de(), v);

    (8, 1)
}

/// inc de
#[allow(unused_variables)]
fn op_0013(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_de().wrapping_add(1);
    cpu.set_de(v);

    (8, 1)
}

/// inc d
#[allow(unused_variables)]
fn op_0014(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

/// dec d
#[allow(unused_variables)]
fn op_0015(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_d(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

/// ld d,d8
#[allow(unused_variables)]
fn op_0016(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_d(v);

    (8, 2)
}

/// rla
#[allow(unused_variables)]
fn op_0017(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0018(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let pc = cpu.get_pc().wrapping_add(alu::signed(p));
    cpu.set_pc(pc);

    (12, 2)
}

/// add hl,de
#[allow(unused_variables)]
fn op_0019(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_001a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_de());
    cpu.set_a(v);

    (8, 1)
}

/// dec de
#[allow(unused_variables)]
fn op_001b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_de().wrapping_sub(1);
    cpu.set_de(v);

    (8, 1)
}

/// inc e
#[allow(unused_variables)]
fn op_001c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

/// dec e
#[allow(unused_variables)]
fn op_001d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_e(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

/// ld e,d8
#[allow(unused_variables)]
fn op_001e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_e(v);

    (8, 2)
}

/// rra
#[allow(unused_variables)]
fn op_001f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0020(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let flg = !cpu.get_zf();
    if flg {
        let p = mmu.get8(cpu.get_pc().wrapping_add(arg));
        let pc = cpu.get_pc().wrapping_add(alu::signed(p));
        cpu.set_pc(pc);
        return (12, 2);
    }

    (8, 2)
}

/// ld hl,d16
#[allow(unused_variables)]
fn op_0021(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get16(cpu.get_pc().wrapping_add(arg));
    cpu.set_hl(v);

    (12, 3)
}

/// ldi (hl),a
#[allow(unused_variables)]
fn op_0022(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    mmu.set8(cpu.get_hl(), v);

    cpu.set_hl(cpu.get_hl().wrapping_add(1));

    (8, 1)
}

/// inc hl
#[allow(unused_variables)]
fn op_0023(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_hl().wrapping_add(1);
    cpu.set_hl(v);

    (8, 1)
}

/// inc h
#[allow(unused_variables)]
fn op_0024(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

/// dec h
#[allow(unused_variables)]
fn op_0025(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_h(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

/// ld h,d8
#[allow(unused_variables)]
fn op_0026(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_h(v);

    (8, 2)
}

/// daa
#[allow(unused_variables)]
fn op_0027(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let l = cpu.get_a() & 0xf;
    let h = cpu.get_a() >> 4;

    let lc = if l > 9 || cpu.get_hf() { 0x06 } else { 0x00 };
    let hc = if h > 9 || cpu.get_cf() { 0x60 } else { 0x00 };

    let v = cpu.get_a();
    let v = if cpu.get_nf() {
        v.wrapping_sub(lc + hc)
    } else {
        v.wrapping_add(lc + hc)
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
#[allow(unused_variables)]
fn op_0028(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let flg = cpu.get_zf();
    if flg {
        let p = mmu.get8(cpu.get_pc().wrapping_add(arg));
        let pc = cpu.get_pc().wrapping_add(alu::signed(p));
        cpu.set_pc(pc);
        return (12, 2);
    }

    (8, 2)
}

/// add hl,hl
#[allow(unused_variables)]
fn op_0029(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_002a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_a(v);

    cpu.set_hl(cpu.get_hl().wrapping_add(1));

    (8, 1)
}

/// dec hl
#[allow(unused_variables)]
fn op_002b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_hl().wrapping_sub(1);
    cpu.set_hl(v);

    (8, 1)
}

/// inc l
#[allow(unused_variables)]
fn op_002c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

/// dec l
#[allow(unused_variables)]
fn op_002d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_l(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

/// ld l,d8
#[allow(unused_variables)]
fn op_002e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_l(v);

    (8, 2)
}

/// cpl
#[allow(unused_variables)]
fn op_002f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ 0xff);

    cpu.set_nf(true);
    cpu.set_hf(true);

    (4, 1)
}

/// jr nc,r8
#[allow(unused_variables)]
fn op_0030(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let flg = !cpu.get_cf();
    if flg {
        let p = mmu.get8(cpu.get_pc().wrapping_add(arg));
        let pc = cpu.get_pc().wrapping_add(alu::signed(p));
        cpu.set_pc(pc);
        return (12, 2);
    }

    (8, 2)
}

/// ld sp,d16
#[allow(unused_variables)]
fn op_0031(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get16(cpu.get_pc().wrapping_add(arg));
    cpu.set_sp(v);

    (12, 3)
}

/// ldd (hl),a
#[allow(unused_variables)]
fn op_0032(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    mmu.set8(cpu.get_hl(), v);

    cpu.set_hl(cpu.get_hl().wrapping_sub(1));

    (8, 1)
}

/// inc sp
#[allow(unused_variables)]
fn op_0033(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_sp().wrapping_add(1);
    cpu.set_sp(v);

    (8, 1)
}

/// inc (hl)
#[allow(unused_variables)]
fn op_0034(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let (v, h, c, z) = alu::add8(v, 1, false);
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (12, 1)
}

/// dec (hl)
#[allow(unused_variables)]
fn op_0035(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    let (v, h, c, z) = alu::sub8(v, 1, false);
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (12, 1)
}

/// ld (hl),d8
#[allow(unused_variables)]
fn op_0036(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    mmu.set8(cpu.get_hl(), v);

    (12, 2)
}

/// scf
#[allow(unused_variables)]
fn op_0037(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_cf(true);

    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(true);

    (4, 1)
}

/// jr cf,r8
#[allow(unused_variables)]
fn op_0038(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let flg = cpu.get_cf();
    if flg {
        let p = mmu.get8(cpu.get_pc().wrapping_add(arg));
        let pc = cpu.get_pc().wrapping_add(alu::signed(p));
        cpu.set_pc(pc);
        return (12, 2);
    }

    (8, 2)
}

/// add hl,sp
#[allow(unused_variables)]
fn op_0039(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_003a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_a(v);

    cpu.set_hl(cpu.get_hl().wrapping_sub(1));

    (8, 1)
}

/// dec sp
#[allow(unused_variables)]
fn op_003b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_sp().wrapping_sub(1);
    cpu.set_sp(v);

    (8, 1)
}

/// inc a
#[allow(unused_variables)]
fn op_003c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

/// dec a
#[allow(unused_variables)]
fn op_003d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

/// ld a,d8
#[allow(unused_variables)]
fn op_003e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_a(v);

    (8, 2)
}

/// ccf
#[allow(unused_variables)]
fn op_003f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let c = !cpu.get_cf();

    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(c);

    (4, 1)
}

/// ld b,b
#[allow(unused_variables)]
fn op_0040(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_b(v);

    (4, 1)
}

/// ld b,c
#[allow(unused_variables)]
fn op_0041(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_b(v);

    (4, 1)
}

/// ld b,d
#[allow(unused_variables)]
fn op_0042(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_b(v);

    (4, 1)
}

/// ld b,e
#[allow(unused_variables)]
fn op_0043(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_b(v);

    (4, 1)
}

/// ld b,h
#[allow(unused_variables)]
fn op_0044(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_b(v);

    (4, 1)
}

/// ld b,l
#[allow(unused_variables)]
fn op_0045(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_b(v);

    (4, 1)
}

/// ld b,(hl)
#[allow(unused_variables)]
fn op_0046(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_b(v);

    (8, 1)
}

/// ld b,a
#[allow(unused_variables)]
fn op_0047(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_b(v);

    (4, 1)
}

/// ld c,b
#[allow(unused_variables)]
fn op_0048(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_c(v);

    (4, 1)
}

/// ld c,c
#[allow(unused_variables)]
fn op_0049(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_c(v);

    (4, 1)
}

/// ld c,d
#[allow(unused_variables)]
fn op_004a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_c(v);

    (4, 1)
}

/// ld c,e
#[allow(unused_variables)]
fn op_004b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_c(v);

    (4, 1)
}

/// ld c,h
#[allow(unused_variables)]
fn op_004c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_c(v);

    (4, 1)
}

/// ld c,l
#[allow(unused_variables)]
fn op_004d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_c(v);

    (4, 1)
}

/// ld c,(hl)
#[allow(unused_variables)]
fn op_004e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_c(v);

    (8, 1)
}

/// ld c,a
#[allow(unused_variables)]
fn op_004f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_c(v);

    (4, 1)
}

/// ld d,b
#[allow(unused_variables)]
fn op_0050(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_d(v);

    (4, 1)
}

/// ld d,c
#[allow(unused_variables)]
fn op_0051(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_d(v);

    (4, 1)
}

/// ld d,d
#[allow(unused_variables)]
fn op_0052(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_d(v);

    (4, 1)
}

/// ld d,e
#[allow(unused_variables)]
fn op_0053(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_d(v);

    (4, 1)
}

/// ld d,h
#[allow(unused_variables)]
fn op_0054(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_d(v);

    (4, 1)
}

/// ld d,l
#[allow(unused_variables)]
fn op_0055(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_d(v);

    (4, 1)
}

/// ld d,(hl)
#[allow(unused_variables)]
fn op_0056(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_d(v);

    (8, 1)
}

/// ld d,a
#[allow(unused_variables)]
fn op_0057(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_d(v);

    (4, 1)
}

/// ld e,b
#[allow(unused_variables)]
fn op_0058(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_e(v);

    (4, 1)
}

/// ld e,c
#[allow(unused_variables)]
fn op_0059(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_e(v);

    (4, 1)
}

/// ld e,d
#[allow(unused_variables)]
fn op_005a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_e(v);

    (4, 1)
}

/// ld e,e
#[allow(unused_variables)]
fn op_005b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_e(v);

    (4, 1)
}

/// ld e,h
#[allow(unused_variables)]
fn op_005c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_e(v);

    (4, 1)
}

/// ld e,l
#[allow(unused_variables)]
fn op_005d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_e(v);

    (4, 1)
}

/// ld e,(hl)
#[allow(unused_variables)]
fn op_005e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_e(v);

    (8, 1)
}

/// ld e,a
#[allow(unused_variables)]
fn op_005f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_e(v);

    (4, 1)
}

/// ld h,b
#[allow(unused_variables)]
fn op_0060(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_h(v);

    (4, 1)
}

/// ld h,c
#[allow(unused_variables)]
fn op_0061(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_h(v);

    (4, 1)
}

/// ld h,d
#[allow(unused_variables)]
fn op_0062(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_h(v);

    (4, 1)
}

/// ld h,e
#[allow(unused_variables)]
fn op_0063(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_h(v);

    (4, 1)
}

/// ld h,h
#[allow(unused_variables)]
fn op_0064(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_h(v);

    (4, 1)
}

/// ld h,l
#[allow(unused_variables)]
fn op_0065(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_h(v);

    (4, 1)
}

/// ld h,(hl)
#[allow(unused_variables)]
fn op_0066(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_h(v);

    (8, 1)
}

/// ld h,a
#[allow(unused_variables)]
fn op_0067(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_h(v);

    (4, 1)
}

/// ld l,b
#[allow(unused_variables)]
fn op_0068(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_l(v);

    (4, 1)
}

/// ld l,c
#[allow(unused_variables)]
fn op_0069(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_l(v);

    (4, 1)
}

/// ld l,d
#[allow(unused_variables)]
fn op_006a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_l(v);

    (4, 1)
}

/// ld l,e
#[allow(unused_variables)]
fn op_006b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_l(v);

    (4, 1)
}

/// ld l,h
#[allow(unused_variables)]
fn op_006c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_l(v);

    (4, 1)
}

/// ld l,l
#[allow(unused_variables)]
fn op_006d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_l(v);

    (4, 1)
}

/// ld l,(hl)
#[allow(unused_variables)]
fn op_006e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_l(v);

    (8, 1)
}

/// ld l,a
#[allow(unused_variables)]
fn op_006f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_l(v);

    (4, 1)
}

/// ld (hl),b
#[allow(unused_variables)]
fn op_0070(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

/// ld (hl),c
#[allow(unused_variables)]
fn op_0071(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

/// ld (hl),d
#[allow(unused_variables)]
fn op_0072(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

/// ld (hl),e
#[allow(unused_variables)]
fn op_0073(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

/// ld (hl),h
#[allow(unused_variables)]
fn op_0074(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

/// ld (hl),l
#[allow(unused_variables)]
fn op_0075(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

/// halt
#[allow(unused_variables)]
fn op_0076(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.halt();

    (4, 1)
}

/// ld (hl),a
#[allow(unused_variables)]
fn op_0077(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

/// ld a,b
#[allow(unused_variables)]
fn op_0078(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_a(v);

    (4, 1)
}

/// ld a,c
#[allow(unused_variables)]
fn op_0079(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_a(v);

    (4, 1)
}

/// ld a,d
#[allow(unused_variables)]
fn op_007a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_a(v);

    (4, 1)
}

/// ld a,e
#[allow(unused_variables)]
fn op_007b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_a(v);

    (4, 1)
}

/// ld a,h
#[allow(unused_variables)]
fn op_007c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_a(v);

    (4, 1)
}

/// ld a,l
#[allow(unused_variables)]
fn op_007d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_a(v);

    (4, 1)
}

/// ld a,(hl)
#[allow(unused_variables)]
fn op_007e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(cpu.get_hl());
    cpu.set_a(v);

    (8, 1)
}

/// ld a,a
#[allow(unused_variables)]
fn op_007f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_a(v);

    (4, 1)
}

/// add a,b
#[allow(unused_variables)]
fn op_0080(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0081(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0082(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0083(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0084(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0085(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0086(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0087(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0088(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0089(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_008a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_008b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_008c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_008d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_008e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_008f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0090(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0091(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0092(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0093(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0094(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0095(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0096(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0097(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0098(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_0099(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_009a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_009b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_009c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_009d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_009e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_009f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_00a0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_b());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

/// and c
#[allow(unused_variables)]
fn op_00a1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_c());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

/// and d
#[allow(unused_variables)]
fn op_00a2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_d());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

/// and e
#[allow(unused_variables)]
fn op_00a3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_e());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

/// and h
#[allow(unused_variables)]
fn op_00a4(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_h());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

/// and l
#[allow(unused_variables)]
fn op_00a5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_l());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

/// and (hl)
#[allow(unused_variables)]
fn op_00a6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & mmu.get8(cpu.get_hl()));
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (8, 1)
}

/// and a
#[allow(unused_variables)]
fn op_00a7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & cpu.get_a());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (4, 1)
}

/// xor b
#[allow(unused_variables)]
fn op_00a8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ cpu.get_b());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// xor c
#[allow(unused_variables)]
fn op_00a9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ cpu.get_c());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// xor d
#[allow(unused_variables)]
fn op_00aa(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ cpu.get_d());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// xor e
#[allow(unused_variables)]
fn op_00ab(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ cpu.get_e());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// xor h
#[allow(unused_variables)]
fn op_00ac(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ cpu.get_h());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// xor l
#[allow(unused_variables)]
fn op_00ad(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ cpu.get_l());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// xor (hl)
#[allow(unused_variables)]
fn op_00ae(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ mmu.get8(cpu.get_hl()));
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 1)
}

/// xor a
#[allow(unused_variables)]
fn op_00af(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ cpu.get_a());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// or b
#[allow(unused_variables)]
fn op_00b0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | cpu.get_b());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// or c
#[allow(unused_variables)]
fn op_00b1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | cpu.get_c());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// or d
#[allow(unused_variables)]
fn op_00b2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | cpu.get_d());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// or e
#[allow(unused_variables)]
fn op_00b3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | cpu.get_e());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// or h
#[allow(unused_variables)]
fn op_00b4(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | cpu.get_h());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// or l
#[allow(unused_variables)]
fn op_00b5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | cpu.get_l());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// or (hl)
#[allow(unused_variables)]
fn op_00b6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | mmu.get8(cpu.get_hl()));
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 1)
}

/// or a
#[allow(unused_variables)]
fn op_00b7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | cpu.get_a());
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (4, 1)
}

/// cp b
#[allow(unused_variables)]
fn op_00b8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_00b9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_00ba(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_00bb(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_00bc(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_00bd(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_00be(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_00bf(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_00c0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let flg = !cpu.get_zf();
    if flg {
        let pc = cpu.pop(mmu);
        cpu.set_pc(pc);
        return (20, 0);
    }

    (8, 1)
}

/// pop bc
#[allow(unused_variables)]
fn op_00c1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.pop(mmu);
    cpu.set_bc(v);

    (12, 1)
}

/// jp nz,a16
#[allow(unused_variables)]
fn op_00c2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let flg = !cpu.get_zf();
    if flg {
        let pc = mmu.get16(cpu.get_pc().wrapping_add(arg));
        cpu.set_pc(pc);
        return (16, 0);
    }

    (12, 3)
}

/// jp a16
#[allow(unused_variables)]
fn op_00c3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let pc = mmu.get16(cpu.get_pc().wrapping_add(arg));
    cpu.set_pc(pc.wrapping_sub(3));

    (16, 3)
}

/// call nz,a16
#[allow(unused_variables)]
fn op_00c4(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let flg = !cpu.get_zf();
    if flg {
        cpu.push(mmu, cpu.get_pc().wrapping_add(3));
        cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(arg)));
        return (24, 0);
    }

    (12, 3)
}

/// push bc
#[allow(unused_variables)]
fn op_00c5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_bc());

    (16, 1)
}

/// add a,d8
#[allow(unused_variables)]
fn op_00c6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

/// rst 0x00
#[allow(unused_variables)]
fn op_00c7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x00u16.wrapping_sub(1));

    (16, 1)
}

/// ret z
#[allow(unused_variables)]
fn op_00c8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let flg = cpu.get_zf();
    if flg {
        let pc = cpu.pop(mmu);
        cpu.set_pc(pc);
        return (20, 0);
    }

    (8, 1)
}

/// ret
#[allow(unused_variables)]
fn op_00c9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let pc = cpu.pop(mmu).wrapping_sub(1);
    cpu.set_pc(pc);

    (16, 1)
}

/// jp z,a16
#[allow(unused_variables)]
fn op_00ca(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let flg = cpu.get_zf();
    if flg {
        let pc = mmu.get16(cpu.get_pc().wrapping_add(arg));
        cpu.set_pc(pc);
        return (16, 0);
    }

    (12, 3)
}

/// prefix cb
#[allow(unused_variables)]
fn op_00cb(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    (4, 1)
}

/// call z,a16
#[allow(unused_variables)]
fn op_00cc(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let flg = cpu.get_zf();
    if flg {
        cpu.push(mmu, cpu.get_pc().wrapping_add(3));
        cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(arg)));
        return (24, 0);
    }

    (12, 3)
}

/// call a16
#[allow(unused_variables)]
fn op_00cd(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_pc().wrapping_add(3));
    cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(arg)).wrapping_sub(3));

    (24, 3)
}

/// adc a,d8
#[allow(unused_variables)]
fn op_00ce(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

/// rst 0x08
#[allow(unused_variables)]
fn op_00cf(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x08u16.wrapping_sub(1));

    (16, 1)
}

/// ret nc
#[allow(unused_variables)]
fn op_00d0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let flg = !cpu.get_cf();
    if flg {
        let pc = cpu.pop(mmu);
        cpu.set_pc(pc);
        return (20, 0);
    }

    (8, 1)
}

/// pop de
#[allow(unused_variables)]
fn op_00d1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.pop(mmu);
    cpu.set_de(v);

    (12, 1)
}

/// jp nc,a16
#[allow(unused_variables)]
fn op_00d2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let flg = !cpu.get_cf();
    if flg {
        let pc = mmu.get16(cpu.get_pc().wrapping_add(arg));
        cpu.set_pc(pc);
        return (16, 0);
    }

    (12, 3)
}

/// call nc,a16
#[allow(unused_variables)]
fn op_00d4(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let flg = !cpu.get_cf();
    if flg {
        cpu.push(mmu, cpu.get_pc().wrapping_add(3));
        cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(arg)));
        return (24, 0);
    }

    (12, 3)
}

/// push de
#[allow(unused_variables)]
fn op_00d5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_de());

    (16, 1)
}

/// sub d8
#[allow(unused_variables)]
fn op_00d6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let (v, h, c, z) = alu::sub8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

/// rst 0x10
#[allow(unused_variables)]
fn op_00d7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x10u16.wrapping_sub(1));

    (16, 1)
}

/// ret cf
#[allow(unused_variables)]
fn op_00d8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let flg = cpu.get_cf();
    if flg {
        let pc = cpu.pop(mmu);
        cpu.set_pc(pc);
        return (20, 0);
    }

    (8, 1)
}

/// reti
#[allow(unused_variables)]
fn op_00d9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let pc = cpu.pop(mmu).wrapping_sub(1);
    cpu.set_pc(pc);
    cpu.enable_interrupt();

    (16, 1)
}

/// jp cf,a16
#[allow(unused_variables)]
fn op_00da(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let flg = cpu.get_cf();
    if flg {
        let pc = mmu.get16(cpu.get_pc().wrapping_add(arg));
        cpu.set_pc(pc);
        return (16, 0);
    }

    (12, 3)
}

/// call cf,a16
#[allow(unused_variables)]
fn op_00dc(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let flg = cpu.get_cf();
    if flg {
        cpu.push(mmu, cpu.get_pc().wrapping_add(3));
        cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(arg)));
        return (24, 0);
    }

    (12, 3)
}

/// sbc a,d8
#[allow(unused_variables)]
fn op_00de(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

/// rst 0x18
#[allow(unused_variables)]
fn op_00df(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x18u16.wrapping_sub(1));

    (16, 1)
}

/// ld (0xff00+a8),a
#[allow(unused_variables)]
fn op_00e0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    mmu.set8(0xff00 + mmu.get8(cpu.get_pc().wrapping_add(arg)) as u16, v);

    (12, 2)
}

/// pop hl
#[allow(unused_variables)]
fn op_00e1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.pop(mmu);
    cpu.set_hl(v);

    (12, 1)
}

/// ld (0xff00+c),a
#[allow(unused_variables)]
fn op_00e2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    mmu.set8(0xff00 + cpu.get_c() as u16, v);

    (8, 1)
}

/// push hl
#[allow(unused_variables)]
fn op_00e5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_hl());

    (16, 1)
}

/// and d8
#[allow(unused_variables)]
fn op_00e6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() & mmu.get8(cpu.get_pc().wrapping_add(arg)));
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);
    cpu.set_cf(false);

    (8, 2)
}

/// rst 0x20
#[allow(unused_variables)]
fn op_00e7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x20u16.wrapping_sub(1));

    (16, 1)
}

/// add sp,r8
#[allow(unused_variables)]
fn op_00e8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = cpu.get_sp();
    let q = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let (v, h, c, z) = alu::add16e(p, q, false);
    cpu.set_sp(v);
    cpu.set_zf(false);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (16, 2)
}

/// jp hl
#[allow(unused_variables)]
fn op_00e9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let pc = cpu.get_hl();
    cpu.set_pc(pc.wrapping_sub(1));

    (4, 1)
}

/// ld (a16),a
#[allow(unused_variables)]
fn op_00ea(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    mmu.set8(mmu.get16(cpu.get_pc().wrapping_add(arg)), v);

    (16, 3)
}

/// xor d8
#[allow(unused_variables)]
fn op_00ee(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() ^ mmu.get8(cpu.get_pc().wrapping_add(arg)));
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// rst 0x28
#[allow(unused_variables)]
fn op_00ef(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x28u16.wrapping_sub(1));

    (16, 1)
}

/// ld a,(0xff00+a8)
#[allow(unused_variables)]
fn op_00f0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(0xff00 + mmu.get8(cpu.get_pc().wrapping_add(arg)) as u16);
    cpu.set_a(v);

    (12, 2)
}

/// pop af
#[allow(unused_variables)]
fn op_00f1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.pop(mmu);
    cpu.set_af(v);

    (12, 1)
}

/// ld a,(0xff00+c)
#[allow(unused_variables)]
fn op_00f2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(0xff00 + cpu.get_c() as u16);
    cpu.set_a(v);

    (8, 1)
}

/// di
#[allow(unused_variables)]
fn op_00f3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.disable_interrupt();

    (4, 1)
}

/// push af
#[allow(unused_variables)]
fn op_00f5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_af());

    (16, 1)
}

/// or d8
#[allow(unused_variables)]
fn op_00f6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.set_a(cpu.get_a() | mmu.get8(cpu.get_pc().wrapping_add(arg)));
    let z = cpu.get_a() == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(false);
    cpu.set_cf(false);

    (8, 2)
}

/// rst 0x30
#[allow(unused_variables)]
fn op_00f7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x30u16.wrapping_sub(1));

    (16, 1)
}

/// ldhl sp,r8
#[allow(unused_variables)]
fn op_00f8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = cpu.get_sp();
    let q = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let (v, h, c, z) = alu::add16e(p, q, false);
    cpu.set_hl(v);
    cpu.set_zf(false);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (12, 2)
}

/// ld sp,hl
#[allow(unused_variables)]
fn op_00f9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = cpu.get_hl();
    cpu.set_sp(v);

    (8, 1)
}

/// ld a,(a16)
#[allow(unused_variables)]
fn op_00fa(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let v = mmu.get8(mmu.get16(cpu.get_pc().wrapping_add(arg)));
    cpu.set_a(v);

    (16, 3)
}

/// ei
#[allow(unused_variables)]
fn op_00fb(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.enable_interrupt();

    (4, 1)
}

/// cp d8
#[allow(unused_variables)]
fn op_00fe(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

/// rst 0x38
#[allow(unused_variables)]
fn op_00ff(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x38u16.wrapping_sub(1));

    (16, 1)
}

/// rlc b
#[allow(unused_variables)]
fn op_cb00(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb01(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb02(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb03(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb04(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb05(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb06(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb07(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb08(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb09(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb0a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb0b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb0c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb0d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb0e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb0f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb10(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb11(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb12(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb13(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb14(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb15(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb16(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb17(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb18(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb19(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb1a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb1b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb1c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb1d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb1e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb1f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb20(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb21(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb22(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb23(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb24(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb25(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb26(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb27(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb28(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb29(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb2a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb2b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb2c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb2d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb2e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb2f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb30(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb31(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb32(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb33(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb34(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb35(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb36(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb37(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb38(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb39(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb3a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb3b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb3c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb3d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb3e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb3f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
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
#[allow(unused_variables)]
fn op_cb40(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 0,c
#[allow(unused_variables)]
fn op_cb41(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 0,d
#[allow(unused_variables)]
fn op_cb42(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 0,e
#[allow(unused_variables)]
fn op_cb43(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 0,h
#[allow(unused_variables)]
fn op_cb44(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 0,l
#[allow(unused_variables)]
fn op_cb45(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 0,(hl)
#[allow(unused_variables)]
fn op_cb46(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (16, 2)
}

/// bit 0,a
#[allow(unused_variables)]
fn op_cb47(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 1,b
#[allow(unused_variables)]
fn op_cb48(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 1,c
#[allow(unused_variables)]
fn op_cb49(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 1,d
#[allow(unused_variables)]
fn op_cb4a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 1,e
#[allow(unused_variables)]
fn op_cb4b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 1,h
#[allow(unused_variables)]
fn op_cb4c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 1,l
#[allow(unused_variables)]
fn op_cb4d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 1,(hl)
#[allow(unused_variables)]
fn op_cb4e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (16, 2)
}

/// bit 1,a
#[allow(unused_variables)]
fn op_cb4f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 2,b
#[allow(unused_variables)]
fn op_cb50(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 2,c
#[allow(unused_variables)]
fn op_cb51(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 2,d
#[allow(unused_variables)]
fn op_cb52(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 2,e
#[allow(unused_variables)]
fn op_cb53(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 2,h
#[allow(unused_variables)]
fn op_cb54(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 2,l
#[allow(unused_variables)]
fn op_cb55(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 2,(hl)
#[allow(unused_variables)]
fn op_cb56(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (16, 2)
}

/// bit 2,a
#[allow(unused_variables)]
fn op_cb57(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 3,b
#[allow(unused_variables)]
fn op_cb58(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 3,c
#[allow(unused_variables)]
fn op_cb59(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 3,d
#[allow(unused_variables)]
fn op_cb5a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 3,e
#[allow(unused_variables)]
fn op_cb5b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 3,h
#[allow(unused_variables)]
fn op_cb5c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 3,l
#[allow(unused_variables)]
fn op_cb5d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 3,(hl)
#[allow(unused_variables)]
fn op_cb5e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (16, 2)
}

/// bit 3,a
#[allow(unused_variables)]
fn op_cb5f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 4,b
#[allow(unused_variables)]
fn op_cb60(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 4,c
#[allow(unused_variables)]
fn op_cb61(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 4,d
#[allow(unused_variables)]
fn op_cb62(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 4,e
#[allow(unused_variables)]
fn op_cb63(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 4,h
#[allow(unused_variables)]
fn op_cb64(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 4,l
#[allow(unused_variables)]
fn op_cb65(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 4,(hl)
#[allow(unused_variables)]
fn op_cb66(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (16, 2)
}

/// bit 4,a
#[allow(unused_variables)]
fn op_cb67(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 5,b
#[allow(unused_variables)]
fn op_cb68(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 5,c
#[allow(unused_variables)]
fn op_cb69(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 5,d
#[allow(unused_variables)]
fn op_cb6a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 5,e
#[allow(unused_variables)]
fn op_cb6b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 5,h
#[allow(unused_variables)]
fn op_cb6c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 5,l
#[allow(unused_variables)]
fn op_cb6d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 5,(hl)
#[allow(unused_variables)]
fn op_cb6e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (16, 2)
}

/// bit 5,a
#[allow(unused_variables)]
fn op_cb6f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 6,b
#[allow(unused_variables)]
fn op_cb70(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 6,c
#[allow(unused_variables)]
fn op_cb71(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 6,d
#[allow(unused_variables)]
fn op_cb72(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 6,e
#[allow(unused_variables)]
fn op_cb73(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 6,h
#[allow(unused_variables)]
fn op_cb74(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 6,l
#[allow(unused_variables)]
fn op_cb75(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 6,(hl)
#[allow(unused_variables)]
fn op_cb76(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (16, 2)
}

/// bit 6,a
#[allow(unused_variables)]
fn op_cb77(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 7,b
#[allow(unused_variables)]
fn op_cb78(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_b();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 7,c
#[allow(unused_variables)]
fn op_cb79(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_c();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 7,d
#[allow(unused_variables)]
fn op_cb7a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_d();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 7,e
#[allow(unused_variables)]
fn op_cb7b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_e();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 7,h
#[allow(unused_variables)]
fn op_cb7c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_h();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 7,l
#[allow(unused_variables)]
fn op_cb7d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_l();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// bit 7,(hl)
#[allow(unused_variables)]
fn op_cb7e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = mmu.get8(cpu.get_hl());
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (16, 2)
}

/// bit 7,a
#[allow(unused_variables)]
fn op_cb7f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_a();
    let z = q & (1 << p) == 0;
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(true);

    (8, 2)
}

/// res 0,b
#[allow(unused_variables)]
fn op_cb80(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 0,c
#[allow(unused_variables)]
fn op_cb81(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 0,d
#[allow(unused_variables)]
fn op_cb82(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 0,e
#[allow(unused_variables)]
fn op_cb83(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 0,h
#[allow(unused_variables)]
fn op_cb84(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 0,l
#[allow(unused_variables)]
fn op_cb85(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 0,(hl)
#[allow(unused_variables)]
fn op_cb86(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 0,a
#[allow(unused_variables)]
fn op_cb87(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 1,b
#[allow(unused_variables)]
fn op_cb88(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 1,c
#[allow(unused_variables)]
fn op_cb89(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 1,d
#[allow(unused_variables)]
fn op_cb8a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 1,e
#[allow(unused_variables)]
fn op_cb8b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 1,h
#[allow(unused_variables)]
fn op_cb8c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 1,l
#[allow(unused_variables)]
fn op_cb8d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 1,(hl)
#[allow(unused_variables)]
fn op_cb8e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 1,a
#[allow(unused_variables)]
fn op_cb8f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 2,b
#[allow(unused_variables)]
fn op_cb90(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 2,c
#[allow(unused_variables)]
fn op_cb91(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 2,d
#[allow(unused_variables)]
fn op_cb92(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 2,e
#[allow(unused_variables)]
fn op_cb93(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 2,h
#[allow(unused_variables)]
fn op_cb94(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 2,l
#[allow(unused_variables)]
fn op_cb95(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 2,(hl)
#[allow(unused_variables)]
fn op_cb96(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 2,a
#[allow(unused_variables)]
fn op_cb97(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 3,b
#[allow(unused_variables)]
fn op_cb98(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 3,c
#[allow(unused_variables)]
fn op_cb99(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 3,d
#[allow(unused_variables)]
fn op_cb9a(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 3,e
#[allow(unused_variables)]
fn op_cb9b(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 3,h
#[allow(unused_variables)]
fn op_cb9c(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 3,l
#[allow(unused_variables)]
fn op_cb9d(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 3,(hl)
#[allow(unused_variables)]
fn op_cb9e(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 3,a
#[allow(unused_variables)]
fn op_cb9f(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 4,b
#[allow(unused_variables)]
fn op_cba0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 4,c
#[allow(unused_variables)]
fn op_cba1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 4,d
#[allow(unused_variables)]
fn op_cba2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 4,e
#[allow(unused_variables)]
fn op_cba3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 4,h
#[allow(unused_variables)]
fn op_cba4(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 4,l
#[allow(unused_variables)]
fn op_cba5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 4,(hl)
#[allow(unused_variables)]
fn op_cba6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 4,a
#[allow(unused_variables)]
fn op_cba7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 5,b
#[allow(unused_variables)]
fn op_cba8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 5,c
#[allow(unused_variables)]
fn op_cba9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 5,d
#[allow(unused_variables)]
fn op_cbaa(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 5,e
#[allow(unused_variables)]
fn op_cbab(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 5,h
#[allow(unused_variables)]
fn op_cbac(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 5,l
#[allow(unused_variables)]
fn op_cbad(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 5,(hl)
#[allow(unused_variables)]
fn op_cbae(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 5,a
#[allow(unused_variables)]
fn op_cbaf(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 6,b
#[allow(unused_variables)]
fn op_cbb0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 6,c
#[allow(unused_variables)]
fn op_cbb1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 6,d
#[allow(unused_variables)]
fn op_cbb2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 6,e
#[allow(unused_variables)]
fn op_cbb3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 6,h
#[allow(unused_variables)]
fn op_cbb4(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 6,l
#[allow(unused_variables)]
fn op_cbb5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 6,(hl)
#[allow(unused_variables)]
fn op_cbb6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 6,a
#[allow(unused_variables)]
fn op_cbb7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// res 7,b
#[allow(unused_variables)]
fn op_cbb8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_b();
    cpu.set_b(q & !(1 << p));

    (8, 2)
}

/// res 7,c
#[allow(unused_variables)]
fn op_cbb9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_c();
    cpu.set_c(q & !(1 << p));

    (8, 2)
}

/// res 7,d
#[allow(unused_variables)]
fn op_cbba(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_d();
    cpu.set_d(q & !(1 << p));

    (8, 2)
}

/// res 7,e
#[allow(unused_variables)]
fn op_cbbb(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_e();
    cpu.set_e(q & !(1 << p));

    (8, 2)
}

/// res 7,h
#[allow(unused_variables)]
fn op_cbbc(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_h();
    cpu.set_h(q & !(1 << p));

    (8, 2)
}

/// res 7,l
#[allow(unused_variables)]
fn op_cbbd(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_l();
    cpu.set_l(q & !(1 << p));

    (8, 2)
}

/// res 7,(hl)
#[allow(unused_variables)]
fn op_cbbe(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q & !(1 << p));

    (16, 2)
}

/// res 7,a
#[allow(unused_variables)]
fn op_cbbf(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_a();
    cpu.set_a(q & !(1 << p));

    (8, 2)
}

/// set 0,b
#[allow(unused_variables)]
fn op_cbc0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 0,c
#[allow(unused_variables)]
fn op_cbc1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 0,d
#[allow(unused_variables)]
fn op_cbc2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 0,e
#[allow(unused_variables)]
fn op_cbc3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 0,h
#[allow(unused_variables)]
fn op_cbc4(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 0,l
#[allow(unused_variables)]
fn op_cbc5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 0,(hl)
#[allow(unused_variables)]
fn op_cbc6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 0,a
#[allow(unused_variables)]
fn op_cbc7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 0;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 1,b
#[allow(unused_variables)]
fn op_cbc8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 1,c
#[allow(unused_variables)]
fn op_cbc9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 1,d
#[allow(unused_variables)]
fn op_cbca(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 1,e
#[allow(unused_variables)]
fn op_cbcb(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 1,h
#[allow(unused_variables)]
fn op_cbcc(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 1,l
#[allow(unused_variables)]
fn op_cbcd(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 1,(hl)
#[allow(unused_variables)]
fn op_cbce(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 1,a
#[allow(unused_variables)]
fn op_cbcf(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 1;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 2,b
#[allow(unused_variables)]
fn op_cbd0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 2,c
#[allow(unused_variables)]
fn op_cbd1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 2,d
#[allow(unused_variables)]
fn op_cbd2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 2,e
#[allow(unused_variables)]
fn op_cbd3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 2,h
#[allow(unused_variables)]
fn op_cbd4(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 2,l
#[allow(unused_variables)]
fn op_cbd5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 2,(hl)
#[allow(unused_variables)]
fn op_cbd6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 2,a
#[allow(unused_variables)]
fn op_cbd7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 2;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 3,b
#[allow(unused_variables)]
fn op_cbd8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 3,c
#[allow(unused_variables)]
fn op_cbd9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 3,d
#[allow(unused_variables)]
fn op_cbda(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 3,e
#[allow(unused_variables)]
fn op_cbdb(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 3,h
#[allow(unused_variables)]
fn op_cbdc(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 3,l
#[allow(unused_variables)]
fn op_cbdd(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 3,(hl)
#[allow(unused_variables)]
fn op_cbde(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 3,a
#[allow(unused_variables)]
fn op_cbdf(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 3;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 4,b
#[allow(unused_variables)]
fn op_cbe0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 4,c
#[allow(unused_variables)]
fn op_cbe1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 4,d
#[allow(unused_variables)]
fn op_cbe2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 4,e
#[allow(unused_variables)]
fn op_cbe3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 4,h
#[allow(unused_variables)]
fn op_cbe4(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 4,l
#[allow(unused_variables)]
fn op_cbe5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 4,(hl)
#[allow(unused_variables)]
fn op_cbe6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 4,a
#[allow(unused_variables)]
fn op_cbe7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 4;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 5,b
#[allow(unused_variables)]
fn op_cbe8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 5,c
#[allow(unused_variables)]
fn op_cbe9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 5,d
#[allow(unused_variables)]
fn op_cbea(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 5,e
#[allow(unused_variables)]
fn op_cbeb(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 5,h
#[allow(unused_variables)]
fn op_cbec(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 5,l
#[allow(unused_variables)]
fn op_cbed(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 5,(hl)
#[allow(unused_variables)]
fn op_cbee(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 5,a
#[allow(unused_variables)]
fn op_cbef(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 5;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 6,b
#[allow(unused_variables)]
fn op_cbf0(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 6,c
#[allow(unused_variables)]
fn op_cbf1(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 6,d
#[allow(unused_variables)]
fn op_cbf2(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 6,e
#[allow(unused_variables)]
fn op_cbf3(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 6,h
#[allow(unused_variables)]
fn op_cbf4(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 6,l
#[allow(unused_variables)]
fn op_cbf5(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 6,(hl)
#[allow(unused_variables)]
fn op_cbf6(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 6,a
#[allow(unused_variables)]
fn op_cbf7(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 6;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

/// set 7,b
#[allow(unused_variables)]
fn op_cbf8(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_b();
    cpu.set_b(q | (1 << p));

    (8, 2)
}

/// set 7,c
#[allow(unused_variables)]
fn op_cbf9(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_c();
    cpu.set_c(q | (1 << p));

    (8, 2)
}

/// set 7,d
#[allow(unused_variables)]
fn op_cbfa(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_d();
    cpu.set_d(q | (1 << p));

    (8, 2)
}

/// set 7,e
#[allow(unused_variables)]
fn op_cbfb(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_e();
    cpu.set_e(q | (1 << p));

    (8, 2)
}

/// set 7,h
#[allow(unused_variables)]
fn op_cbfc(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_h();
    cpu.set_h(q | (1 << p));

    (8, 2)
}

/// set 7,l
#[allow(unused_variables)]
fn op_cbfd(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_l();
    cpu.set_l(q | (1 << p));

    (8, 2)
}

/// set 7,(hl)
#[allow(unused_variables)]
fn op_cbfe(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = mmu.get8(cpu.get_hl());
    mmu.set8(cpu.get_hl(), q | (1 << p));

    (16, 2)
}

/// set 7,a
#[allow(unused_variables)]
fn op_cbff(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    let p = 7;
    let q = cpu.get_a();
    cpu.set_a(q | (1 << p));

    (8, 2)
}

pub fn mnem(code: u16) -> &'static str {
    MNEMONICS.get(&code).unwrap_or(&"(unknown opcode)")
}

pub fn decode(code: u16, arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    trace!("{:04x}: {:04x}: {}", cpu.get_pc(), code, mnem(code));

    match code {
        0x0000 => op_0000(arg, cpu, mmu),
        0x0001 => op_0001(arg, cpu, mmu),
        0x0002 => op_0002(arg, cpu, mmu),
        0x0003 => op_0003(arg, cpu, mmu),
        0x0004 => op_0004(arg, cpu, mmu),
        0x0005 => op_0005(arg, cpu, mmu),
        0x0006 => op_0006(arg, cpu, mmu),
        0x0007 => op_0007(arg, cpu, mmu),
        0x0008 => op_0008(arg, cpu, mmu),
        0x0009 => op_0009(arg, cpu, mmu),
        0x000a => op_000a(arg, cpu, mmu),
        0x000b => op_000b(arg, cpu, mmu),
        0x000c => op_000c(arg, cpu, mmu),
        0x000d => op_000d(arg, cpu, mmu),
        0x000e => op_000e(arg, cpu, mmu),
        0x000f => op_000f(arg, cpu, mmu),
        0x0010 => op_0010(arg, cpu, mmu),
        0x0011 => op_0011(arg, cpu, mmu),
        0x0012 => op_0012(arg, cpu, mmu),
        0x0013 => op_0013(arg, cpu, mmu),
        0x0014 => op_0014(arg, cpu, mmu),
        0x0015 => op_0015(arg, cpu, mmu),
        0x0016 => op_0016(arg, cpu, mmu),
        0x0017 => op_0017(arg, cpu, mmu),
        0x0018 => op_0018(arg, cpu, mmu),
        0x0019 => op_0019(arg, cpu, mmu),
        0x001a => op_001a(arg, cpu, mmu),
        0x001b => op_001b(arg, cpu, mmu),
        0x001c => op_001c(arg, cpu, mmu),
        0x001d => op_001d(arg, cpu, mmu),
        0x001e => op_001e(arg, cpu, mmu),
        0x001f => op_001f(arg, cpu, mmu),
        0x0020 => op_0020(arg, cpu, mmu),
        0x0021 => op_0021(arg, cpu, mmu),
        0x0022 => op_0022(arg, cpu, mmu),
        0x0023 => op_0023(arg, cpu, mmu),
        0x0024 => op_0024(arg, cpu, mmu),
        0x0025 => op_0025(arg, cpu, mmu),
        0x0026 => op_0026(arg, cpu, mmu),
        0x0027 => op_0027(arg, cpu, mmu),
        0x0028 => op_0028(arg, cpu, mmu),
        0x0029 => op_0029(arg, cpu, mmu),
        0x002a => op_002a(arg, cpu, mmu),
        0x002b => op_002b(arg, cpu, mmu),
        0x002c => op_002c(arg, cpu, mmu),
        0x002d => op_002d(arg, cpu, mmu),
        0x002e => op_002e(arg, cpu, mmu),
        0x002f => op_002f(arg, cpu, mmu),
        0x0030 => op_0030(arg, cpu, mmu),
        0x0031 => op_0031(arg, cpu, mmu),
        0x0032 => op_0032(arg, cpu, mmu),
        0x0033 => op_0033(arg, cpu, mmu),
        0x0034 => op_0034(arg, cpu, mmu),
        0x0035 => op_0035(arg, cpu, mmu),
        0x0036 => op_0036(arg, cpu, mmu),
        0x0037 => op_0037(arg, cpu, mmu),
        0x0038 => op_0038(arg, cpu, mmu),
        0x0039 => op_0039(arg, cpu, mmu),
        0x003a => op_003a(arg, cpu, mmu),
        0x003b => op_003b(arg, cpu, mmu),
        0x003c => op_003c(arg, cpu, mmu),
        0x003d => op_003d(arg, cpu, mmu),
        0x003e => op_003e(arg, cpu, mmu),
        0x003f => op_003f(arg, cpu, mmu),
        0x0040 => op_0040(arg, cpu, mmu),
        0x0041 => op_0041(arg, cpu, mmu),
        0x0042 => op_0042(arg, cpu, mmu),
        0x0043 => op_0043(arg, cpu, mmu),
        0x0044 => op_0044(arg, cpu, mmu),
        0x0045 => op_0045(arg, cpu, mmu),
        0x0046 => op_0046(arg, cpu, mmu),
        0x0047 => op_0047(arg, cpu, mmu),
        0x0048 => op_0048(arg, cpu, mmu),
        0x0049 => op_0049(arg, cpu, mmu),
        0x004a => op_004a(arg, cpu, mmu),
        0x004b => op_004b(arg, cpu, mmu),
        0x004c => op_004c(arg, cpu, mmu),
        0x004d => op_004d(arg, cpu, mmu),
        0x004e => op_004e(arg, cpu, mmu),
        0x004f => op_004f(arg, cpu, mmu),
        0x0050 => op_0050(arg, cpu, mmu),
        0x0051 => op_0051(arg, cpu, mmu),
        0x0052 => op_0052(arg, cpu, mmu),
        0x0053 => op_0053(arg, cpu, mmu),
        0x0054 => op_0054(arg, cpu, mmu),
        0x0055 => op_0055(arg, cpu, mmu),
        0x0056 => op_0056(arg, cpu, mmu),
        0x0057 => op_0057(arg, cpu, mmu),
        0x0058 => op_0058(arg, cpu, mmu),
        0x0059 => op_0059(arg, cpu, mmu),
        0x005a => op_005a(arg, cpu, mmu),
        0x005b => op_005b(arg, cpu, mmu),
        0x005c => op_005c(arg, cpu, mmu),
        0x005d => op_005d(arg, cpu, mmu),
        0x005e => op_005e(arg, cpu, mmu),
        0x005f => op_005f(arg, cpu, mmu),
        0x0060 => op_0060(arg, cpu, mmu),
        0x0061 => op_0061(arg, cpu, mmu),
        0x0062 => op_0062(arg, cpu, mmu),
        0x0063 => op_0063(arg, cpu, mmu),
        0x0064 => op_0064(arg, cpu, mmu),
        0x0065 => op_0065(arg, cpu, mmu),
        0x0066 => op_0066(arg, cpu, mmu),
        0x0067 => op_0067(arg, cpu, mmu),
        0x0068 => op_0068(arg, cpu, mmu),
        0x0069 => op_0069(arg, cpu, mmu),
        0x006a => op_006a(arg, cpu, mmu),
        0x006b => op_006b(arg, cpu, mmu),
        0x006c => op_006c(arg, cpu, mmu),
        0x006d => op_006d(arg, cpu, mmu),
        0x006e => op_006e(arg, cpu, mmu),
        0x006f => op_006f(arg, cpu, mmu),
        0x0070 => op_0070(arg, cpu, mmu),
        0x0071 => op_0071(arg, cpu, mmu),
        0x0072 => op_0072(arg, cpu, mmu),
        0x0073 => op_0073(arg, cpu, mmu),
        0x0074 => op_0074(arg, cpu, mmu),
        0x0075 => op_0075(arg, cpu, mmu),
        0x0076 => op_0076(arg, cpu, mmu),
        0x0077 => op_0077(arg, cpu, mmu),
        0x0078 => op_0078(arg, cpu, mmu),
        0x0079 => op_0079(arg, cpu, mmu),
        0x007a => op_007a(arg, cpu, mmu),
        0x007b => op_007b(arg, cpu, mmu),
        0x007c => op_007c(arg, cpu, mmu),
        0x007d => op_007d(arg, cpu, mmu),
        0x007e => op_007e(arg, cpu, mmu),
        0x007f => op_007f(arg, cpu, mmu),
        0x0080 => op_0080(arg, cpu, mmu),
        0x0081 => op_0081(arg, cpu, mmu),
        0x0082 => op_0082(arg, cpu, mmu),
        0x0083 => op_0083(arg, cpu, mmu),
        0x0084 => op_0084(arg, cpu, mmu),
        0x0085 => op_0085(arg, cpu, mmu),
        0x0086 => op_0086(arg, cpu, mmu),
        0x0087 => op_0087(arg, cpu, mmu),
        0x0088 => op_0088(arg, cpu, mmu),
        0x0089 => op_0089(arg, cpu, mmu),
        0x008a => op_008a(arg, cpu, mmu),
        0x008b => op_008b(arg, cpu, mmu),
        0x008c => op_008c(arg, cpu, mmu),
        0x008d => op_008d(arg, cpu, mmu),
        0x008e => op_008e(arg, cpu, mmu),
        0x008f => op_008f(arg, cpu, mmu),
        0x0090 => op_0090(arg, cpu, mmu),
        0x0091 => op_0091(arg, cpu, mmu),
        0x0092 => op_0092(arg, cpu, mmu),
        0x0093 => op_0093(arg, cpu, mmu),
        0x0094 => op_0094(arg, cpu, mmu),
        0x0095 => op_0095(arg, cpu, mmu),
        0x0096 => op_0096(arg, cpu, mmu),
        0x0097 => op_0097(arg, cpu, mmu),
        0x0098 => op_0098(arg, cpu, mmu),
        0x0099 => op_0099(arg, cpu, mmu),
        0x009a => op_009a(arg, cpu, mmu),
        0x009b => op_009b(arg, cpu, mmu),
        0x009c => op_009c(arg, cpu, mmu),
        0x009d => op_009d(arg, cpu, mmu),
        0x009e => op_009e(arg, cpu, mmu),
        0x009f => op_009f(arg, cpu, mmu),
        0x00a0 => op_00a0(arg, cpu, mmu),
        0x00a1 => op_00a1(arg, cpu, mmu),
        0x00a2 => op_00a2(arg, cpu, mmu),
        0x00a3 => op_00a3(arg, cpu, mmu),
        0x00a4 => op_00a4(arg, cpu, mmu),
        0x00a5 => op_00a5(arg, cpu, mmu),
        0x00a6 => op_00a6(arg, cpu, mmu),
        0x00a7 => op_00a7(arg, cpu, mmu),
        0x00a8 => op_00a8(arg, cpu, mmu),
        0x00a9 => op_00a9(arg, cpu, mmu),
        0x00aa => op_00aa(arg, cpu, mmu),
        0x00ab => op_00ab(arg, cpu, mmu),
        0x00ac => op_00ac(arg, cpu, mmu),
        0x00ad => op_00ad(arg, cpu, mmu),
        0x00ae => op_00ae(arg, cpu, mmu),
        0x00af => op_00af(arg, cpu, mmu),
        0x00b0 => op_00b0(arg, cpu, mmu),
        0x00b1 => op_00b1(arg, cpu, mmu),
        0x00b2 => op_00b2(arg, cpu, mmu),
        0x00b3 => op_00b3(arg, cpu, mmu),
        0x00b4 => op_00b4(arg, cpu, mmu),
        0x00b5 => op_00b5(arg, cpu, mmu),
        0x00b6 => op_00b6(arg, cpu, mmu),
        0x00b7 => op_00b7(arg, cpu, mmu),
        0x00b8 => op_00b8(arg, cpu, mmu),
        0x00b9 => op_00b9(arg, cpu, mmu),
        0x00ba => op_00ba(arg, cpu, mmu),
        0x00bb => op_00bb(arg, cpu, mmu),
        0x00bc => op_00bc(arg, cpu, mmu),
        0x00bd => op_00bd(arg, cpu, mmu),
        0x00be => op_00be(arg, cpu, mmu),
        0x00bf => op_00bf(arg, cpu, mmu),
        0x00c0 => op_00c0(arg, cpu, mmu),
        0x00c1 => op_00c1(arg, cpu, mmu),
        0x00c2 => op_00c2(arg, cpu, mmu),
        0x00c3 => op_00c3(arg, cpu, mmu),
        0x00c4 => op_00c4(arg, cpu, mmu),
        0x00c5 => op_00c5(arg, cpu, mmu),
        0x00c6 => op_00c6(arg, cpu, mmu),
        0x00c7 => op_00c7(arg, cpu, mmu),
        0x00c8 => op_00c8(arg, cpu, mmu),
        0x00c9 => op_00c9(arg, cpu, mmu),
        0x00ca => op_00ca(arg, cpu, mmu),
        0x00cb => op_00cb(arg, cpu, mmu),
        0x00cc => op_00cc(arg, cpu, mmu),
        0x00cd => op_00cd(arg, cpu, mmu),
        0x00ce => op_00ce(arg, cpu, mmu),
        0x00cf => op_00cf(arg, cpu, mmu),
        0x00d0 => op_00d0(arg, cpu, mmu),
        0x00d1 => op_00d1(arg, cpu, mmu),
        0x00d2 => op_00d2(arg, cpu, mmu),
        0x00d4 => op_00d4(arg, cpu, mmu),
        0x00d5 => op_00d5(arg, cpu, mmu),
        0x00d6 => op_00d6(arg, cpu, mmu),
        0x00d7 => op_00d7(arg, cpu, mmu),
        0x00d8 => op_00d8(arg, cpu, mmu),
        0x00d9 => op_00d9(arg, cpu, mmu),
        0x00da => op_00da(arg, cpu, mmu),
        0x00dc => op_00dc(arg, cpu, mmu),
        0x00de => op_00de(arg, cpu, mmu),
        0x00df => op_00df(arg, cpu, mmu),
        0x00e0 => op_00e0(arg, cpu, mmu),
        0x00e1 => op_00e1(arg, cpu, mmu),
        0x00e2 => op_00e2(arg, cpu, mmu),
        0x00e5 => op_00e5(arg, cpu, mmu),
        0x00e6 => op_00e6(arg, cpu, mmu),
        0x00e7 => op_00e7(arg, cpu, mmu),
        0x00e8 => op_00e8(arg, cpu, mmu),
        0x00e9 => op_00e9(arg, cpu, mmu),
        0x00ea => op_00ea(arg, cpu, mmu),
        0x00ee => op_00ee(arg, cpu, mmu),
        0x00ef => op_00ef(arg, cpu, mmu),
        0x00f0 => op_00f0(arg, cpu, mmu),
        0x00f1 => op_00f1(arg, cpu, mmu),
        0x00f2 => op_00f2(arg, cpu, mmu),
        0x00f3 => op_00f3(arg, cpu, mmu),
        0x00f5 => op_00f5(arg, cpu, mmu),
        0x00f6 => op_00f6(arg, cpu, mmu),
        0x00f7 => op_00f7(arg, cpu, mmu),
        0x00f8 => op_00f8(arg, cpu, mmu),
        0x00f9 => op_00f9(arg, cpu, mmu),
        0x00fa => op_00fa(arg, cpu, mmu),
        0x00fb => op_00fb(arg, cpu, mmu),
        0x00fe => op_00fe(arg, cpu, mmu),
        0x00ff => op_00ff(arg, cpu, mmu),
        0xcb00 => op_cb00(arg, cpu, mmu),
        0xcb01 => op_cb01(arg, cpu, mmu),
        0xcb02 => op_cb02(arg, cpu, mmu),
        0xcb03 => op_cb03(arg, cpu, mmu),
        0xcb04 => op_cb04(arg, cpu, mmu),
        0xcb05 => op_cb05(arg, cpu, mmu),
        0xcb06 => op_cb06(arg, cpu, mmu),
        0xcb07 => op_cb07(arg, cpu, mmu),
        0xcb08 => op_cb08(arg, cpu, mmu),
        0xcb09 => op_cb09(arg, cpu, mmu),
        0xcb0a => op_cb0a(arg, cpu, mmu),
        0xcb0b => op_cb0b(arg, cpu, mmu),
        0xcb0c => op_cb0c(arg, cpu, mmu),
        0xcb0d => op_cb0d(arg, cpu, mmu),
        0xcb0e => op_cb0e(arg, cpu, mmu),
        0xcb0f => op_cb0f(arg, cpu, mmu),
        0xcb10 => op_cb10(arg, cpu, mmu),
        0xcb11 => op_cb11(arg, cpu, mmu),
        0xcb12 => op_cb12(arg, cpu, mmu),
        0xcb13 => op_cb13(arg, cpu, mmu),
        0xcb14 => op_cb14(arg, cpu, mmu),
        0xcb15 => op_cb15(arg, cpu, mmu),
        0xcb16 => op_cb16(arg, cpu, mmu),
        0xcb17 => op_cb17(arg, cpu, mmu),
        0xcb18 => op_cb18(arg, cpu, mmu),
        0xcb19 => op_cb19(arg, cpu, mmu),
        0xcb1a => op_cb1a(arg, cpu, mmu),
        0xcb1b => op_cb1b(arg, cpu, mmu),
        0xcb1c => op_cb1c(arg, cpu, mmu),
        0xcb1d => op_cb1d(arg, cpu, mmu),
        0xcb1e => op_cb1e(arg, cpu, mmu),
        0xcb1f => op_cb1f(arg, cpu, mmu),
        0xcb20 => op_cb20(arg, cpu, mmu),
        0xcb21 => op_cb21(arg, cpu, mmu),
        0xcb22 => op_cb22(arg, cpu, mmu),
        0xcb23 => op_cb23(arg, cpu, mmu),
        0xcb24 => op_cb24(arg, cpu, mmu),
        0xcb25 => op_cb25(arg, cpu, mmu),
        0xcb26 => op_cb26(arg, cpu, mmu),
        0xcb27 => op_cb27(arg, cpu, mmu),
        0xcb28 => op_cb28(arg, cpu, mmu),
        0xcb29 => op_cb29(arg, cpu, mmu),
        0xcb2a => op_cb2a(arg, cpu, mmu),
        0xcb2b => op_cb2b(arg, cpu, mmu),
        0xcb2c => op_cb2c(arg, cpu, mmu),
        0xcb2d => op_cb2d(arg, cpu, mmu),
        0xcb2e => op_cb2e(arg, cpu, mmu),
        0xcb2f => op_cb2f(arg, cpu, mmu),
        0xcb30 => op_cb30(arg, cpu, mmu),
        0xcb31 => op_cb31(arg, cpu, mmu),
        0xcb32 => op_cb32(arg, cpu, mmu),
        0xcb33 => op_cb33(arg, cpu, mmu),
        0xcb34 => op_cb34(arg, cpu, mmu),
        0xcb35 => op_cb35(arg, cpu, mmu),
        0xcb36 => op_cb36(arg, cpu, mmu),
        0xcb37 => op_cb37(arg, cpu, mmu),
        0xcb38 => op_cb38(arg, cpu, mmu),
        0xcb39 => op_cb39(arg, cpu, mmu),
        0xcb3a => op_cb3a(arg, cpu, mmu),
        0xcb3b => op_cb3b(arg, cpu, mmu),
        0xcb3c => op_cb3c(arg, cpu, mmu),
        0xcb3d => op_cb3d(arg, cpu, mmu),
        0xcb3e => op_cb3e(arg, cpu, mmu),
        0xcb3f => op_cb3f(arg, cpu, mmu),
        0xcb40 => op_cb40(arg, cpu, mmu),
        0xcb41 => op_cb41(arg, cpu, mmu),
        0xcb42 => op_cb42(arg, cpu, mmu),
        0xcb43 => op_cb43(arg, cpu, mmu),
        0xcb44 => op_cb44(arg, cpu, mmu),
        0xcb45 => op_cb45(arg, cpu, mmu),
        0xcb46 => op_cb46(arg, cpu, mmu),
        0xcb47 => op_cb47(arg, cpu, mmu),
        0xcb48 => op_cb48(arg, cpu, mmu),
        0xcb49 => op_cb49(arg, cpu, mmu),
        0xcb4a => op_cb4a(arg, cpu, mmu),
        0xcb4b => op_cb4b(arg, cpu, mmu),
        0xcb4c => op_cb4c(arg, cpu, mmu),
        0xcb4d => op_cb4d(arg, cpu, mmu),
        0xcb4e => op_cb4e(arg, cpu, mmu),
        0xcb4f => op_cb4f(arg, cpu, mmu),
        0xcb50 => op_cb50(arg, cpu, mmu),
        0xcb51 => op_cb51(arg, cpu, mmu),
        0xcb52 => op_cb52(arg, cpu, mmu),
        0xcb53 => op_cb53(arg, cpu, mmu),
        0xcb54 => op_cb54(arg, cpu, mmu),
        0xcb55 => op_cb55(arg, cpu, mmu),
        0xcb56 => op_cb56(arg, cpu, mmu),
        0xcb57 => op_cb57(arg, cpu, mmu),
        0xcb58 => op_cb58(arg, cpu, mmu),
        0xcb59 => op_cb59(arg, cpu, mmu),
        0xcb5a => op_cb5a(arg, cpu, mmu),
        0xcb5b => op_cb5b(arg, cpu, mmu),
        0xcb5c => op_cb5c(arg, cpu, mmu),
        0xcb5d => op_cb5d(arg, cpu, mmu),
        0xcb5e => op_cb5e(arg, cpu, mmu),
        0xcb5f => op_cb5f(arg, cpu, mmu),
        0xcb60 => op_cb60(arg, cpu, mmu),
        0xcb61 => op_cb61(arg, cpu, mmu),
        0xcb62 => op_cb62(arg, cpu, mmu),
        0xcb63 => op_cb63(arg, cpu, mmu),
        0xcb64 => op_cb64(arg, cpu, mmu),
        0xcb65 => op_cb65(arg, cpu, mmu),
        0xcb66 => op_cb66(arg, cpu, mmu),
        0xcb67 => op_cb67(arg, cpu, mmu),
        0xcb68 => op_cb68(arg, cpu, mmu),
        0xcb69 => op_cb69(arg, cpu, mmu),
        0xcb6a => op_cb6a(arg, cpu, mmu),
        0xcb6b => op_cb6b(arg, cpu, mmu),
        0xcb6c => op_cb6c(arg, cpu, mmu),
        0xcb6d => op_cb6d(arg, cpu, mmu),
        0xcb6e => op_cb6e(arg, cpu, mmu),
        0xcb6f => op_cb6f(arg, cpu, mmu),
        0xcb70 => op_cb70(arg, cpu, mmu),
        0xcb71 => op_cb71(arg, cpu, mmu),
        0xcb72 => op_cb72(arg, cpu, mmu),
        0xcb73 => op_cb73(arg, cpu, mmu),
        0xcb74 => op_cb74(arg, cpu, mmu),
        0xcb75 => op_cb75(arg, cpu, mmu),
        0xcb76 => op_cb76(arg, cpu, mmu),
        0xcb77 => op_cb77(arg, cpu, mmu),
        0xcb78 => op_cb78(arg, cpu, mmu),
        0xcb79 => op_cb79(arg, cpu, mmu),
        0xcb7a => op_cb7a(arg, cpu, mmu),
        0xcb7b => op_cb7b(arg, cpu, mmu),
        0xcb7c => op_cb7c(arg, cpu, mmu),
        0xcb7d => op_cb7d(arg, cpu, mmu),
        0xcb7e => op_cb7e(arg, cpu, mmu),
        0xcb7f => op_cb7f(arg, cpu, mmu),
        0xcb80 => op_cb80(arg, cpu, mmu),
        0xcb81 => op_cb81(arg, cpu, mmu),
        0xcb82 => op_cb82(arg, cpu, mmu),
        0xcb83 => op_cb83(arg, cpu, mmu),
        0xcb84 => op_cb84(arg, cpu, mmu),
        0xcb85 => op_cb85(arg, cpu, mmu),
        0xcb86 => op_cb86(arg, cpu, mmu),
        0xcb87 => op_cb87(arg, cpu, mmu),
        0xcb88 => op_cb88(arg, cpu, mmu),
        0xcb89 => op_cb89(arg, cpu, mmu),
        0xcb8a => op_cb8a(arg, cpu, mmu),
        0xcb8b => op_cb8b(arg, cpu, mmu),
        0xcb8c => op_cb8c(arg, cpu, mmu),
        0xcb8d => op_cb8d(arg, cpu, mmu),
        0xcb8e => op_cb8e(arg, cpu, mmu),
        0xcb8f => op_cb8f(arg, cpu, mmu),
        0xcb90 => op_cb90(arg, cpu, mmu),
        0xcb91 => op_cb91(arg, cpu, mmu),
        0xcb92 => op_cb92(arg, cpu, mmu),
        0xcb93 => op_cb93(arg, cpu, mmu),
        0xcb94 => op_cb94(arg, cpu, mmu),
        0xcb95 => op_cb95(arg, cpu, mmu),
        0xcb96 => op_cb96(arg, cpu, mmu),
        0xcb97 => op_cb97(arg, cpu, mmu),
        0xcb98 => op_cb98(arg, cpu, mmu),
        0xcb99 => op_cb99(arg, cpu, mmu),
        0xcb9a => op_cb9a(arg, cpu, mmu),
        0xcb9b => op_cb9b(arg, cpu, mmu),
        0xcb9c => op_cb9c(arg, cpu, mmu),
        0xcb9d => op_cb9d(arg, cpu, mmu),
        0xcb9e => op_cb9e(arg, cpu, mmu),
        0xcb9f => op_cb9f(arg, cpu, mmu),
        0xcba0 => op_cba0(arg, cpu, mmu),
        0xcba1 => op_cba1(arg, cpu, mmu),
        0xcba2 => op_cba2(arg, cpu, mmu),
        0xcba3 => op_cba3(arg, cpu, mmu),
        0xcba4 => op_cba4(arg, cpu, mmu),
        0xcba5 => op_cba5(arg, cpu, mmu),
        0xcba6 => op_cba6(arg, cpu, mmu),
        0xcba7 => op_cba7(arg, cpu, mmu),
        0xcba8 => op_cba8(arg, cpu, mmu),
        0xcba9 => op_cba9(arg, cpu, mmu),
        0xcbaa => op_cbaa(arg, cpu, mmu),
        0xcbab => op_cbab(arg, cpu, mmu),
        0xcbac => op_cbac(arg, cpu, mmu),
        0xcbad => op_cbad(arg, cpu, mmu),
        0xcbae => op_cbae(arg, cpu, mmu),
        0xcbaf => op_cbaf(arg, cpu, mmu),
        0xcbb0 => op_cbb0(arg, cpu, mmu),
        0xcbb1 => op_cbb1(arg, cpu, mmu),
        0xcbb2 => op_cbb2(arg, cpu, mmu),
        0xcbb3 => op_cbb3(arg, cpu, mmu),
        0xcbb4 => op_cbb4(arg, cpu, mmu),
        0xcbb5 => op_cbb5(arg, cpu, mmu),
        0xcbb6 => op_cbb6(arg, cpu, mmu),
        0xcbb7 => op_cbb7(arg, cpu, mmu),
        0xcbb8 => op_cbb8(arg, cpu, mmu),
        0xcbb9 => op_cbb9(arg, cpu, mmu),
        0xcbba => op_cbba(arg, cpu, mmu),
        0xcbbb => op_cbbb(arg, cpu, mmu),
        0xcbbc => op_cbbc(arg, cpu, mmu),
        0xcbbd => op_cbbd(arg, cpu, mmu),
        0xcbbe => op_cbbe(arg, cpu, mmu),
        0xcbbf => op_cbbf(arg, cpu, mmu),
        0xcbc0 => op_cbc0(arg, cpu, mmu),
        0xcbc1 => op_cbc1(arg, cpu, mmu),
        0xcbc2 => op_cbc2(arg, cpu, mmu),
        0xcbc3 => op_cbc3(arg, cpu, mmu),
        0xcbc4 => op_cbc4(arg, cpu, mmu),
        0xcbc5 => op_cbc5(arg, cpu, mmu),
        0xcbc6 => op_cbc6(arg, cpu, mmu),
        0xcbc7 => op_cbc7(arg, cpu, mmu),
        0xcbc8 => op_cbc8(arg, cpu, mmu),
        0xcbc9 => op_cbc9(arg, cpu, mmu),
        0xcbca => op_cbca(arg, cpu, mmu),
        0xcbcb => op_cbcb(arg, cpu, mmu),
        0xcbcc => op_cbcc(arg, cpu, mmu),
        0xcbcd => op_cbcd(arg, cpu, mmu),
        0xcbce => op_cbce(arg, cpu, mmu),
        0xcbcf => op_cbcf(arg, cpu, mmu),
        0xcbd0 => op_cbd0(arg, cpu, mmu),
        0xcbd1 => op_cbd1(arg, cpu, mmu),
        0xcbd2 => op_cbd2(arg, cpu, mmu),
        0xcbd3 => op_cbd3(arg, cpu, mmu),
        0xcbd4 => op_cbd4(arg, cpu, mmu),
        0xcbd5 => op_cbd5(arg, cpu, mmu),
        0xcbd6 => op_cbd6(arg, cpu, mmu),
        0xcbd7 => op_cbd7(arg, cpu, mmu),
        0xcbd8 => op_cbd8(arg, cpu, mmu),
        0xcbd9 => op_cbd9(arg, cpu, mmu),
        0xcbda => op_cbda(arg, cpu, mmu),
        0xcbdb => op_cbdb(arg, cpu, mmu),
        0xcbdc => op_cbdc(arg, cpu, mmu),
        0xcbdd => op_cbdd(arg, cpu, mmu),
        0xcbde => op_cbde(arg, cpu, mmu),
        0xcbdf => op_cbdf(arg, cpu, mmu),
        0xcbe0 => op_cbe0(arg, cpu, mmu),
        0xcbe1 => op_cbe1(arg, cpu, mmu),
        0xcbe2 => op_cbe2(arg, cpu, mmu),
        0xcbe3 => op_cbe3(arg, cpu, mmu),
        0xcbe4 => op_cbe4(arg, cpu, mmu),
        0xcbe5 => op_cbe5(arg, cpu, mmu),
        0xcbe6 => op_cbe6(arg, cpu, mmu),
        0xcbe7 => op_cbe7(arg, cpu, mmu),
        0xcbe8 => op_cbe8(arg, cpu, mmu),
        0xcbe9 => op_cbe9(arg, cpu, mmu),
        0xcbea => op_cbea(arg, cpu, mmu),
        0xcbeb => op_cbeb(arg, cpu, mmu),
        0xcbec => op_cbec(arg, cpu, mmu),
        0xcbed => op_cbed(arg, cpu, mmu),
        0xcbee => op_cbee(arg, cpu, mmu),
        0xcbef => op_cbef(arg, cpu, mmu),
        0xcbf0 => op_cbf0(arg, cpu, mmu),
        0xcbf1 => op_cbf1(arg, cpu, mmu),
        0xcbf2 => op_cbf2(arg, cpu, mmu),
        0xcbf3 => op_cbf3(arg, cpu, mmu),
        0xcbf4 => op_cbf4(arg, cpu, mmu),
        0xcbf5 => op_cbf5(arg, cpu, mmu),
        0xcbf6 => op_cbf6(arg, cpu, mmu),
        0xcbf7 => op_cbf7(arg, cpu, mmu),
        0xcbf8 => op_cbf8(arg, cpu, mmu),
        0xcbf9 => op_cbf9(arg, cpu, mmu),
        0xcbfa => op_cbfa(arg, cpu, mmu),
        0xcbfb => op_cbfb(arg, cpu, mmu),
        0xcbfc => op_cbfc(arg, cpu, mmu),
        0xcbfd => op_cbfd(arg, cpu, mmu),
        0xcbfe => op_cbfe(arg, cpu, mmu),
        0xcbff => op_cbff(arg, cpu, mmu),
        _ => panic!("Invalid opcode: {:04x}: {:04x}", cpu.get_pc(), code),
    }
}
