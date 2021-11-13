use crate::alu;
use crate::cpu::{Cpu, Sys};
use hashbrown::HashMap;
use lazy_static::lazy_static;
use log::*;

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

impl<T: Sys> Cpu<T> {
    /// nop
    #[allow(unused_variables)]
    fn op_0000(&mut self, arg: u16) -> (usize, usize) {
        (4, 1)
    }

    /// ld bc,d16
    #[allow(unused_variables)]
    fn op_0001(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get16(self.get_pc().wrapping_add(arg));
        self.set_bc(v);

        (12, 3)
    }

    /// ld (bc),a
    #[allow(unused_variables)]
    fn op_0002(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let x = self.get_bc();
        self.set8(x, v);

        (8, 1)
    }

    /// inc bc
    #[allow(unused_variables)]
    fn op_0003(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_bc().wrapping_add(1);
        self.set_bc(v);
        self.step(4);

        (8, 1)
    }

    /// inc b
    #[allow(unused_variables)]
    fn op_0004(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        let (v, h, c, z) = alu::add8(v, 1, false);
        self.set_b(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);

        (4, 1)
    }

    /// dec b
    #[allow(unused_variables)]
    fn op_0005(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        let (v, h, c, z) = alu::sub8(v, 1, false);
        self.set_b(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);

        (4, 1)
    }

    /// ld b,d8
    #[allow(unused_variables)]
    fn op_0006(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get8(self.get_pc().wrapping_add(arg));
        self.set_b(v);

        (8, 2)
    }

    /// rlca
    #[allow(unused_variables)]
    fn op_0007(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let c = v & 0x80 != 0;
        let v = v.rotate_left(1);
        let z = v == 0;
        self.set_a(v);
        self.set_zf(false);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (4, 1)
    }

    /// ld (a16),sp
    #[allow(unused_variables)]
    fn op_0008(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_sp();
        let x = self.get16(self.get_pc().wrapping_add(arg));
        self.set16(x, v);

        (20, 3)
    }

    /// add hl,bc
    #[allow(unused_variables)]
    fn op_0009(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_hl();
        let q = self.get_bc();
        let (v, h, c, z) = alu::add16(p, q, false);
        self.set_hl(v);
        self.step(4);

        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (8, 1)
    }

    /// ld a,(bc)
    #[allow(unused_variables)]
    fn op_000a(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_bc();
            self.get8(x)
        };
        self.set_a(v);

        (8, 1)
    }

    /// dec bc
    #[allow(unused_variables)]
    fn op_000b(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_bc().wrapping_sub(1);
        self.set_bc(v);
        self.step(4);

        (8, 1)
    }

    /// inc c
    #[allow(unused_variables)]
    fn op_000c(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        let (v, h, c, z) = alu::add8(v, 1, false);
        self.set_c(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);

        (4, 1)
    }

    /// dec c
    #[allow(unused_variables)]
    fn op_000d(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        let (v, h, c, z) = alu::sub8(v, 1, false);
        self.set_c(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);

        (4, 1)
    }

    /// ld c,d8
    #[allow(unused_variables)]
    fn op_000e(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get8(self.get_pc().wrapping_add(arg));
        self.set_c(v);

        (8, 2)
    }

    /// rrca
    #[allow(unused_variables)]
    fn op_000f(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let c = v & 1 != 0;
        let v = v.rotate_right(1);
        let z = v == 0;
        self.set_a(v);
        self.set_zf(false);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (4, 1)
    }

    /// stop 0
    #[allow(unused_variables)]
    fn op_0010(&mut self, arg: u16) -> (usize, usize) {
        self.stop();

        (4, 2)
    }

    /// ld de,d16
    #[allow(unused_variables)]
    fn op_0011(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get16(self.get_pc().wrapping_add(arg));
        self.set_de(v);

        (12, 3)
    }

    /// ld (de),a
    #[allow(unused_variables)]
    fn op_0012(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let x = self.get_de();
        self.set8(x, v);

        (8, 1)
    }

    /// inc de
    #[allow(unused_variables)]
    fn op_0013(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_de().wrapping_add(1);
        self.set_de(v);
        self.step(4);

        (8, 1)
    }

    /// inc d
    #[allow(unused_variables)]
    fn op_0014(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        let (v, h, c, z) = alu::add8(v, 1, false);
        self.set_d(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);

        (4, 1)
    }

    /// dec d
    #[allow(unused_variables)]
    fn op_0015(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        let (v, h, c, z) = alu::sub8(v, 1, false);
        self.set_d(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);

        (4, 1)
    }

    /// ld d,d8
    #[allow(unused_variables)]
    fn op_0016(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get8(self.get_pc().wrapping_add(arg));
        self.set_d(v);

        (8, 2)
    }

    /// rla
    #[allow(unused_variables)]
    fn op_0017(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let v = v | if self.get_cf() { 1 } else { 0 };
        self.set_a(v);
        self.set_zf(false);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (4, 1)
    }

    /// jr r8
    #[allow(unused_variables)]
    fn op_0018(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get8(self.get_pc().wrapping_add(arg));
        let pc = self.get_pc().wrapping_add(alu::signed(p));
        self.jump(pc);

        (12, 2)
    }

    /// add hl,de
    #[allow(unused_variables)]
    fn op_0019(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_hl();
        let q = self.get_de();
        let (v, h, c, z) = alu::add16(p, q, false);
        self.set_hl(v);
        self.step(4);

        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (8, 1)
    }

    /// ld a,(de)
    #[allow(unused_variables)]
    fn op_001a(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_de();
            self.get8(x)
        };
        self.set_a(v);

        (8, 1)
    }

    /// dec de
    #[allow(unused_variables)]
    fn op_001b(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_de().wrapping_sub(1);
        self.set_de(v);
        self.step(4);

        (8, 1)
    }

    /// inc e
    #[allow(unused_variables)]
    fn op_001c(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        let (v, h, c, z) = alu::add8(v, 1, false);
        self.set_e(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);

        (4, 1)
    }

    /// dec e
    #[allow(unused_variables)]
    fn op_001d(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        let (v, h, c, z) = alu::sub8(v, 1, false);
        self.set_e(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);

        (4, 1)
    }

    /// ld e,d8
    #[allow(unused_variables)]
    fn op_001e(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get8(self.get_pc().wrapping_add(arg));
        self.set_e(v);

        (8, 2)
    }

    /// rra
    #[allow(unused_variables)]
    fn op_001f(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let v = v | if self.get_cf() { 0x80 } else { 0 };
        self.set_a(v);
        self.set_zf(false);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (4, 1)
    }

    /// jr nz,r8
    #[allow(unused_variables)]
    fn op_0020(&mut self, arg: u16) -> (usize, usize) {
        let flg = !self.get_zf();
        let p = self.get8(self.get_pc().wrapping_add(arg));
        if flg {
            let pc = self.get_pc().wrapping_add(alu::signed(p));
            self.jump(pc);
            return (12, 2);
        }

        (8, 2)
    }

    /// ld hl,d16
    #[allow(unused_variables)]
    fn op_0021(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get16(self.get_pc().wrapping_add(arg));
        self.set_hl(v);

        (12, 3)
    }

    /// ldi (hl),a
    #[allow(unused_variables)]
    fn op_0022(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let x = self.get_hl();
        self.set8(x, v);

        self.set_hl(self.get_hl().wrapping_add(1));

        (8, 1)
    }

    /// inc hl
    #[allow(unused_variables)]
    fn op_0023(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_hl().wrapping_add(1);
        self.set_hl(v);
        self.step(4);

        (8, 1)
    }

    /// inc h
    #[allow(unused_variables)]
    fn op_0024(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        let (v, h, c, z) = alu::add8(v, 1, false);
        self.set_h(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);

        (4, 1)
    }

    /// dec h
    #[allow(unused_variables)]
    fn op_0025(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        let (v, h, c, z) = alu::sub8(v, 1, false);
        self.set_h(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);

        (4, 1)
    }

    /// ld h,d8
    #[allow(unused_variables)]
    fn op_0026(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get8(self.get_pc().wrapping_add(arg));
        self.set_h(v);

        (8, 2)
    }

    /// daa
    #[allow(unused_variables)]
    fn op_0027(&mut self, arg: u16) -> (usize, usize) {
        let mut adj = 0;

        let v = self.get_a() as usize;

        if self.get_hf() || (!self.get_nf() && (v & 0xf) > 9) {
            adj |= 0x6;
        }

        let c = if self.get_cf() || (!self.get_nf() && v > 0x99) {
            adj |= 0x60;
            true
        } else {
            false
        };

        let v = if self.get_nf() { v - adj } else { v + adj };
        let v = (v & 0xff) as u8;
        let z = v == 0;

        self.set_a(v);
        self.set_zf(z);

        self.set_hf(false);
        self.set_cf(c);

        (4, 1)
    }

    /// jr z,r8
    #[allow(unused_variables)]
    fn op_0028(&mut self, arg: u16) -> (usize, usize) {
        let flg = self.get_zf();
        let p = self.get8(self.get_pc().wrapping_add(arg));
        if flg {
            let pc = self.get_pc().wrapping_add(alu::signed(p));
            self.jump(pc);
            return (12, 2);
        }

        (8, 2)
    }

    /// add hl,hl
    #[allow(unused_variables)]
    fn op_0029(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_hl();
        let q = self.get_hl();
        let (v, h, c, z) = alu::add16(p, q, false);
        self.set_hl(v);
        self.step(4);

        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (8, 1)
    }

    /// ldi a,(hl)
    #[allow(unused_variables)]
    fn op_002a(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        self.set_a(v);

        self.set_hl(self.get_hl().wrapping_add(1));

        (8, 1)
    }

    /// dec hl
    #[allow(unused_variables)]
    fn op_002b(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_hl().wrapping_sub(1);
        self.set_hl(v);
        self.step(4);

        (8, 1)
    }

    /// inc l
    #[allow(unused_variables)]
    fn op_002c(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        let (v, h, c, z) = alu::add8(v, 1, false);
        self.set_l(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);

        (4, 1)
    }

    /// dec l
    #[allow(unused_variables)]
    fn op_002d(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        let (v, h, c, z) = alu::sub8(v, 1, false);
        self.set_l(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);

        (4, 1)
    }

    /// ld l,d8
    #[allow(unused_variables)]
    fn op_002e(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get8(self.get_pc().wrapping_add(arg));
        self.set_l(v);

        (8, 2)
    }

    /// cpl
    #[allow(unused_variables)]
    fn op_002f(&mut self, arg: u16) -> (usize, usize) {
        self.set_a(self.get_a() ^ 0xff);

        self.set_nf(true);
        self.set_hf(true);

        (4, 1)
    }

    /// jr nc,r8
    #[allow(unused_variables)]
    fn op_0030(&mut self, arg: u16) -> (usize, usize) {
        let flg = !self.get_cf();
        let p = self.get8(self.get_pc().wrapping_add(arg));
        if flg {
            let pc = self.get_pc().wrapping_add(alu::signed(p));
            self.jump(pc);
            return (12, 2);
        }

        (8, 2)
    }

    /// ld sp,d16
    #[allow(unused_variables)]
    fn op_0031(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get16(self.get_pc().wrapping_add(arg));
        self.set_sp(v);

        (12, 3)
    }

    /// ldd (hl),a
    #[allow(unused_variables)]
    fn op_0032(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let x = self.get_hl();
        self.set8(x, v);

        self.set_hl(self.get_hl().wrapping_sub(1));

        (8, 1)
    }

    /// inc sp
    #[allow(unused_variables)]
    fn op_0033(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_sp().wrapping_add(1);
        self.set_sp(v);
        self.step(4);

        (8, 1)
    }

    /// inc (hl)
    #[allow(unused_variables)]
    fn op_0034(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        let (v, h, c, z) = alu::add8(v, 1, false);
        let x = self.get_hl();
        self.set8(x, v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);

        (12, 1)
    }

    /// dec (hl)
    #[allow(unused_variables)]
    fn op_0035(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        let (v, h, c, z) = alu::sub8(v, 1, false);
        let x = self.get_hl();
        self.set8(x, v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);

        (12, 1)
    }

    /// ld (hl),d8
    #[allow(unused_variables)]
    fn op_0036(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get8(self.get_pc().wrapping_add(arg));
        let x = self.get_hl();
        self.set8(x, v);

        (12, 2)
    }

    /// scf
    #[allow(unused_variables)]
    fn op_0037(&mut self, arg: u16) -> (usize, usize) {
        self.set_cf(true);

        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(true);

        (4, 1)
    }

    /// jr cf,r8
    #[allow(unused_variables)]
    fn op_0038(&mut self, arg: u16) -> (usize, usize) {
        let flg = self.get_cf();
        let p = self.get8(self.get_pc().wrapping_add(arg));
        if flg {
            let pc = self.get_pc().wrapping_add(alu::signed(p));
            self.jump(pc);
            return (12, 2);
        }

        (8, 2)
    }

    /// add hl,sp
    #[allow(unused_variables)]
    fn op_0039(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_hl();
        let q = self.get_sp();
        let (v, h, c, z) = alu::add16(p, q, false);
        self.set_hl(v);
        self.step(4);

        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (8, 1)
    }

    /// ldd a,(hl)
    #[allow(unused_variables)]
    fn op_003a(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        self.set_a(v);

        self.set_hl(self.get_hl().wrapping_sub(1));

        (8, 1)
    }

    /// dec sp
    #[allow(unused_variables)]
    fn op_003b(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_sp().wrapping_sub(1);
        self.set_sp(v);
        self.step(4);

        (8, 1)
    }

    /// inc a
    #[allow(unused_variables)]
    fn op_003c(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let (v, h, c, z) = alu::add8(v, 1, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);

        (4, 1)
    }

    /// dec a
    #[allow(unused_variables)]
    fn op_003d(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let (v, h, c, z) = alu::sub8(v, 1, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);

        (4, 1)
    }

    /// ld a,d8
    #[allow(unused_variables)]
    fn op_003e(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get8(self.get_pc().wrapping_add(arg));
        self.set_a(v);

        (8, 2)
    }

    /// ccf
    #[allow(unused_variables)]
    fn op_003f(&mut self, arg: u16) -> (usize, usize) {
        let c = !self.get_cf();

        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (4, 1)
    }

    /// ld b,b
    #[allow(unused_variables)]
    fn op_0040(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        self.set_b(v);

        (4, 1)
    }

    /// ld b,c
    #[allow(unused_variables)]
    fn op_0041(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        self.set_b(v);

        (4, 1)
    }

    /// ld b,d
    #[allow(unused_variables)]
    fn op_0042(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        self.set_b(v);

        (4, 1)
    }

    /// ld b,e
    #[allow(unused_variables)]
    fn op_0043(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        self.set_b(v);

        (4, 1)
    }

    /// ld b,h
    #[allow(unused_variables)]
    fn op_0044(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        self.set_b(v);

        (4, 1)
    }

    /// ld b,l
    #[allow(unused_variables)]
    fn op_0045(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        self.set_b(v);

        (4, 1)
    }

    /// ld b,(hl)
    #[allow(unused_variables)]
    fn op_0046(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        self.set_b(v);

        (8, 1)
    }

    /// ld b,a
    #[allow(unused_variables)]
    fn op_0047(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        self.set_b(v);

        (4, 1)
    }

    /// ld c,b
    #[allow(unused_variables)]
    fn op_0048(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        self.set_c(v);

        (4, 1)
    }

    /// ld c,c
    #[allow(unused_variables)]
    fn op_0049(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        self.set_c(v);

        (4, 1)
    }

    /// ld c,d
    #[allow(unused_variables)]
    fn op_004a(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        self.set_c(v);

        (4, 1)
    }

    /// ld c,e
    #[allow(unused_variables)]
    fn op_004b(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        self.set_c(v);

        (4, 1)
    }

    /// ld c,h
    #[allow(unused_variables)]
    fn op_004c(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        self.set_c(v);

        (4, 1)
    }

    /// ld c,l
    #[allow(unused_variables)]
    fn op_004d(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        self.set_c(v);

        (4, 1)
    }

    /// ld c,(hl)
    #[allow(unused_variables)]
    fn op_004e(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        self.set_c(v);

        (8, 1)
    }

    /// ld c,a
    #[allow(unused_variables)]
    fn op_004f(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        self.set_c(v);

        (4, 1)
    }

    /// ld d,b
    #[allow(unused_variables)]
    fn op_0050(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        self.set_d(v);

        (4, 1)
    }

    /// ld d,c
    #[allow(unused_variables)]
    fn op_0051(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        self.set_d(v);

        (4, 1)
    }

    /// ld d,d
    #[allow(unused_variables)]
    fn op_0052(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        self.set_d(v);

        (4, 1)
    }

    /// ld d,e
    #[allow(unused_variables)]
    fn op_0053(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        self.set_d(v);

        (4, 1)
    }

    /// ld d,h
    #[allow(unused_variables)]
    fn op_0054(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        self.set_d(v);

        (4, 1)
    }

    /// ld d,l
    #[allow(unused_variables)]
    fn op_0055(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        self.set_d(v);

        (4, 1)
    }

    /// ld d,(hl)
    #[allow(unused_variables)]
    fn op_0056(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        self.set_d(v);

        (8, 1)
    }

    /// ld d,a
    #[allow(unused_variables)]
    fn op_0057(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        self.set_d(v);

        (4, 1)
    }

    /// ld e,b
    #[allow(unused_variables)]
    fn op_0058(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        self.set_e(v);

        (4, 1)
    }

    /// ld e,c
    #[allow(unused_variables)]
    fn op_0059(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        self.set_e(v);

        (4, 1)
    }

    /// ld e,d
    #[allow(unused_variables)]
    fn op_005a(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        self.set_e(v);

        (4, 1)
    }

    /// ld e,e
    #[allow(unused_variables)]
    fn op_005b(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        self.set_e(v);

        (4, 1)
    }

    /// ld e,h
    #[allow(unused_variables)]
    fn op_005c(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        self.set_e(v);

        (4, 1)
    }

    /// ld e,l
    #[allow(unused_variables)]
    fn op_005d(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        self.set_e(v);

        (4, 1)
    }

    /// ld e,(hl)
    #[allow(unused_variables)]
    fn op_005e(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        self.set_e(v);

        (8, 1)
    }

    /// ld e,a
    #[allow(unused_variables)]
    fn op_005f(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        self.set_e(v);

        (4, 1)
    }

    /// ld h,b
    #[allow(unused_variables)]
    fn op_0060(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        self.set_h(v);

        (4, 1)
    }

    /// ld h,c
    #[allow(unused_variables)]
    fn op_0061(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        self.set_h(v);

        (4, 1)
    }

    /// ld h,d
    #[allow(unused_variables)]
    fn op_0062(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        self.set_h(v);

        (4, 1)
    }

    /// ld h,e
    #[allow(unused_variables)]
    fn op_0063(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        self.set_h(v);

        (4, 1)
    }

    /// ld h,h
    #[allow(unused_variables)]
    fn op_0064(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        self.set_h(v);

        (4, 1)
    }

    /// ld h,l
    #[allow(unused_variables)]
    fn op_0065(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        self.set_h(v);

        (4, 1)
    }

    /// ld h,(hl)
    #[allow(unused_variables)]
    fn op_0066(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        self.set_h(v);

        (8, 1)
    }

    /// ld h,a
    #[allow(unused_variables)]
    fn op_0067(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        self.set_h(v);

        (4, 1)
    }

    /// ld l,b
    #[allow(unused_variables)]
    fn op_0068(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        self.set_l(v);

        (4, 1)
    }

    /// ld l,c
    #[allow(unused_variables)]
    fn op_0069(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        self.set_l(v);

        (4, 1)
    }

    /// ld l,d
    #[allow(unused_variables)]
    fn op_006a(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        self.set_l(v);

        (4, 1)
    }

    /// ld l,e
    #[allow(unused_variables)]
    fn op_006b(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        self.set_l(v);

        (4, 1)
    }

    /// ld l,h
    #[allow(unused_variables)]
    fn op_006c(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        self.set_l(v);

        (4, 1)
    }

    /// ld l,l
    #[allow(unused_variables)]
    fn op_006d(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        self.set_l(v);

        (4, 1)
    }

    /// ld l,(hl)
    #[allow(unused_variables)]
    fn op_006e(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        self.set_l(v);

        (8, 1)
    }

    /// ld l,a
    #[allow(unused_variables)]
    fn op_006f(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        self.set_l(v);

        (4, 1)
    }

    /// ld (hl),b
    #[allow(unused_variables)]
    fn op_0070(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        let x = self.get_hl();
        self.set8(x, v);

        (8, 1)
    }

    /// ld (hl),c
    #[allow(unused_variables)]
    fn op_0071(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        let x = self.get_hl();
        self.set8(x, v);

        (8, 1)
    }

    /// ld (hl),d
    #[allow(unused_variables)]
    fn op_0072(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        let x = self.get_hl();
        self.set8(x, v);

        (8, 1)
    }

    /// ld (hl),e
    #[allow(unused_variables)]
    fn op_0073(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        let x = self.get_hl();
        self.set8(x, v);

        (8, 1)
    }

    /// ld (hl),h
    #[allow(unused_variables)]
    fn op_0074(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        let x = self.get_hl();
        self.set8(x, v);

        (8, 1)
    }

    /// ld (hl),l
    #[allow(unused_variables)]
    fn op_0075(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        let x = self.get_hl();
        self.set8(x, v);

        (8, 1)
    }

    /// halt
    #[allow(unused_variables)]
    fn op_0076(&mut self, arg: u16) -> (usize, usize) {
        self.halt();

        (4, 1)
    }

    /// ld (hl),a
    #[allow(unused_variables)]
    fn op_0077(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let x = self.get_hl();
        self.set8(x, v);

        (8, 1)
    }

    /// ld a,b
    #[allow(unused_variables)]
    fn op_0078(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        self.set_a(v);

        (4, 1)
    }

    /// ld a,c
    #[allow(unused_variables)]
    fn op_0079(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        self.set_a(v);

        (4, 1)
    }

    /// ld a,d
    #[allow(unused_variables)]
    fn op_007a(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        self.set_a(v);

        (4, 1)
    }

    /// ld a,e
    #[allow(unused_variables)]
    fn op_007b(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        self.set_a(v);

        (4, 1)
    }

    /// ld a,h
    #[allow(unused_variables)]
    fn op_007c(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        self.set_a(v);

        (4, 1)
    }

    /// ld a,l
    #[allow(unused_variables)]
    fn op_007d(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        self.set_a(v);

        (4, 1)
    }

    /// ld a,(hl)
    #[allow(unused_variables)]
    fn op_007e(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        self.set_a(v);

        (8, 1)
    }

    /// ld a,a
    #[allow(unused_variables)]
    fn op_007f(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        self.set_a(v);

        (4, 1)
    }

    /// add a,b
    #[allow(unused_variables)]
    fn op_0080(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_b();
        let (v, h, c, z) = alu::add8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// add a,c
    #[allow(unused_variables)]
    fn op_0081(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_c();
        let (v, h, c, z) = alu::add8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// add a,d
    #[allow(unused_variables)]
    fn op_0082(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_d();
        let (v, h, c, z) = alu::add8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// add a,e
    #[allow(unused_variables)]
    fn op_0083(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_e();
        let (v, h, c, z) = alu::add8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// add a,h
    #[allow(unused_variables)]
    fn op_0084(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_h();
        let (v, h, c, z) = alu::add8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// add a,l
    #[allow(unused_variables)]
    fn op_0085(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_l();
        let (v, h, c, z) = alu::add8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// add a,(hl)
    #[allow(unused_variables)]
    fn op_0086(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let (v, h, c, z) = alu::add8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (8, 1)
    }

    /// add a,a
    #[allow(unused_variables)]
    fn op_0087(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_a();
        let (v, h, c, z) = alu::add8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// adc a,b
    #[allow(unused_variables)]
    fn op_0088(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_b();
        let (v, h, c, z) = alu::add8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// adc a,c
    #[allow(unused_variables)]
    fn op_0089(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_c();
        let (v, h, c, z) = alu::add8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// adc a,d
    #[allow(unused_variables)]
    fn op_008a(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_d();
        let (v, h, c, z) = alu::add8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// adc a,e
    #[allow(unused_variables)]
    fn op_008b(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_e();
        let (v, h, c, z) = alu::add8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// adc a,h
    #[allow(unused_variables)]
    fn op_008c(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_h();
        let (v, h, c, z) = alu::add8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// adc a,l
    #[allow(unused_variables)]
    fn op_008d(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_l();
        let (v, h, c, z) = alu::add8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// adc a,(hl)
    #[allow(unused_variables)]
    fn op_008e(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let (v, h, c, z) = alu::add8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (8, 1)
    }

    /// adc a,a
    #[allow(unused_variables)]
    fn op_008f(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_a();
        let (v, h, c, z) = alu::add8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// sub b
    #[allow(unused_variables)]
    fn op_0090(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_b();
        let (v, h, c, z) = alu::sub8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// sub c
    #[allow(unused_variables)]
    fn op_0091(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_c();
        let (v, h, c, z) = alu::sub8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// sub d
    #[allow(unused_variables)]
    fn op_0092(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_d();
        let (v, h, c, z) = alu::sub8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// sub e
    #[allow(unused_variables)]
    fn op_0093(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_e();
        let (v, h, c, z) = alu::sub8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// sub h
    #[allow(unused_variables)]
    fn op_0094(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_h();
        let (v, h, c, z) = alu::sub8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// sub l
    #[allow(unused_variables)]
    fn op_0095(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_l();
        let (v, h, c, z) = alu::sub8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// sub (hl)
    #[allow(unused_variables)]
    fn op_0096(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let (v, h, c, z) = alu::sub8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (8, 1)
    }

    /// sub a
    #[allow(unused_variables)]
    fn op_0097(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_a();
        let (v, h, c, z) = alu::sub8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// sbc a,b
    #[allow(unused_variables)]
    fn op_0098(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_b();
        let (v, h, c, z) = alu::sub8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// sbc a,c
    #[allow(unused_variables)]
    fn op_0099(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_c();
        let (v, h, c, z) = alu::sub8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// sbc a,d
    #[allow(unused_variables)]
    fn op_009a(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_d();
        let (v, h, c, z) = alu::sub8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// sbc a,e
    #[allow(unused_variables)]
    fn op_009b(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_e();
        let (v, h, c, z) = alu::sub8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// sbc a,h
    #[allow(unused_variables)]
    fn op_009c(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_h();
        let (v, h, c, z) = alu::sub8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// sbc a,l
    #[allow(unused_variables)]
    fn op_009d(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_l();
        let (v, h, c, z) = alu::sub8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// sbc a,(hl)
    #[allow(unused_variables)]
    fn op_009e(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let (v, h, c, z) = alu::sub8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (8, 1)
    }

    /// sbc a,a
    #[allow(unused_variables)]
    fn op_009f(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_a();
        let (v, h, c, z) = alu::sub8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// and b
    #[allow(unused_variables)]
    fn op_00a0(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() & self.get_b();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);
        self.set_cf(false);

        (4, 1)
    }

    /// and c
    #[allow(unused_variables)]
    fn op_00a1(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() & self.get_c();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);
        self.set_cf(false);

        (4, 1)
    }

    /// and d
    #[allow(unused_variables)]
    fn op_00a2(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() & self.get_d();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);
        self.set_cf(false);

        (4, 1)
    }

    /// and e
    #[allow(unused_variables)]
    fn op_00a3(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() & self.get_e();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);
        self.set_cf(false);

        (4, 1)
    }

    /// and h
    #[allow(unused_variables)]
    fn op_00a4(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() & self.get_h();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);
        self.set_cf(false);

        (4, 1)
    }

    /// and l
    #[allow(unused_variables)]
    fn op_00a5(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() & self.get_l();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);
        self.set_cf(false);

        (4, 1)
    }

    /// and (hl)
    #[allow(unused_variables)]
    fn op_00a6(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() & {
            let x = self.get_hl();
            self.get8(x)
        };
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);
        self.set_cf(false);

        (8, 1)
    }

    /// and a
    #[allow(unused_variables)]
    fn op_00a7(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() & self.get_a();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);
        self.set_cf(false);

        (4, 1)
    }

    /// xor b
    #[allow(unused_variables)]
    fn op_00a8(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() ^ self.get_b();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (4, 1)
    }

    /// xor c
    #[allow(unused_variables)]
    fn op_00a9(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() ^ self.get_c();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (4, 1)
    }

    /// xor d
    #[allow(unused_variables)]
    fn op_00aa(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() ^ self.get_d();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (4, 1)
    }

    /// xor e
    #[allow(unused_variables)]
    fn op_00ab(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() ^ self.get_e();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (4, 1)
    }

    /// xor h
    #[allow(unused_variables)]
    fn op_00ac(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() ^ self.get_h();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (4, 1)
    }

    /// xor l
    #[allow(unused_variables)]
    fn op_00ad(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() ^ self.get_l();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (4, 1)
    }

    /// xor (hl)
    #[allow(unused_variables)]
    fn op_00ae(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() ^ {
            let x = self.get_hl();
            self.get8(x)
        };
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (8, 1)
    }

    /// xor a
    #[allow(unused_variables)]
    fn op_00af(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() ^ self.get_a();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (4, 1)
    }

    /// or b
    #[allow(unused_variables)]
    fn op_00b0(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() | self.get_b();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (4, 1)
    }

    /// or c
    #[allow(unused_variables)]
    fn op_00b1(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() | self.get_c();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (4, 1)
    }

    /// or d
    #[allow(unused_variables)]
    fn op_00b2(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() | self.get_d();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (4, 1)
    }

    /// or e
    #[allow(unused_variables)]
    fn op_00b3(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() | self.get_e();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (4, 1)
    }

    /// or h
    #[allow(unused_variables)]
    fn op_00b4(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() | self.get_h();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (4, 1)
    }

    /// or l
    #[allow(unused_variables)]
    fn op_00b5(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() | self.get_l();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (4, 1)
    }

    /// or (hl)
    #[allow(unused_variables)]
    fn op_00b6(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() | {
            let x = self.get_hl();
            self.get8(x)
        };
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (8, 1)
    }

    /// or a
    #[allow(unused_variables)]
    fn op_00b7(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() | self.get_a();
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (4, 1)
    }

    /// cp b
    #[allow(unused_variables)]
    fn op_00b8(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_b();
        let (_, h, c, z) = alu::sub8(p, q, false);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// cp c
    #[allow(unused_variables)]
    fn op_00b9(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_c();
        let (_, h, c, z) = alu::sub8(p, q, false);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// cp d
    #[allow(unused_variables)]
    fn op_00ba(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_d();
        let (_, h, c, z) = alu::sub8(p, q, false);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// cp e
    #[allow(unused_variables)]
    fn op_00bb(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_e();
        let (_, h, c, z) = alu::sub8(p, q, false);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// cp h
    #[allow(unused_variables)]
    fn op_00bc(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_h();
        let (_, h, c, z) = alu::sub8(p, q, false);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// cp l
    #[allow(unused_variables)]
    fn op_00bd(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_l();
        let (_, h, c, z) = alu::sub8(p, q, false);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// cp (hl)
    #[allow(unused_variables)]
    fn op_00be(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let (_, h, c, z) = alu::sub8(p, q, false);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (8, 1)
    }

    /// cp a
    #[allow(unused_variables)]
    fn op_00bf(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get_a();
        let (_, h, c, z) = alu::sub8(p, q, false);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (4, 1)
    }

    /// ret nz
    #[allow(unused_variables)]
    fn op_00c0(&mut self, arg: u16) -> (usize, usize) {
        let flg = !self.get_zf();
        self.step(4);
        if flg {
            let pc = self.pop();
            self.jump(pc);
            return (20, 0);
        }

        (8, 1)
    }

    /// pop bc
    #[allow(unused_variables)]
    fn op_00c1(&mut self, arg: u16) -> (usize, usize) {
        let v = self.pop();
        self.set_bc(v);

        (12, 1)
    }

    /// jp nz,a16
    #[allow(unused_variables)]
    fn op_00c2(&mut self, arg: u16) -> (usize, usize) {
        let flg = !self.get_zf();
        let pc = self.get16(self.get_pc().wrapping_add(arg));
        if flg {
            self.jump(pc);
            return (16, 0);
        }

        (12, 3)
    }

    /// jp a16
    #[allow(unused_variables)]
    fn op_00c3(&mut self, arg: u16) -> (usize, usize) {
        let pc = self.get16(self.get_pc().wrapping_add(arg));

        self.jump(pc.wrapping_sub(3));

        (16, 3)
    }

    /// call nz,a16
    #[allow(unused_variables)]
    fn op_00c4(&mut self, arg: u16) -> (usize, usize) {
        let flg = !self.get_zf();
        let pc = self.get16(self.get_pc().wrapping_add(arg));
        if flg {
            self.push(self.get_pc().wrapping_add(3));
            self.jump(pc);
            return (24, 0);
        }

        (12, 3)
    }

    /// push bc
    #[allow(unused_variables)]
    fn op_00c5(&mut self, arg: u16) -> (usize, usize) {
        self.push(self.get_bc());
        self.step(4);

        (16, 1)
    }

    /// add a,d8
    #[allow(unused_variables)]
    fn op_00c6(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get8(self.get_pc().wrapping_add(arg));
        let (v, h, c, z) = alu::add8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (8, 2)
    }

    /// rst 0x00
    #[allow(unused_variables)]
    fn op_00c7(&mut self, arg: u16) -> (usize, usize) {
        self.push(self.get_pc().wrapping_add(1));
        let pc = 0x00u16.wrapping_sub(1);
        self.jump(pc);

        (16, 1)
    }

    /// ret z
    #[allow(unused_variables)]
    fn op_00c8(&mut self, arg: u16) -> (usize, usize) {
        let flg = self.get_zf();
        self.step(4);
        if flg {
            let pc = self.pop();
            self.jump(pc);
            return (20, 0);
        }

        (8, 1)
    }

    /// ret
    #[allow(unused_variables)]
    fn op_00c9(&mut self, arg: u16) -> (usize, usize) {
        let pc = self.pop().wrapping_sub(1);
        self.jump(pc);

        (16, 1)
    }

    /// jp z,a16
    #[allow(unused_variables)]
    fn op_00ca(&mut self, arg: u16) -> (usize, usize) {
        let flg = self.get_zf();
        let pc = self.get16(self.get_pc().wrapping_add(arg));
        if flg {
            self.jump(pc);
            return (16, 0);
        }

        (12, 3)
    }

    /// prefix cb
    #[allow(unused_variables)]
    fn op_00cb(&mut self, arg: u16) -> (usize, usize) {
        (4, 1)
    }

    /// call z,a16
    #[allow(unused_variables)]
    fn op_00cc(&mut self, arg: u16) -> (usize, usize) {
        let flg = self.get_zf();
        let pc = self.get16(self.get_pc().wrapping_add(arg));
        if flg {
            self.push(self.get_pc().wrapping_add(3));
            self.jump(pc);
            return (24, 0);
        }

        (12, 3)
    }

    /// call a16
    #[allow(unused_variables)]
    fn op_00cd(&mut self, arg: u16) -> (usize, usize) {
        self.push(self.get_pc().wrapping_add(3));
        let pc = self.get16(self.get_pc().wrapping_add(arg)).wrapping_sub(3);
        self.jump(pc);

        (24, 3)
    }

    /// adc a,d8
    #[allow(unused_variables)]
    fn op_00ce(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get8(self.get_pc().wrapping_add(arg));
        let (v, h, c, z) = alu::add8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (8, 2)
    }

    /// rst 0x08
    #[allow(unused_variables)]
    fn op_00cf(&mut self, arg: u16) -> (usize, usize) {
        self.push(self.get_pc().wrapping_add(1));
        let pc = 0x08u16.wrapping_sub(1);
        self.jump(pc);

        (16, 1)
    }

    /// ret nc
    #[allow(unused_variables)]
    fn op_00d0(&mut self, arg: u16) -> (usize, usize) {
        let flg = !self.get_cf();
        self.step(4);
        if flg {
            let pc = self.pop();
            self.jump(pc);
            return (20, 0);
        }

        (8, 1)
    }

    /// pop de
    #[allow(unused_variables)]
    fn op_00d1(&mut self, arg: u16) -> (usize, usize) {
        let v = self.pop();
        self.set_de(v);

        (12, 1)
    }

    /// jp nc,a16
    #[allow(unused_variables)]
    fn op_00d2(&mut self, arg: u16) -> (usize, usize) {
        let flg = !self.get_cf();
        let pc = self.get16(self.get_pc().wrapping_add(arg));
        if flg {
            self.jump(pc);
            return (16, 0);
        }

        (12, 3)
    }

    /// call nc,a16
    #[allow(unused_variables)]
    fn op_00d4(&mut self, arg: u16) -> (usize, usize) {
        let flg = !self.get_cf();
        let pc = self.get16(self.get_pc().wrapping_add(arg));
        if flg {
            self.push(self.get_pc().wrapping_add(3));
            self.jump(pc);
            return (24, 0);
        }

        (12, 3)
    }

    /// push de
    #[allow(unused_variables)]
    fn op_00d5(&mut self, arg: u16) -> (usize, usize) {
        self.push(self.get_de());
        self.step(4);

        (16, 1)
    }

    /// sub d8
    #[allow(unused_variables)]
    fn op_00d6(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get8(self.get_pc().wrapping_add(arg));
        let (v, h, c, z) = alu::sub8(p, q, false);
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (8, 2)
    }

    /// rst 0x10
    #[allow(unused_variables)]
    fn op_00d7(&mut self, arg: u16) -> (usize, usize) {
        self.push(self.get_pc().wrapping_add(1));
        let pc = 0x10u16.wrapping_sub(1);
        self.jump(pc);

        (16, 1)
    }

    /// ret cf
    #[allow(unused_variables)]
    fn op_00d8(&mut self, arg: u16) -> (usize, usize) {
        let flg = self.get_cf();
        self.step(4);
        if flg {
            let pc = self.pop();
            self.jump(pc);
            return (20, 0);
        }

        (8, 1)
    }

    /// reti
    #[allow(unused_variables)]
    fn op_00d9(&mut self, arg: u16) -> (usize, usize) {
        let pc = self.pop().wrapping_sub(1);
        self.jump(pc);
        self.enable_interrupt();

        (16, 1)
    }

    /// jp cf,a16
    #[allow(unused_variables)]
    fn op_00da(&mut self, arg: u16) -> (usize, usize) {
        let flg = self.get_cf();
        let pc = self.get16(self.get_pc().wrapping_add(arg));
        if flg {
            self.jump(pc);
            return (16, 0);
        }

        (12, 3)
    }

    /// call cf,a16
    #[allow(unused_variables)]
    fn op_00dc(&mut self, arg: u16) -> (usize, usize) {
        let flg = self.get_cf();
        let pc = self.get16(self.get_pc().wrapping_add(arg));
        if flg {
            self.push(self.get_pc().wrapping_add(3));
            self.jump(pc);
            return (24, 0);
        }

        (12, 3)
    }

    /// sbc a,d8
    #[allow(unused_variables)]
    fn op_00de(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get8(self.get_pc().wrapping_add(arg));
        let (v, h, c, z) = alu::sub8(p, q, self.get_cf());
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (8, 2)
    }

    /// rst 0x18
    #[allow(unused_variables)]
    fn op_00df(&mut self, arg: u16) -> (usize, usize) {
        self.push(self.get_pc().wrapping_add(1));
        let pc = 0x18u16.wrapping_sub(1);
        self.jump(pc);

        (16, 1)
    }

    /// ld (0xff00+a8),a
    #[allow(unused_variables)]
    fn op_00e0(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let x = 0xff00 + self.get8(self.get_pc().wrapping_add(arg)) as u16;
        self.set8(x, v);

        (12, 2)
    }

    /// pop hl
    #[allow(unused_variables)]
    fn op_00e1(&mut self, arg: u16) -> (usize, usize) {
        let v = self.pop();
        self.set_hl(v);

        (12, 1)
    }

    /// ld (0xff00+c),a
    #[allow(unused_variables)]
    fn op_00e2(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let x = 0xff00 + self.get_c() as u16;
        self.set8(x, v);

        (8, 1)
    }

    /// push hl
    #[allow(unused_variables)]
    fn op_00e5(&mut self, arg: u16) -> (usize, usize) {
        self.push(self.get_hl());
        self.step(4);

        (16, 1)
    }

    /// and d8
    #[allow(unused_variables)]
    fn op_00e6(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() & self.get8(self.get_pc().wrapping_add(arg));
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);
        self.set_cf(false);

        (8, 2)
    }

    /// rst 0x20
    #[allow(unused_variables)]
    fn op_00e7(&mut self, arg: u16) -> (usize, usize) {
        self.push(self.get_pc().wrapping_add(1));
        let pc = 0x20u16.wrapping_sub(1);
        self.jump(pc);

        (16, 1)
    }

    /// add sp,r8
    #[allow(unused_variables)]
    fn op_00e8(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_sp();
        let q = self.get8(self.get_pc().wrapping_add(arg));
        let (v, h, c, z) = alu::add16e(p, q, false);
        self.set_sp(v);
        self.step(8);
        self.set_zf(false);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (16, 2)
    }

    /// jp hl
    #[allow(unused_variables)]
    fn op_00e9(&mut self, arg: u16) -> (usize, usize) {
        let pc = self.get_hl();

        self.set_pc(pc.wrapping_sub(1));

        (4, 1)
    }

    /// ld (a16),a
    #[allow(unused_variables)]
    fn op_00ea(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let x = self.get16(self.get_pc().wrapping_add(arg));
        self.set8(x, v);

        (16, 3)
    }

    /// xor d8
    #[allow(unused_variables)]
    fn op_00ee(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() ^ self.get8(self.get_pc().wrapping_add(arg));
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (8, 2)
    }

    /// rst 0x28
    #[allow(unused_variables)]
    fn op_00ef(&mut self, arg: u16) -> (usize, usize) {
        self.push(self.get_pc().wrapping_add(1));
        let pc = 0x28u16.wrapping_sub(1);
        self.jump(pc);

        (16, 1)
    }

    /// ld a,(0xff00+a8)
    #[allow(unused_variables)]
    fn op_00f0(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = 0xff00 + self.get8(self.get_pc().wrapping_add(arg)) as u16;
            self.get8(x)
        };
        self.set_a(v);

        (12, 2)
    }

    /// pop af
    #[allow(unused_variables)]
    fn op_00f1(&mut self, arg: u16) -> (usize, usize) {
        let v = self.pop();
        self.set_af(v);

        (12, 1)
    }

    /// ld a,(0xff00+c)
    #[allow(unused_variables)]
    fn op_00f2(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = 0xff00 + self.get_c() as u16;
            self.get8(x)
        };
        self.set_a(v);

        (8, 1)
    }

    /// di
    #[allow(unused_variables)]
    fn op_00f3(&mut self, arg: u16) -> (usize, usize) {
        self.disable_interrupt();

        (4, 1)
    }

    /// push af
    #[allow(unused_variables)]
    fn op_00f5(&mut self, arg: u16) -> (usize, usize) {
        self.push(self.get_af());
        self.step(4);

        (16, 1)
    }

    /// or d8
    #[allow(unused_variables)]
    fn op_00f6(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a() | self.get8(self.get_pc().wrapping_add(arg));
        self.set_a(v);
        let z = self.get_a() == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (8, 2)
    }

    /// rst 0x30
    #[allow(unused_variables)]
    fn op_00f7(&mut self, arg: u16) -> (usize, usize) {
        self.push(self.get_pc().wrapping_add(1));
        let pc = 0x30u16.wrapping_sub(1);
        self.jump(pc);

        (16, 1)
    }

    /// ldhl sp,r8
    #[allow(unused_variables)]
    fn op_00f8(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_sp();
        let q = self.get8(self.get_pc().wrapping_add(arg));
        let (v, h, c, z) = alu::add16e(p, q, false);
        self.set_hl(v);
        self.step(4);
        self.set_zf(false);
        self.set_nf(false);
        self.set_hf(h);
        self.set_cf(c);

        (12, 2)
    }

    /// ld sp,hl
    #[allow(unused_variables)]
    fn op_00f9(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_hl();
        self.set_sp(v);

        self.step(4);

        (8, 1)
    }

    /// ld a,(a16)
    #[allow(unused_variables)]
    fn op_00fa(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get16(self.get_pc().wrapping_add(arg));
            self.get8(x)
        };
        self.set_a(v);

        (16, 3)
    }

    /// ei
    #[allow(unused_variables)]
    fn op_00fb(&mut self, arg: u16) -> (usize, usize) {
        self.enable_interrupt();

        (4, 1)
    }

    /// cp d8
    #[allow(unused_variables)]
    fn op_00fe(&mut self, arg: u16) -> (usize, usize) {
        let p = self.get_a();
        let q = self.get8(self.get_pc().wrapping_add(arg));
        let (_, h, c, z) = alu::sub8(p, q, false);
        self.set_zf(z);
        self.set_nf(true);
        self.set_hf(h);
        self.set_cf(c);

        (8, 2)
    }

    /// rst 0x38
    #[allow(unused_variables)]
    fn op_00ff(&mut self, arg: u16) -> (usize, usize) {
        self.push(self.get_pc().wrapping_add(1));
        let pc = 0x38u16.wrapping_sub(1);
        self.jump(pc);

        (16, 1)
    }

    /// rlc b
    #[allow(unused_variables)]
    fn op_cb00(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        let c = v & 0x80 != 0;
        let v = v.rotate_left(1);
        let z = v == 0;
        self.set_b(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rlc c
    #[allow(unused_variables)]
    fn op_cb01(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        let c = v & 0x80 != 0;
        let v = v.rotate_left(1);
        let z = v == 0;
        self.set_c(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rlc d
    #[allow(unused_variables)]
    fn op_cb02(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        let c = v & 0x80 != 0;
        let v = v.rotate_left(1);
        let z = v == 0;
        self.set_d(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rlc e
    #[allow(unused_variables)]
    fn op_cb03(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        let c = v & 0x80 != 0;
        let v = v.rotate_left(1);
        let z = v == 0;
        self.set_e(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rlc h
    #[allow(unused_variables)]
    fn op_cb04(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        let c = v & 0x80 != 0;
        let v = v.rotate_left(1);
        let z = v == 0;
        self.set_h(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rlc l
    #[allow(unused_variables)]
    fn op_cb05(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        let c = v & 0x80 != 0;
        let v = v.rotate_left(1);
        let z = v == 0;
        self.set_l(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rlc (hl)
    #[allow(unused_variables)]
    fn op_cb06(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        let c = v & 0x80 != 0;
        let v = v.rotate_left(1);
        let z = v == 0;
        let x = self.get_hl();
        self.set8(x, v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (16, 2)
    }

    /// rlc a
    #[allow(unused_variables)]
    fn op_cb07(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let c = v & 0x80 != 0;
        let v = v.rotate_left(1);
        let z = v == 0;
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rrc b
    #[allow(unused_variables)]
    fn op_cb08(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        let c = v & 1 != 0;
        let v = v.rotate_right(1);
        let z = v == 0;
        self.set_b(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rrc c
    #[allow(unused_variables)]
    fn op_cb09(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        let c = v & 1 != 0;
        let v = v.rotate_right(1);
        let z = v == 0;
        self.set_c(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rrc d
    #[allow(unused_variables)]
    fn op_cb0a(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        let c = v & 1 != 0;
        let v = v.rotate_right(1);
        let z = v == 0;
        self.set_d(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rrc e
    #[allow(unused_variables)]
    fn op_cb0b(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        let c = v & 1 != 0;
        let v = v.rotate_right(1);
        let z = v == 0;
        self.set_e(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rrc h
    #[allow(unused_variables)]
    fn op_cb0c(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        let c = v & 1 != 0;
        let v = v.rotate_right(1);
        let z = v == 0;
        self.set_h(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rrc l
    #[allow(unused_variables)]
    fn op_cb0d(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        let c = v & 1 != 0;
        let v = v.rotate_right(1);
        let z = v == 0;
        self.set_l(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rrc (hl)
    #[allow(unused_variables)]
    fn op_cb0e(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        let c = v & 1 != 0;
        let v = v.rotate_right(1);
        let z = v == 0;
        let x = self.get_hl();
        self.set8(x, v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (16, 2)
    }

    /// rrc a
    #[allow(unused_variables)]
    fn op_cb0f(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let c = v & 1 != 0;
        let v = v.rotate_right(1);
        let z = v == 0;
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rl b
    #[allow(unused_variables)]
    fn op_cb10(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let v = v | if self.get_cf() { 1 } else { 0 };
        let z = v == 0;
        self.set_b(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rl c
    #[allow(unused_variables)]
    fn op_cb11(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let v = v | if self.get_cf() { 1 } else { 0 };
        let z = v == 0;
        self.set_c(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rl d
    #[allow(unused_variables)]
    fn op_cb12(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let v = v | if self.get_cf() { 1 } else { 0 };
        let z = v == 0;
        self.set_d(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rl e
    #[allow(unused_variables)]
    fn op_cb13(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let v = v | if self.get_cf() { 1 } else { 0 };
        let z = v == 0;
        self.set_e(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rl h
    #[allow(unused_variables)]
    fn op_cb14(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let v = v | if self.get_cf() { 1 } else { 0 };
        let z = v == 0;
        self.set_h(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rl l
    #[allow(unused_variables)]
    fn op_cb15(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let v = v | if self.get_cf() { 1 } else { 0 };
        let z = v == 0;
        self.set_l(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rl (hl)
    #[allow(unused_variables)]
    fn op_cb16(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let v = v | if self.get_cf() { 1 } else { 0 };
        let z = v == 0;
        let x = self.get_hl();
        self.set8(x, v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (16, 2)
    }

    /// rl a
    #[allow(unused_variables)]
    fn op_cb17(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let v = v | if self.get_cf() { 1 } else { 0 };
        let z = v == 0;
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rr b
    #[allow(unused_variables)]
    fn op_cb18(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let v = v | if self.get_cf() { 0x80 } else { 0 };
        let z = v == 0;
        self.set_b(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rr c
    #[allow(unused_variables)]
    fn op_cb19(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let v = v | if self.get_cf() { 0x80 } else { 0 };
        let z = v == 0;
        self.set_c(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rr d
    #[allow(unused_variables)]
    fn op_cb1a(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let v = v | if self.get_cf() { 0x80 } else { 0 };
        let z = v == 0;
        self.set_d(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rr e
    #[allow(unused_variables)]
    fn op_cb1b(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let v = v | if self.get_cf() { 0x80 } else { 0 };
        let z = v == 0;
        self.set_e(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rr h
    #[allow(unused_variables)]
    fn op_cb1c(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let v = v | if self.get_cf() { 0x80 } else { 0 };
        let z = v == 0;
        self.set_h(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rr l
    #[allow(unused_variables)]
    fn op_cb1d(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let v = v | if self.get_cf() { 0x80 } else { 0 };
        let z = v == 0;
        self.set_l(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// rr (hl)
    #[allow(unused_variables)]
    fn op_cb1e(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let v = v | if self.get_cf() { 0x80 } else { 0 };
        let z = v == 0;
        let x = self.get_hl();
        self.set8(x, v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (16, 2)
    }

    /// rr a
    #[allow(unused_variables)]
    fn op_cb1f(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let v = v | if self.get_cf() { 0x80 } else { 0 };
        let z = v == 0;
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// sla b
    #[allow(unused_variables)]
    fn op_cb20(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let z = v == 0;
        self.set_b(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// sla c
    #[allow(unused_variables)]
    fn op_cb21(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let z = v == 0;
        self.set_c(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// sla d
    #[allow(unused_variables)]
    fn op_cb22(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let z = v == 0;
        self.set_d(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// sla e
    #[allow(unused_variables)]
    fn op_cb23(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let z = v == 0;
        self.set_e(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// sla h
    #[allow(unused_variables)]
    fn op_cb24(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let z = v == 0;
        self.set_h(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// sla l
    #[allow(unused_variables)]
    fn op_cb25(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let z = v == 0;
        self.set_l(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// sla (hl)
    #[allow(unused_variables)]
    fn op_cb26(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let z = v == 0;
        let x = self.get_hl();
        self.set8(x, v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (16, 2)
    }

    /// sla a
    #[allow(unused_variables)]
    fn op_cb27(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let c = v & 0x80 != 0;
        let v = v.wrapping_shl(1);
        let z = v == 0;
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// sra b
    #[allow(unused_variables)]
    fn op_cb28(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        let c = v & 1 != 0;
        let msb = v & 0x80;
        let v = v.wrapping_shr(1);
        let v = v | msb;
        let z = v == 0;
        self.set_b(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// sra c
    #[allow(unused_variables)]
    fn op_cb29(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        let c = v & 1 != 0;
        let msb = v & 0x80;
        let v = v.wrapping_shr(1);
        let v = v | msb;
        let z = v == 0;
        self.set_c(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// sra d
    #[allow(unused_variables)]
    fn op_cb2a(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        let c = v & 1 != 0;
        let msb = v & 0x80;
        let v = v.wrapping_shr(1);
        let v = v | msb;
        let z = v == 0;
        self.set_d(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// sra e
    #[allow(unused_variables)]
    fn op_cb2b(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        let c = v & 1 != 0;
        let msb = v & 0x80;
        let v = v.wrapping_shr(1);
        let v = v | msb;
        let z = v == 0;
        self.set_e(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// sra h
    #[allow(unused_variables)]
    fn op_cb2c(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        let c = v & 1 != 0;
        let msb = v & 0x80;
        let v = v.wrapping_shr(1);
        let v = v | msb;
        let z = v == 0;
        self.set_h(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// sra l
    #[allow(unused_variables)]
    fn op_cb2d(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        let c = v & 1 != 0;
        let msb = v & 0x80;
        let v = v.wrapping_shr(1);
        let v = v | msb;
        let z = v == 0;
        self.set_l(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// sra (hl)
    #[allow(unused_variables)]
    fn op_cb2e(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        let c = v & 1 != 0;
        let msb = v & 0x80;
        let v = v.wrapping_shr(1);
        let v = v | msb;
        let z = v == 0;
        let x = self.get_hl();
        self.set8(x, v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (16, 2)
    }

    /// sra a
    #[allow(unused_variables)]
    fn op_cb2f(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let c = v & 1 != 0;
        let msb = v & 0x80;
        let v = v.wrapping_shr(1);
        let v = v | msb;
        let z = v == 0;
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// swap b
    #[allow(unused_variables)]
    fn op_cb30(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        let v = v.rotate_left(4);
        self.set_b(v);
        let z = v == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (8, 2)
    }

    /// swap c
    #[allow(unused_variables)]
    fn op_cb31(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        let v = v.rotate_left(4);
        self.set_c(v);
        let z = v == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (8, 2)
    }

    /// swap d
    #[allow(unused_variables)]
    fn op_cb32(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        let v = v.rotate_left(4);
        self.set_d(v);
        let z = v == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (8, 2)
    }

    /// swap e
    #[allow(unused_variables)]
    fn op_cb33(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        let v = v.rotate_left(4);
        self.set_e(v);
        let z = v == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (8, 2)
    }

    /// swap h
    #[allow(unused_variables)]
    fn op_cb34(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        let v = v.rotate_left(4);
        self.set_h(v);
        let z = v == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (8, 2)
    }

    /// swap l
    #[allow(unused_variables)]
    fn op_cb35(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        let v = v.rotate_left(4);
        self.set_l(v);
        let z = v == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (8, 2)
    }

    /// swap (hl)
    #[allow(unused_variables)]
    fn op_cb36(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        let v = v.rotate_left(4);
        let x = self.get_hl();
        self.set8(x, v);
        let z = v == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (16, 2)
    }

    /// swap a
    #[allow(unused_variables)]
    fn op_cb37(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let v = v.rotate_left(4);
        self.set_a(v);
        let z = v == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(false);

        (8, 2)
    }

    /// srl b
    #[allow(unused_variables)]
    fn op_cb38(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_b();
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let z = v == 0;
        self.set_b(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// srl c
    #[allow(unused_variables)]
    fn op_cb39(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_c();
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let z = v == 0;
        self.set_c(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// srl d
    #[allow(unused_variables)]
    fn op_cb3a(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_d();
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let z = v == 0;
        self.set_d(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// srl e
    #[allow(unused_variables)]
    fn op_cb3b(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_e();
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let z = v == 0;
        self.set_e(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// srl h
    #[allow(unused_variables)]
    fn op_cb3c(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_h();
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let z = v == 0;
        self.set_h(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// srl l
    #[allow(unused_variables)]
    fn op_cb3d(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_l();
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let z = v == 0;
        self.set_l(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// srl (hl)
    #[allow(unused_variables)]
    fn op_cb3e(&mut self, arg: u16) -> (usize, usize) {
        let v = {
            let x = self.get_hl();
            self.get8(x)
        };
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let z = v == 0;
        let x = self.get_hl();
        self.set8(x, v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (16, 2)
    }

    /// srl a
    #[allow(unused_variables)]
    fn op_cb3f(&mut self, arg: u16) -> (usize, usize) {
        let v = self.get_a();
        let c = v & 1 != 0;
        let v = v.wrapping_shr(1);
        let z = v == 0;
        self.set_a(v);
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(false);
        self.set_cf(c);

        (8, 2)
    }

    /// bit 0,b
    #[allow(unused_variables)]
    fn op_cb40(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_b();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 0,c
    #[allow(unused_variables)]
    fn op_cb41(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_c();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 0,d
    #[allow(unused_variables)]
    fn op_cb42(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_d();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 0,e
    #[allow(unused_variables)]
    fn op_cb43(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_e();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 0,h
    #[allow(unused_variables)]
    fn op_cb44(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_h();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 0,l
    #[allow(unused_variables)]
    fn op_cb45(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_l();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 0,(hl)
    #[allow(unused_variables)]
    fn op_cb46(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (12, 2)
    }

    /// bit 0,a
    #[allow(unused_variables)]
    fn op_cb47(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_a();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 1,b
    #[allow(unused_variables)]
    fn op_cb48(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_b();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 1,c
    #[allow(unused_variables)]
    fn op_cb49(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_c();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 1,d
    #[allow(unused_variables)]
    fn op_cb4a(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_d();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 1,e
    #[allow(unused_variables)]
    fn op_cb4b(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_e();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 1,h
    #[allow(unused_variables)]
    fn op_cb4c(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_h();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 1,l
    #[allow(unused_variables)]
    fn op_cb4d(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_l();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 1,(hl)
    #[allow(unused_variables)]
    fn op_cb4e(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (12, 2)
    }

    /// bit 1,a
    #[allow(unused_variables)]
    fn op_cb4f(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_a();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 2,b
    #[allow(unused_variables)]
    fn op_cb50(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_b();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 2,c
    #[allow(unused_variables)]
    fn op_cb51(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_c();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 2,d
    #[allow(unused_variables)]
    fn op_cb52(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_d();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 2,e
    #[allow(unused_variables)]
    fn op_cb53(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_e();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 2,h
    #[allow(unused_variables)]
    fn op_cb54(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_h();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 2,l
    #[allow(unused_variables)]
    fn op_cb55(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_l();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 2,(hl)
    #[allow(unused_variables)]
    fn op_cb56(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (12, 2)
    }

    /// bit 2,a
    #[allow(unused_variables)]
    fn op_cb57(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_a();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 3,b
    #[allow(unused_variables)]
    fn op_cb58(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_b();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 3,c
    #[allow(unused_variables)]
    fn op_cb59(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_c();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 3,d
    #[allow(unused_variables)]
    fn op_cb5a(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_d();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 3,e
    #[allow(unused_variables)]
    fn op_cb5b(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_e();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 3,h
    #[allow(unused_variables)]
    fn op_cb5c(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_h();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 3,l
    #[allow(unused_variables)]
    fn op_cb5d(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_l();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 3,(hl)
    #[allow(unused_variables)]
    fn op_cb5e(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (12, 2)
    }

    /// bit 3,a
    #[allow(unused_variables)]
    fn op_cb5f(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_a();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 4,b
    #[allow(unused_variables)]
    fn op_cb60(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_b();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 4,c
    #[allow(unused_variables)]
    fn op_cb61(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_c();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 4,d
    #[allow(unused_variables)]
    fn op_cb62(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_d();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 4,e
    #[allow(unused_variables)]
    fn op_cb63(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_e();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 4,h
    #[allow(unused_variables)]
    fn op_cb64(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_h();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 4,l
    #[allow(unused_variables)]
    fn op_cb65(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_l();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 4,(hl)
    #[allow(unused_variables)]
    fn op_cb66(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (12, 2)
    }

    /// bit 4,a
    #[allow(unused_variables)]
    fn op_cb67(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_a();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 5,b
    #[allow(unused_variables)]
    fn op_cb68(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_b();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 5,c
    #[allow(unused_variables)]
    fn op_cb69(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_c();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 5,d
    #[allow(unused_variables)]
    fn op_cb6a(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_d();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 5,e
    #[allow(unused_variables)]
    fn op_cb6b(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_e();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 5,h
    #[allow(unused_variables)]
    fn op_cb6c(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_h();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 5,l
    #[allow(unused_variables)]
    fn op_cb6d(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_l();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 5,(hl)
    #[allow(unused_variables)]
    fn op_cb6e(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (12, 2)
    }

    /// bit 5,a
    #[allow(unused_variables)]
    fn op_cb6f(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_a();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 6,b
    #[allow(unused_variables)]
    fn op_cb70(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_b();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 6,c
    #[allow(unused_variables)]
    fn op_cb71(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_c();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 6,d
    #[allow(unused_variables)]
    fn op_cb72(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_d();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 6,e
    #[allow(unused_variables)]
    fn op_cb73(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_e();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 6,h
    #[allow(unused_variables)]
    fn op_cb74(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_h();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 6,l
    #[allow(unused_variables)]
    fn op_cb75(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_l();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 6,(hl)
    #[allow(unused_variables)]
    fn op_cb76(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (12, 2)
    }

    /// bit 6,a
    #[allow(unused_variables)]
    fn op_cb77(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_a();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 7,b
    #[allow(unused_variables)]
    fn op_cb78(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_b();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 7,c
    #[allow(unused_variables)]
    fn op_cb79(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_c();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 7,d
    #[allow(unused_variables)]
    fn op_cb7a(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_d();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 7,e
    #[allow(unused_variables)]
    fn op_cb7b(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_e();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 7,h
    #[allow(unused_variables)]
    fn op_cb7c(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_h();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 7,l
    #[allow(unused_variables)]
    fn op_cb7d(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_l();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// bit 7,(hl)
    #[allow(unused_variables)]
    fn op_cb7e(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (12, 2)
    }

    /// bit 7,a
    #[allow(unused_variables)]
    fn op_cb7f(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_a();
        let z = q & (1 << p) == 0;
        self.set_zf(z);
        self.set_nf(false);
        self.set_hf(true);

        (8, 2)
    }

    /// res 0,b
    #[allow(unused_variables)]
    fn op_cb80(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_b();
        self.set_b(q & !(1 << p));

        (8, 2)
    }

    /// res 0,c
    #[allow(unused_variables)]
    fn op_cb81(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_c();
        self.set_c(q & !(1 << p));

        (8, 2)
    }

    /// res 0,d
    #[allow(unused_variables)]
    fn op_cb82(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_d();
        self.set_d(q & !(1 << p));

        (8, 2)
    }

    /// res 0,e
    #[allow(unused_variables)]
    fn op_cb83(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_e();
        self.set_e(q & !(1 << p));

        (8, 2)
    }

    /// res 0,h
    #[allow(unused_variables)]
    fn op_cb84(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_h();
        self.set_h(q & !(1 << p));

        (8, 2)
    }

    /// res 0,l
    #[allow(unused_variables)]
    fn op_cb85(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_l();
        self.set_l(q & !(1 << p));

        (8, 2)
    }

    /// res 0,(hl)
    #[allow(unused_variables)]
    fn op_cb86(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let x = self.get_hl();
        self.set8(x, q & !(1 << p));

        (16, 2)
    }

    /// res 0,a
    #[allow(unused_variables)]
    fn op_cb87(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_a();
        self.set_a(q & !(1 << p));

        (8, 2)
    }

    /// res 1,b
    #[allow(unused_variables)]
    fn op_cb88(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_b();
        self.set_b(q & !(1 << p));

        (8, 2)
    }

    /// res 1,c
    #[allow(unused_variables)]
    fn op_cb89(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_c();
        self.set_c(q & !(1 << p));

        (8, 2)
    }

    /// res 1,d
    #[allow(unused_variables)]
    fn op_cb8a(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_d();
        self.set_d(q & !(1 << p));

        (8, 2)
    }

    /// res 1,e
    #[allow(unused_variables)]
    fn op_cb8b(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_e();
        self.set_e(q & !(1 << p));

        (8, 2)
    }

    /// res 1,h
    #[allow(unused_variables)]
    fn op_cb8c(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_h();
        self.set_h(q & !(1 << p));

        (8, 2)
    }

    /// res 1,l
    #[allow(unused_variables)]
    fn op_cb8d(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_l();
        self.set_l(q & !(1 << p));

        (8, 2)
    }

    /// res 1,(hl)
    #[allow(unused_variables)]
    fn op_cb8e(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let x = self.get_hl();
        self.set8(x, q & !(1 << p));

        (16, 2)
    }

    /// res 1,a
    #[allow(unused_variables)]
    fn op_cb8f(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_a();
        self.set_a(q & !(1 << p));

        (8, 2)
    }

    /// res 2,b
    #[allow(unused_variables)]
    fn op_cb90(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_b();
        self.set_b(q & !(1 << p));

        (8, 2)
    }

    /// res 2,c
    #[allow(unused_variables)]
    fn op_cb91(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_c();
        self.set_c(q & !(1 << p));

        (8, 2)
    }

    /// res 2,d
    #[allow(unused_variables)]
    fn op_cb92(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_d();
        self.set_d(q & !(1 << p));

        (8, 2)
    }

    /// res 2,e
    #[allow(unused_variables)]
    fn op_cb93(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_e();
        self.set_e(q & !(1 << p));

        (8, 2)
    }

    /// res 2,h
    #[allow(unused_variables)]
    fn op_cb94(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_h();
        self.set_h(q & !(1 << p));

        (8, 2)
    }

    /// res 2,l
    #[allow(unused_variables)]
    fn op_cb95(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_l();
        self.set_l(q & !(1 << p));

        (8, 2)
    }

    /// res 2,(hl)
    #[allow(unused_variables)]
    fn op_cb96(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let x = self.get_hl();
        self.set8(x, q & !(1 << p));

        (16, 2)
    }

    /// res 2,a
    #[allow(unused_variables)]
    fn op_cb97(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_a();
        self.set_a(q & !(1 << p));

        (8, 2)
    }

    /// res 3,b
    #[allow(unused_variables)]
    fn op_cb98(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_b();
        self.set_b(q & !(1 << p));

        (8, 2)
    }

    /// res 3,c
    #[allow(unused_variables)]
    fn op_cb99(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_c();
        self.set_c(q & !(1 << p));

        (8, 2)
    }

    /// res 3,d
    #[allow(unused_variables)]
    fn op_cb9a(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_d();
        self.set_d(q & !(1 << p));

        (8, 2)
    }

    /// res 3,e
    #[allow(unused_variables)]
    fn op_cb9b(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_e();
        self.set_e(q & !(1 << p));

        (8, 2)
    }

    /// res 3,h
    #[allow(unused_variables)]
    fn op_cb9c(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_h();
        self.set_h(q & !(1 << p));

        (8, 2)
    }

    /// res 3,l
    #[allow(unused_variables)]
    fn op_cb9d(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_l();
        self.set_l(q & !(1 << p));

        (8, 2)
    }

    /// res 3,(hl)
    #[allow(unused_variables)]
    fn op_cb9e(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let x = self.get_hl();
        self.set8(x, q & !(1 << p));

        (16, 2)
    }

    /// res 3,a
    #[allow(unused_variables)]
    fn op_cb9f(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_a();
        self.set_a(q & !(1 << p));

        (8, 2)
    }

    /// res 4,b
    #[allow(unused_variables)]
    fn op_cba0(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_b();
        self.set_b(q & !(1 << p));

        (8, 2)
    }

    /// res 4,c
    #[allow(unused_variables)]
    fn op_cba1(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_c();
        self.set_c(q & !(1 << p));

        (8, 2)
    }

    /// res 4,d
    #[allow(unused_variables)]
    fn op_cba2(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_d();
        self.set_d(q & !(1 << p));

        (8, 2)
    }

    /// res 4,e
    #[allow(unused_variables)]
    fn op_cba3(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_e();
        self.set_e(q & !(1 << p));

        (8, 2)
    }

    /// res 4,h
    #[allow(unused_variables)]
    fn op_cba4(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_h();
        self.set_h(q & !(1 << p));

        (8, 2)
    }

    /// res 4,l
    #[allow(unused_variables)]
    fn op_cba5(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_l();
        self.set_l(q & !(1 << p));

        (8, 2)
    }

    /// res 4,(hl)
    #[allow(unused_variables)]
    fn op_cba6(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let x = self.get_hl();
        self.set8(x, q & !(1 << p));

        (16, 2)
    }

    /// res 4,a
    #[allow(unused_variables)]
    fn op_cba7(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_a();
        self.set_a(q & !(1 << p));

        (8, 2)
    }

    /// res 5,b
    #[allow(unused_variables)]
    fn op_cba8(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_b();
        self.set_b(q & !(1 << p));

        (8, 2)
    }

    /// res 5,c
    #[allow(unused_variables)]
    fn op_cba9(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_c();
        self.set_c(q & !(1 << p));

        (8, 2)
    }

    /// res 5,d
    #[allow(unused_variables)]
    fn op_cbaa(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_d();
        self.set_d(q & !(1 << p));

        (8, 2)
    }

    /// res 5,e
    #[allow(unused_variables)]
    fn op_cbab(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_e();
        self.set_e(q & !(1 << p));

        (8, 2)
    }

    /// res 5,h
    #[allow(unused_variables)]
    fn op_cbac(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_h();
        self.set_h(q & !(1 << p));

        (8, 2)
    }

    /// res 5,l
    #[allow(unused_variables)]
    fn op_cbad(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_l();
        self.set_l(q & !(1 << p));

        (8, 2)
    }

    /// res 5,(hl)
    #[allow(unused_variables)]
    fn op_cbae(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let x = self.get_hl();
        self.set8(x, q & !(1 << p));

        (16, 2)
    }

    /// res 5,a
    #[allow(unused_variables)]
    fn op_cbaf(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_a();
        self.set_a(q & !(1 << p));

        (8, 2)
    }

    /// res 6,b
    #[allow(unused_variables)]
    fn op_cbb0(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_b();
        self.set_b(q & !(1 << p));

        (8, 2)
    }

    /// res 6,c
    #[allow(unused_variables)]
    fn op_cbb1(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_c();
        self.set_c(q & !(1 << p));

        (8, 2)
    }

    /// res 6,d
    #[allow(unused_variables)]
    fn op_cbb2(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_d();
        self.set_d(q & !(1 << p));

        (8, 2)
    }

    /// res 6,e
    #[allow(unused_variables)]
    fn op_cbb3(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_e();
        self.set_e(q & !(1 << p));

        (8, 2)
    }

    /// res 6,h
    #[allow(unused_variables)]
    fn op_cbb4(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_h();
        self.set_h(q & !(1 << p));

        (8, 2)
    }

    /// res 6,l
    #[allow(unused_variables)]
    fn op_cbb5(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_l();
        self.set_l(q & !(1 << p));

        (8, 2)
    }

    /// res 6,(hl)
    #[allow(unused_variables)]
    fn op_cbb6(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let x = self.get_hl();
        self.set8(x, q & !(1 << p));

        (16, 2)
    }

    /// res 6,a
    #[allow(unused_variables)]
    fn op_cbb7(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_a();
        self.set_a(q & !(1 << p));

        (8, 2)
    }

    /// res 7,b
    #[allow(unused_variables)]
    fn op_cbb8(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_b();
        self.set_b(q & !(1 << p));

        (8, 2)
    }

    /// res 7,c
    #[allow(unused_variables)]
    fn op_cbb9(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_c();
        self.set_c(q & !(1 << p));

        (8, 2)
    }

    /// res 7,d
    #[allow(unused_variables)]
    fn op_cbba(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_d();
        self.set_d(q & !(1 << p));

        (8, 2)
    }

    /// res 7,e
    #[allow(unused_variables)]
    fn op_cbbb(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_e();
        self.set_e(q & !(1 << p));

        (8, 2)
    }

    /// res 7,h
    #[allow(unused_variables)]
    fn op_cbbc(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_h();
        self.set_h(q & !(1 << p));

        (8, 2)
    }

    /// res 7,l
    #[allow(unused_variables)]
    fn op_cbbd(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_l();
        self.set_l(q & !(1 << p));

        (8, 2)
    }

    /// res 7,(hl)
    #[allow(unused_variables)]
    fn op_cbbe(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let x = self.get_hl();
        self.set8(x, q & !(1 << p));

        (16, 2)
    }

    /// res 7,a
    #[allow(unused_variables)]
    fn op_cbbf(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_a();
        self.set_a(q & !(1 << p));

        (8, 2)
    }

    /// set 0,b
    #[allow(unused_variables)]
    fn op_cbc0(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_b();
        self.set_b(q | (1 << p));

        (8, 2)
    }

    /// set 0,c
    #[allow(unused_variables)]
    fn op_cbc1(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_c();
        self.set_c(q | (1 << p));

        (8, 2)
    }

    /// set 0,d
    #[allow(unused_variables)]
    fn op_cbc2(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_d();
        self.set_d(q | (1 << p));

        (8, 2)
    }

    /// set 0,e
    #[allow(unused_variables)]
    fn op_cbc3(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_e();
        self.set_e(q | (1 << p));

        (8, 2)
    }

    /// set 0,h
    #[allow(unused_variables)]
    fn op_cbc4(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_h();
        self.set_h(q | (1 << p));

        (8, 2)
    }

    /// set 0,l
    #[allow(unused_variables)]
    fn op_cbc5(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_l();
        self.set_l(q | (1 << p));

        (8, 2)
    }

    /// set 0,(hl)
    #[allow(unused_variables)]
    fn op_cbc6(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let x = self.get_hl();
        self.set8(x, q | (1 << p));

        (16, 2)
    }

    /// set 0,a
    #[allow(unused_variables)]
    fn op_cbc7(&mut self, arg: u16) -> (usize, usize) {
        let p = 0;
        let q = self.get_a();
        self.set_a(q | (1 << p));

        (8, 2)
    }

    /// set 1,b
    #[allow(unused_variables)]
    fn op_cbc8(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_b();
        self.set_b(q | (1 << p));

        (8, 2)
    }

    /// set 1,c
    #[allow(unused_variables)]
    fn op_cbc9(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_c();
        self.set_c(q | (1 << p));

        (8, 2)
    }

    /// set 1,d
    #[allow(unused_variables)]
    fn op_cbca(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_d();
        self.set_d(q | (1 << p));

        (8, 2)
    }

    /// set 1,e
    #[allow(unused_variables)]
    fn op_cbcb(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_e();
        self.set_e(q | (1 << p));

        (8, 2)
    }

    /// set 1,h
    #[allow(unused_variables)]
    fn op_cbcc(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_h();
        self.set_h(q | (1 << p));

        (8, 2)
    }

    /// set 1,l
    #[allow(unused_variables)]
    fn op_cbcd(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_l();
        self.set_l(q | (1 << p));

        (8, 2)
    }

    /// set 1,(hl)
    #[allow(unused_variables)]
    fn op_cbce(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let x = self.get_hl();
        self.set8(x, q | (1 << p));

        (16, 2)
    }

    /// set 1,a
    #[allow(unused_variables)]
    fn op_cbcf(&mut self, arg: u16) -> (usize, usize) {
        let p = 1;
        let q = self.get_a();
        self.set_a(q | (1 << p));

        (8, 2)
    }

    /// set 2,b
    #[allow(unused_variables)]
    fn op_cbd0(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_b();
        self.set_b(q | (1 << p));

        (8, 2)
    }

    /// set 2,c
    #[allow(unused_variables)]
    fn op_cbd1(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_c();
        self.set_c(q | (1 << p));

        (8, 2)
    }

    /// set 2,d
    #[allow(unused_variables)]
    fn op_cbd2(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_d();
        self.set_d(q | (1 << p));

        (8, 2)
    }

    /// set 2,e
    #[allow(unused_variables)]
    fn op_cbd3(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_e();
        self.set_e(q | (1 << p));

        (8, 2)
    }

    /// set 2,h
    #[allow(unused_variables)]
    fn op_cbd4(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_h();
        self.set_h(q | (1 << p));

        (8, 2)
    }

    /// set 2,l
    #[allow(unused_variables)]
    fn op_cbd5(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_l();
        self.set_l(q | (1 << p));

        (8, 2)
    }

    /// set 2,(hl)
    #[allow(unused_variables)]
    fn op_cbd6(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let x = self.get_hl();
        self.set8(x, q | (1 << p));

        (16, 2)
    }

    /// set 2,a
    #[allow(unused_variables)]
    fn op_cbd7(&mut self, arg: u16) -> (usize, usize) {
        let p = 2;
        let q = self.get_a();
        self.set_a(q | (1 << p));

        (8, 2)
    }

    /// set 3,b
    #[allow(unused_variables)]
    fn op_cbd8(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_b();
        self.set_b(q | (1 << p));

        (8, 2)
    }

    /// set 3,c
    #[allow(unused_variables)]
    fn op_cbd9(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_c();
        self.set_c(q | (1 << p));

        (8, 2)
    }

    /// set 3,d
    #[allow(unused_variables)]
    fn op_cbda(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_d();
        self.set_d(q | (1 << p));

        (8, 2)
    }

    /// set 3,e
    #[allow(unused_variables)]
    fn op_cbdb(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_e();
        self.set_e(q | (1 << p));

        (8, 2)
    }

    /// set 3,h
    #[allow(unused_variables)]
    fn op_cbdc(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_h();
        self.set_h(q | (1 << p));

        (8, 2)
    }

    /// set 3,l
    #[allow(unused_variables)]
    fn op_cbdd(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_l();
        self.set_l(q | (1 << p));

        (8, 2)
    }

    /// set 3,(hl)
    #[allow(unused_variables)]
    fn op_cbde(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let x = self.get_hl();
        self.set8(x, q | (1 << p));

        (16, 2)
    }

    /// set 3,a
    #[allow(unused_variables)]
    fn op_cbdf(&mut self, arg: u16) -> (usize, usize) {
        let p = 3;
        let q = self.get_a();
        self.set_a(q | (1 << p));

        (8, 2)
    }

    /// set 4,b
    #[allow(unused_variables)]
    fn op_cbe0(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_b();
        self.set_b(q | (1 << p));

        (8, 2)
    }

    /// set 4,c
    #[allow(unused_variables)]
    fn op_cbe1(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_c();
        self.set_c(q | (1 << p));

        (8, 2)
    }

    /// set 4,d
    #[allow(unused_variables)]
    fn op_cbe2(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_d();
        self.set_d(q | (1 << p));

        (8, 2)
    }

    /// set 4,e
    #[allow(unused_variables)]
    fn op_cbe3(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_e();
        self.set_e(q | (1 << p));

        (8, 2)
    }

    /// set 4,h
    #[allow(unused_variables)]
    fn op_cbe4(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_h();
        self.set_h(q | (1 << p));

        (8, 2)
    }

    /// set 4,l
    #[allow(unused_variables)]
    fn op_cbe5(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_l();
        self.set_l(q | (1 << p));

        (8, 2)
    }

    /// set 4,(hl)
    #[allow(unused_variables)]
    fn op_cbe6(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let x = self.get_hl();
        self.set8(x, q | (1 << p));

        (16, 2)
    }

    /// set 4,a
    #[allow(unused_variables)]
    fn op_cbe7(&mut self, arg: u16) -> (usize, usize) {
        let p = 4;
        let q = self.get_a();
        self.set_a(q | (1 << p));

        (8, 2)
    }

    /// set 5,b
    #[allow(unused_variables)]
    fn op_cbe8(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_b();
        self.set_b(q | (1 << p));

        (8, 2)
    }

    /// set 5,c
    #[allow(unused_variables)]
    fn op_cbe9(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_c();
        self.set_c(q | (1 << p));

        (8, 2)
    }

    /// set 5,d
    #[allow(unused_variables)]
    fn op_cbea(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_d();
        self.set_d(q | (1 << p));

        (8, 2)
    }

    /// set 5,e
    #[allow(unused_variables)]
    fn op_cbeb(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_e();
        self.set_e(q | (1 << p));

        (8, 2)
    }

    /// set 5,h
    #[allow(unused_variables)]
    fn op_cbec(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_h();
        self.set_h(q | (1 << p));

        (8, 2)
    }

    /// set 5,l
    #[allow(unused_variables)]
    fn op_cbed(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_l();
        self.set_l(q | (1 << p));

        (8, 2)
    }

    /// set 5,(hl)
    #[allow(unused_variables)]
    fn op_cbee(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let x = self.get_hl();
        self.set8(x, q | (1 << p));

        (16, 2)
    }

    /// set 5,a
    #[allow(unused_variables)]
    fn op_cbef(&mut self, arg: u16) -> (usize, usize) {
        let p = 5;
        let q = self.get_a();
        self.set_a(q | (1 << p));

        (8, 2)
    }

    /// set 6,b
    #[allow(unused_variables)]
    fn op_cbf0(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_b();
        self.set_b(q | (1 << p));

        (8, 2)
    }

    /// set 6,c
    #[allow(unused_variables)]
    fn op_cbf1(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_c();
        self.set_c(q | (1 << p));

        (8, 2)
    }

    /// set 6,d
    #[allow(unused_variables)]
    fn op_cbf2(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_d();
        self.set_d(q | (1 << p));

        (8, 2)
    }

    /// set 6,e
    #[allow(unused_variables)]
    fn op_cbf3(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_e();
        self.set_e(q | (1 << p));

        (8, 2)
    }

    /// set 6,h
    #[allow(unused_variables)]
    fn op_cbf4(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_h();
        self.set_h(q | (1 << p));

        (8, 2)
    }

    /// set 6,l
    #[allow(unused_variables)]
    fn op_cbf5(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_l();
        self.set_l(q | (1 << p));

        (8, 2)
    }

    /// set 6,(hl)
    #[allow(unused_variables)]
    fn op_cbf6(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let x = self.get_hl();
        self.set8(x, q | (1 << p));

        (16, 2)
    }

    /// set 6,a
    #[allow(unused_variables)]
    fn op_cbf7(&mut self, arg: u16) -> (usize, usize) {
        let p = 6;
        let q = self.get_a();
        self.set_a(q | (1 << p));

        (8, 2)
    }

    /// set 7,b
    #[allow(unused_variables)]
    fn op_cbf8(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_b();
        self.set_b(q | (1 << p));

        (8, 2)
    }

    /// set 7,c
    #[allow(unused_variables)]
    fn op_cbf9(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_c();
        self.set_c(q | (1 << p));

        (8, 2)
    }

    /// set 7,d
    #[allow(unused_variables)]
    fn op_cbfa(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_d();
        self.set_d(q | (1 << p));

        (8, 2)
    }

    /// set 7,e
    #[allow(unused_variables)]
    fn op_cbfb(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_e();
        self.set_e(q | (1 << p));

        (8, 2)
    }

    /// set 7,h
    #[allow(unused_variables)]
    fn op_cbfc(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_h();
        self.set_h(q | (1 << p));

        (8, 2)
    }

    /// set 7,l
    #[allow(unused_variables)]
    fn op_cbfd(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_l();
        self.set_l(q | (1 << p));

        (8, 2)
    }

    /// set 7,(hl)
    #[allow(unused_variables)]
    fn op_cbfe(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = {
            let x = self.get_hl();
            self.get8(x)
        };
        let x = self.get_hl();
        self.set8(x, q | (1 << p));

        (16, 2)
    }

    /// set 7,a
    #[allow(unused_variables)]
    fn op_cbff(&mut self, arg: u16) -> (usize, usize) {
        let p = 7;
        let q = self.get_a();
        self.set_a(q | (1 << p));

        (8, 2)
    }
}

/// Return the mnemonic string for the given opcode.
pub fn mnem(code: u16) -> &'static str {
    MNEMONICS.get(&code).unwrap_or(&"(unknown opcode)")
}

/// Decodes the opecode and actually executes one instruction.
impl<T: Sys> Cpu<T> {
    /// Execute the instruction returning the expected consumed cycles and the instruction size
    pub fn decode(&mut self, code: u16, arg: u16) -> (usize, usize) {
        trace!("{:04x}: {:04x}: {}", self.get_pc(), code, mnem(code));

        let (time, size) = match code {
            0x0000 => self.op_0000(arg),
            0x0001 => self.op_0001(arg),
            0x0002 => self.op_0002(arg),
            0x0003 => self.op_0003(arg),
            0x0004 => self.op_0004(arg),
            0x0005 => self.op_0005(arg),
            0x0006 => self.op_0006(arg),
            0x0007 => self.op_0007(arg),
            0x0008 => self.op_0008(arg),
            0x0009 => self.op_0009(arg),
            0x000a => self.op_000a(arg),
            0x000b => self.op_000b(arg),
            0x000c => self.op_000c(arg),
            0x000d => self.op_000d(arg),
            0x000e => self.op_000e(arg),
            0x000f => self.op_000f(arg),
            0x0010 => self.op_0010(arg),
            0x0011 => self.op_0011(arg),
            0x0012 => self.op_0012(arg),
            0x0013 => self.op_0013(arg),
            0x0014 => self.op_0014(arg),
            0x0015 => self.op_0015(arg),
            0x0016 => self.op_0016(arg),
            0x0017 => self.op_0017(arg),
            0x0018 => self.op_0018(arg),
            0x0019 => self.op_0019(arg),
            0x001a => self.op_001a(arg),
            0x001b => self.op_001b(arg),
            0x001c => self.op_001c(arg),
            0x001d => self.op_001d(arg),
            0x001e => self.op_001e(arg),
            0x001f => self.op_001f(arg),
            0x0020 => self.op_0020(arg),
            0x0021 => self.op_0021(arg),
            0x0022 => self.op_0022(arg),
            0x0023 => self.op_0023(arg),
            0x0024 => self.op_0024(arg),
            0x0025 => self.op_0025(arg),
            0x0026 => self.op_0026(arg),
            0x0027 => self.op_0027(arg),
            0x0028 => self.op_0028(arg),
            0x0029 => self.op_0029(arg),
            0x002a => self.op_002a(arg),
            0x002b => self.op_002b(arg),
            0x002c => self.op_002c(arg),
            0x002d => self.op_002d(arg),
            0x002e => self.op_002e(arg),
            0x002f => self.op_002f(arg),
            0x0030 => self.op_0030(arg),
            0x0031 => self.op_0031(arg),
            0x0032 => self.op_0032(arg),
            0x0033 => self.op_0033(arg),
            0x0034 => self.op_0034(arg),
            0x0035 => self.op_0035(arg),
            0x0036 => self.op_0036(arg),
            0x0037 => self.op_0037(arg),
            0x0038 => self.op_0038(arg),
            0x0039 => self.op_0039(arg),
            0x003a => self.op_003a(arg),
            0x003b => self.op_003b(arg),
            0x003c => self.op_003c(arg),
            0x003d => self.op_003d(arg),
            0x003e => self.op_003e(arg),
            0x003f => self.op_003f(arg),
            0x0040 => self.op_0040(arg),
            0x0041 => self.op_0041(arg),
            0x0042 => self.op_0042(arg),
            0x0043 => self.op_0043(arg),
            0x0044 => self.op_0044(arg),
            0x0045 => self.op_0045(arg),
            0x0046 => self.op_0046(arg),
            0x0047 => self.op_0047(arg),
            0x0048 => self.op_0048(arg),
            0x0049 => self.op_0049(arg),
            0x004a => self.op_004a(arg),
            0x004b => self.op_004b(arg),
            0x004c => self.op_004c(arg),
            0x004d => self.op_004d(arg),
            0x004e => self.op_004e(arg),
            0x004f => self.op_004f(arg),
            0x0050 => self.op_0050(arg),
            0x0051 => self.op_0051(arg),
            0x0052 => self.op_0052(arg),
            0x0053 => self.op_0053(arg),
            0x0054 => self.op_0054(arg),
            0x0055 => self.op_0055(arg),
            0x0056 => self.op_0056(arg),
            0x0057 => self.op_0057(arg),
            0x0058 => self.op_0058(arg),
            0x0059 => self.op_0059(arg),
            0x005a => self.op_005a(arg),
            0x005b => self.op_005b(arg),
            0x005c => self.op_005c(arg),
            0x005d => self.op_005d(arg),
            0x005e => self.op_005e(arg),
            0x005f => self.op_005f(arg),
            0x0060 => self.op_0060(arg),
            0x0061 => self.op_0061(arg),
            0x0062 => self.op_0062(arg),
            0x0063 => self.op_0063(arg),
            0x0064 => self.op_0064(arg),
            0x0065 => self.op_0065(arg),
            0x0066 => self.op_0066(arg),
            0x0067 => self.op_0067(arg),
            0x0068 => self.op_0068(arg),
            0x0069 => self.op_0069(arg),
            0x006a => self.op_006a(arg),
            0x006b => self.op_006b(arg),
            0x006c => self.op_006c(arg),
            0x006d => self.op_006d(arg),
            0x006e => self.op_006e(arg),
            0x006f => self.op_006f(arg),
            0x0070 => self.op_0070(arg),
            0x0071 => self.op_0071(arg),
            0x0072 => self.op_0072(arg),
            0x0073 => self.op_0073(arg),
            0x0074 => self.op_0074(arg),
            0x0075 => self.op_0075(arg),
            0x0076 => self.op_0076(arg),
            0x0077 => self.op_0077(arg),
            0x0078 => self.op_0078(arg),
            0x0079 => self.op_0079(arg),
            0x007a => self.op_007a(arg),
            0x007b => self.op_007b(arg),
            0x007c => self.op_007c(arg),
            0x007d => self.op_007d(arg),
            0x007e => self.op_007e(arg),
            0x007f => self.op_007f(arg),
            0x0080 => self.op_0080(arg),
            0x0081 => self.op_0081(arg),
            0x0082 => self.op_0082(arg),
            0x0083 => self.op_0083(arg),
            0x0084 => self.op_0084(arg),
            0x0085 => self.op_0085(arg),
            0x0086 => self.op_0086(arg),
            0x0087 => self.op_0087(arg),
            0x0088 => self.op_0088(arg),
            0x0089 => self.op_0089(arg),
            0x008a => self.op_008a(arg),
            0x008b => self.op_008b(arg),
            0x008c => self.op_008c(arg),
            0x008d => self.op_008d(arg),
            0x008e => self.op_008e(arg),
            0x008f => self.op_008f(arg),
            0x0090 => self.op_0090(arg),
            0x0091 => self.op_0091(arg),
            0x0092 => self.op_0092(arg),
            0x0093 => self.op_0093(arg),
            0x0094 => self.op_0094(arg),
            0x0095 => self.op_0095(arg),
            0x0096 => self.op_0096(arg),
            0x0097 => self.op_0097(arg),
            0x0098 => self.op_0098(arg),
            0x0099 => self.op_0099(arg),
            0x009a => self.op_009a(arg),
            0x009b => self.op_009b(arg),
            0x009c => self.op_009c(arg),
            0x009d => self.op_009d(arg),
            0x009e => self.op_009e(arg),
            0x009f => self.op_009f(arg),
            0x00a0 => self.op_00a0(arg),
            0x00a1 => self.op_00a1(arg),
            0x00a2 => self.op_00a2(arg),
            0x00a3 => self.op_00a3(arg),
            0x00a4 => self.op_00a4(arg),
            0x00a5 => self.op_00a5(arg),
            0x00a6 => self.op_00a6(arg),
            0x00a7 => self.op_00a7(arg),
            0x00a8 => self.op_00a8(arg),
            0x00a9 => self.op_00a9(arg),
            0x00aa => self.op_00aa(arg),
            0x00ab => self.op_00ab(arg),
            0x00ac => self.op_00ac(arg),
            0x00ad => self.op_00ad(arg),
            0x00ae => self.op_00ae(arg),
            0x00af => self.op_00af(arg),
            0x00b0 => self.op_00b0(arg),
            0x00b1 => self.op_00b1(arg),
            0x00b2 => self.op_00b2(arg),
            0x00b3 => self.op_00b3(arg),
            0x00b4 => self.op_00b4(arg),
            0x00b5 => self.op_00b5(arg),
            0x00b6 => self.op_00b6(arg),
            0x00b7 => self.op_00b7(arg),
            0x00b8 => self.op_00b8(arg),
            0x00b9 => self.op_00b9(arg),
            0x00ba => self.op_00ba(arg),
            0x00bb => self.op_00bb(arg),
            0x00bc => self.op_00bc(arg),
            0x00bd => self.op_00bd(arg),
            0x00be => self.op_00be(arg),
            0x00bf => self.op_00bf(arg),
            0x00c0 => self.op_00c0(arg),
            0x00c1 => self.op_00c1(arg),
            0x00c2 => self.op_00c2(arg),
            0x00c3 => self.op_00c3(arg),
            0x00c4 => self.op_00c4(arg),
            0x00c5 => self.op_00c5(arg),
            0x00c6 => self.op_00c6(arg),
            0x00c7 => self.op_00c7(arg),
            0x00c8 => self.op_00c8(arg),
            0x00c9 => self.op_00c9(arg),
            0x00ca => self.op_00ca(arg),
            0x00cb => self.op_00cb(arg),
            0x00cc => self.op_00cc(arg),
            0x00cd => self.op_00cd(arg),
            0x00ce => self.op_00ce(arg),
            0x00cf => self.op_00cf(arg),
            0x00d0 => self.op_00d0(arg),
            0x00d1 => self.op_00d1(arg),
            0x00d2 => self.op_00d2(arg),
            0x00d4 => self.op_00d4(arg),
            0x00d5 => self.op_00d5(arg),
            0x00d6 => self.op_00d6(arg),
            0x00d7 => self.op_00d7(arg),
            0x00d8 => self.op_00d8(arg),
            0x00d9 => self.op_00d9(arg),
            0x00da => self.op_00da(arg),
            0x00dc => self.op_00dc(arg),
            0x00de => self.op_00de(arg),
            0x00df => self.op_00df(arg),
            0x00e0 => self.op_00e0(arg),
            0x00e1 => self.op_00e1(arg),
            0x00e2 => self.op_00e2(arg),
            0x00e5 => self.op_00e5(arg),
            0x00e6 => self.op_00e6(arg),
            0x00e7 => self.op_00e7(arg),
            0x00e8 => self.op_00e8(arg),
            0x00e9 => self.op_00e9(arg),
            0x00ea => self.op_00ea(arg),
            0x00ee => self.op_00ee(arg),
            0x00ef => self.op_00ef(arg),
            0x00f0 => self.op_00f0(arg),
            0x00f1 => self.op_00f1(arg),
            0x00f2 => self.op_00f2(arg),
            0x00f3 => self.op_00f3(arg),
            0x00f5 => self.op_00f5(arg),
            0x00f6 => self.op_00f6(arg),
            0x00f7 => self.op_00f7(arg),
            0x00f8 => self.op_00f8(arg),
            0x00f9 => self.op_00f9(arg),
            0x00fa => self.op_00fa(arg),
            0x00fb => self.op_00fb(arg),
            0x00fe => self.op_00fe(arg),
            0x00ff => self.op_00ff(arg),
            0xcb00 => self.op_cb00(arg),
            0xcb01 => self.op_cb01(arg),
            0xcb02 => self.op_cb02(arg),
            0xcb03 => self.op_cb03(arg),
            0xcb04 => self.op_cb04(arg),
            0xcb05 => self.op_cb05(arg),
            0xcb06 => self.op_cb06(arg),
            0xcb07 => self.op_cb07(arg),
            0xcb08 => self.op_cb08(arg),
            0xcb09 => self.op_cb09(arg),
            0xcb0a => self.op_cb0a(arg),
            0xcb0b => self.op_cb0b(arg),
            0xcb0c => self.op_cb0c(arg),
            0xcb0d => self.op_cb0d(arg),
            0xcb0e => self.op_cb0e(arg),
            0xcb0f => self.op_cb0f(arg),
            0xcb10 => self.op_cb10(arg),
            0xcb11 => self.op_cb11(arg),
            0xcb12 => self.op_cb12(arg),
            0xcb13 => self.op_cb13(arg),
            0xcb14 => self.op_cb14(arg),
            0xcb15 => self.op_cb15(arg),
            0xcb16 => self.op_cb16(arg),
            0xcb17 => self.op_cb17(arg),
            0xcb18 => self.op_cb18(arg),
            0xcb19 => self.op_cb19(arg),
            0xcb1a => self.op_cb1a(arg),
            0xcb1b => self.op_cb1b(arg),
            0xcb1c => self.op_cb1c(arg),
            0xcb1d => self.op_cb1d(arg),
            0xcb1e => self.op_cb1e(arg),
            0xcb1f => self.op_cb1f(arg),
            0xcb20 => self.op_cb20(arg),
            0xcb21 => self.op_cb21(arg),
            0xcb22 => self.op_cb22(arg),
            0xcb23 => self.op_cb23(arg),
            0xcb24 => self.op_cb24(arg),
            0xcb25 => self.op_cb25(arg),
            0xcb26 => self.op_cb26(arg),
            0xcb27 => self.op_cb27(arg),
            0xcb28 => self.op_cb28(arg),
            0xcb29 => self.op_cb29(arg),
            0xcb2a => self.op_cb2a(arg),
            0xcb2b => self.op_cb2b(arg),
            0xcb2c => self.op_cb2c(arg),
            0xcb2d => self.op_cb2d(arg),
            0xcb2e => self.op_cb2e(arg),
            0xcb2f => self.op_cb2f(arg),
            0xcb30 => self.op_cb30(arg),
            0xcb31 => self.op_cb31(arg),
            0xcb32 => self.op_cb32(arg),
            0xcb33 => self.op_cb33(arg),
            0xcb34 => self.op_cb34(arg),
            0xcb35 => self.op_cb35(arg),
            0xcb36 => self.op_cb36(arg),
            0xcb37 => self.op_cb37(arg),
            0xcb38 => self.op_cb38(arg),
            0xcb39 => self.op_cb39(arg),
            0xcb3a => self.op_cb3a(arg),
            0xcb3b => self.op_cb3b(arg),
            0xcb3c => self.op_cb3c(arg),
            0xcb3d => self.op_cb3d(arg),
            0xcb3e => self.op_cb3e(arg),
            0xcb3f => self.op_cb3f(arg),
            0xcb40 => self.op_cb40(arg),
            0xcb41 => self.op_cb41(arg),
            0xcb42 => self.op_cb42(arg),
            0xcb43 => self.op_cb43(arg),
            0xcb44 => self.op_cb44(arg),
            0xcb45 => self.op_cb45(arg),
            0xcb46 => self.op_cb46(arg),
            0xcb47 => self.op_cb47(arg),
            0xcb48 => self.op_cb48(arg),
            0xcb49 => self.op_cb49(arg),
            0xcb4a => self.op_cb4a(arg),
            0xcb4b => self.op_cb4b(arg),
            0xcb4c => self.op_cb4c(arg),
            0xcb4d => self.op_cb4d(arg),
            0xcb4e => self.op_cb4e(arg),
            0xcb4f => self.op_cb4f(arg),
            0xcb50 => self.op_cb50(arg),
            0xcb51 => self.op_cb51(arg),
            0xcb52 => self.op_cb52(arg),
            0xcb53 => self.op_cb53(arg),
            0xcb54 => self.op_cb54(arg),
            0xcb55 => self.op_cb55(arg),
            0xcb56 => self.op_cb56(arg),
            0xcb57 => self.op_cb57(arg),
            0xcb58 => self.op_cb58(arg),
            0xcb59 => self.op_cb59(arg),
            0xcb5a => self.op_cb5a(arg),
            0xcb5b => self.op_cb5b(arg),
            0xcb5c => self.op_cb5c(arg),
            0xcb5d => self.op_cb5d(arg),
            0xcb5e => self.op_cb5e(arg),
            0xcb5f => self.op_cb5f(arg),
            0xcb60 => self.op_cb60(arg),
            0xcb61 => self.op_cb61(arg),
            0xcb62 => self.op_cb62(arg),
            0xcb63 => self.op_cb63(arg),
            0xcb64 => self.op_cb64(arg),
            0xcb65 => self.op_cb65(arg),
            0xcb66 => self.op_cb66(arg),
            0xcb67 => self.op_cb67(arg),
            0xcb68 => self.op_cb68(arg),
            0xcb69 => self.op_cb69(arg),
            0xcb6a => self.op_cb6a(arg),
            0xcb6b => self.op_cb6b(arg),
            0xcb6c => self.op_cb6c(arg),
            0xcb6d => self.op_cb6d(arg),
            0xcb6e => self.op_cb6e(arg),
            0xcb6f => self.op_cb6f(arg),
            0xcb70 => self.op_cb70(arg),
            0xcb71 => self.op_cb71(arg),
            0xcb72 => self.op_cb72(arg),
            0xcb73 => self.op_cb73(arg),
            0xcb74 => self.op_cb74(arg),
            0xcb75 => self.op_cb75(arg),
            0xcb76 => self.op_cb76(arg),
            0xcb77 => self.op_cb77(arg),
            0xcb78 => self.op_cb78(arg),
            0xcb79 => self.op_cb79(arg),
            0xcb7a => self.op_cb7a(arg),
            0xcb7b => self.op_cb7b(arg),
            0xcb7c => self.op_cb7c(arg),
            0xcb7d => self.op_cb7d(arg),
            0xcb7e => self.op_cb7e(arg),
            0xcb7f => self.op_cb7f(arg),
            0xcb80 => self.op_cb80(arg),
            0xcb81 => self.op_cb81(arg),
            0xcb82 => self.op_cb82(arg),
            0xcb83 => self.op_cb83(arg),
            0xcb84 => self.op_cb84(arg),
            0xcb85 => self.op_cb85(arg),
            0xcb86 => self.op_cb86(arg),
            0xcb87 => self.op_cb87(arg),
            0xcb88 => self.op_cb88(arg),
            0xcb89 => self.op_cb89(arg),
            0xcb8a => self.op_cb8a(arg),
            0xcb8b => self.op_cb8b(arg),
            0xcb8c => self.op_cb8c(arg),
            0xcb8d => self.op_cb8d(arg),
            0xcb8e => self.op_cb8e(arg),
            0xcb8f => self.op_cb8f(arg),
            0xcb90 => self.op_cb90(arg),
            0xcb91 => self.op_cb91(arg),
            0xcb92 => self.op_cb92(arg),
            0xcb93 => self.op_cb93(arg),
            0xcb94 => self.op_cb94(arg),
            0xcb95 => self.op_cb95(arg),
            0xcb96 => self.op_cb96(arg),
            0xcb97 => self.op_cb97(arg),
            0xcb98 => self.op_cb98(arg),
            0xcb99 => self.op_cb99(arg),
            0xcb9a => self.op_cb9a(arg),
            0xcb9b => self.op_cb9b(arg),
            0xcb9c => self.op_cb9c(arg),
            0xcb9d => self.op_cb9d(arg),
            0xcb9e => self.op_cb9e(arg),
            0xcb9f => self.op_cb9f(arg),
            0xcba0 => self.op_cba0(arg),
            0xcba1 => self.op_cba1(arg),
            0xcba2 => self.op_cba2(arg),
            0xcba3 => self.op_cba3(arg),
            0xcba4 => self.op_cba4(arg),
            0xcba5 => self.op_cba5(arg),
            0xcba6 => self.op_cba6(arg),
            0xcba7 => self.op_cba7(arg),
            0xcba8 => self.op_cba8(arg),
            0xcba9 => self.op_cba9(arg),
            0xcbaa => self.op_cbaa(arg),
            0xcbab => self.op_cbab(arg),
            0xcbac => self.op_cbac(arg),
            0xcbad => self.op_cbad(arg),
            0xcbae => self.op_cbae(arg),
            0xcbaf => self.op_cbaf(arg),
            0xcbb0 => self.op_cbb0(arg),
            0xcbb1 => self.op_cbb1(arg),
            0xcbb2 => self.op_cbb2(arg),
            0xcbb3 => self.op_cbb3(arg),
            0xcbb4 => self.op_cbb4(arg),
            0xcbb5 => self.op_cbb5(arg),
            0xcbb6 => self.op_cbb6(arg),
            0xcbb7 => self.op_cbb7(arg),
            0xcbb8 => self.op_cbb8(arg),
            0xcbb9 => self.op_cbb9(arg),
            0xcbba => self.op_cbba(arg),
            0xcbbb => self.op_cbbb(arg),
            0xcbbc => self.op_cbbc(arg),
            0xcbbd => self.op_cbbd(arg),
            0xcbbe => self.op_cbbe(arg),
            0xcbbf => self.op_cbbf(arg),
            0xcbc0 => self.op_cbc0(arg),
            0xcbc1 => self.op_cbc1(arg),
            0xcbc2 => self.op_cbc2(arg),
            0xcbc3 => self.op_cbc3(arg),
            0xcbc4 => self.op_cbc4(arg),
            0xcbc5 => self.op_cbc5(arg),
            0xcbc6 => self.op_cbc6(arg),
            0xcbc7 => self.op_cbc7(arg),
            0xcbc8 => self.op_cbc8(arg),
            0xcbc9 => self.op_cbc9(arg),
            0xcbca => self.op_cbca(arg),
            0xcbcb => self.op_cbcb(arg),
            0xcbcc => self.op_cbcc(arg),
            0xcbcd => self.op_cbcd(arg),
            0xcbce => self.op_cbce(arg),
            0xcbcf => self.op_cbcf(arg),
            0xcbd0 => self.op_cbd0(arg),
            0xcbd1 => self.op_cbd1(arg),
            0xcbd2 => self.op_cbd2(arg),
            0xcbd3 => self.op_cbd3(arg),
            0xcbd4 => self.op_cbd4(arg),
            0xcbd5 => self.op_cbd5(arg),
            0xcbd6 => self.op_cbd6(arg),
            0xcbd7 => self.op_cbd7(arg),
            0xcbd8 => self.op_cbd8(arg),
            0xcbd9 => self.op_cbd9(arg),
            0xcbda => self.op_cbda(arg),
            0xcbdb => self.op_cbdb(arg),
            0xcbdc => self.op_cbdc(arg),
            0xcbdd => self.op_cbdd(arg),
            0xcbde => self.op_cbde(arg),
            0xcbdf => self.op_cbdf(arg),
            0xcbe0 => self.op_cbe0(arg),
            0xcbe1 => self.op_cbe1(arg),
            0xcbe2 => self.op_cbe2(arg),
            0xcbe3 => self.op_cbe3(arg),
            0xcbe4 => self.op_cbe4(arg),
            0xcbe5 => self.op_cbe5(arg),
            0xcbe6 => self.op_cbe6(arg),
            0xcbe7 => self.op_cbe7(arg),
            0xcbe8 => self.op_cbe8(arg),
            0xcbe9 => self.op_cbe9(arg),
            0xcbea => self.op_cbea(arg),
            0xcbeb => self.op_cbeb(arg),
            0xcbec => self.op_cbec(arg),
            0xcbed => self.op_cbed(arg),
            0xcbee => self.op_cbee(arg),
            0xcbef => self.op_cbef(arg),
            0xcbf0 => self.op_cbf0(arg),
            0xcbf1 => self.op_cbf1(arg),
            0xcbf2 => self.op_cbf2(arg),
            0xcbf3 => self.op_cbf3(arg),
            0xcbf4 => self.op_cbf4(arg),
            0xcbf5 => self.op_cbf5(arg),
            0xcbf6 => self.op_cbf6(arg),
            0xcbf7 => self.op_cbf7(arg),
            0xcbf8 => self.op_cbf8(arg),
            0xcbf9 => self.op_cbf9(arg),
            0xcbfa => self.op_cbfa(arg),
            0xcbfb => self.op_cbfb(arg),
            0xcbfc => self.op_cbfc(arg),
            0xcbfd => self.op_cbfd(arg),
            0xcbfe => self.op_cbfe(arg),
            0xcbff => self.op_cbff(arg),
            _ => panic!("Invalid opcode: {:04x}: {:04x}", self.get_pc(), code),
        };

        // Every instruction consumes at least 4 cycles.
        self.step(4);

        (time, size)
    }
}
