


fn op_0000(cpu: &Cpu) {}

fn op_0001(cpu: &Cpu) {
  let v = mmu.get16(cpu.get_pc().wrapping_add(1));
  cpu.set_bc(v.into());
}

fn op_0002(cpu: &Cpu) {
  let v = cpu.get_a();
  mmu.set8(cpu.get_bc(), v.into());
}

fn op_0003(cpu: &Cpu) {}

fn op_0004(cpu: &Cpu) {}

fn op_0005(cpu: &Cpu) {}

fn op_0006(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_pc().wrapping_add(1));
  cpu.set_b(v.into());
}

fn op_0007(cpu: &Cpu) {}

fn op_0008(cpu: &Cpu) {
  let v = cpu.get_sp();
  mmu.set16(mmu.get16(cpu.get_pc().wrapping_add(1)), v.into());
}

fn op_0009(cpu: &Cpu) {}

fn op_000a(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_bc());
  cpu.set_a(v.into());
}

fn op_000b(cpu: &Cpu) {}

fn op_000c(cpu: &Cpu) {}

fn op_000d(cpu: &Cpu) {}

fn op_000e(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_pc().wrapping_add(1));
  cpu.set_c(v.into());
}

fn op_000f(cpu: &Cpu) {}

fn op_0010(cpu: &Cpu) {}

fn op_0011(cpu: &Cpu) {
  let v = mmu.get16(cpu.get_pc().wrapping_add(1));
  cpu.set_de(v.into());
}

fn op_0012(cpu: &Cpu) {
  let v = cpu.get_a();
  mmu.set8(cpu.get_de(), v.into());
}

fn op_0013(cpu: &Cpu) {}

fn op_0014(cpu: &Cpu) {}

fn op_0015(cpu: &Cpu) {}

fn op_0016(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_pc().wrapping_add(1));
  cpu.set_d(v.into());
}

fn op_0017(cpu: &Cpu) {}

fn op_0018(cpu: &Cpu) {}

fn op_0019(cpu: &Cpu) {}

fn op_001a(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_de());
  cpu.set_a(v.into());
}

fn op_001b(cpu: &Cpu) {}

fn op_001c(cpu: &Cpu) {}

fn op_001d(cpu: &Cpu) {}

fn op_001e(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_pc().wrapping_add(1));
  cpu.set_e(v.into());
}

fn op_001f(cpu: &Cpu) {}

fn op_0020(cpu: &Cpu) {}

fn op_0021(cpu: &Cpu) {
  let v = mmu.get16(cpu.get_pc().wrapping_add(1));
  cpu.set_hl(v.into());
}

fn op_0022(cpu: &Cpu) {}

fn op_0023(cpu: &Cpu) {}

fn op_0024(cpu: &Cpu) {}

fn op_0025(cpu: &Cpu) {}

fn op_0026(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_pc().wrapping_add(1));
  cpu.set_h(v.into());
}

fn op_0027(cpu: &Cpu) {}

fn op_0028(cpu: &Cpu) {}

fn op_0029(cpu: &Cpu) {}

fn op_002a(cpu: &Cpu) {}

fn op_002b(cpu: &Cpu) {}

fn op_002c(cpu: &Cpu) {}

fn op_002d(cpu: &Cpu) {}

fn op_002e(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_pc().wrapping_add(1));
  cpu.set_l(v.into());
}

fn op_002f(cpu: &Cpu) {}

fn op_0030(cpu: &Cpu) {}

fn op_0031(cpu: &Cpu) {
  let v = mmu.get16(cpu.get_pc().wrapping_add(1));
  cpu.set_sp(v.into());
}

fn op_0032(cpu: &Cpu) {}

fn op_0033(cpu: &Cpu) {}

fn op_0034(cpu: &Cpu) {}

fn op_0035(cpu: &Cpu) {}

fn op_0036(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_pc().wrapping_add(1));
  mmu.set8(cpu.get_hl(), v.into());
}

fn op_0037(cpu: &Cpu) {}

fn op_0038(cpu: &Cpu) {}

fn op_0039(cpu: &Cpu) {}

fn op_003a(cpu: &Cpu) {}

fn op_003b(cpu: &Cpu) {}

fn op_003c(cpu: &Cpu) {}

fn op_003d(cpu: &Cpu) {}

fn op_003e(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_pc().wrapping_add(1));
  cpu.set_a(v.into());
}

fn op_003f(cpu: &Cpu) {}

fn op_0040(cpu: &Cpu) {
  let v = cpu.get_b();
  cpu.set_b(v.into());
}

fn op_0041(cpu: &Cpu) {
  let v = cpu.get_c();
  cpu.set_b(v.into());
}

fn op_0042(cpu: &Cpu) {
  let v = cpu.get_d();
  cpu.set_b(v.into());
}

fn op_0043(cpu: &Cpu) {
  let v = cpu.get_e();
  cpu.set_b(v.into());
}

fn op_0044(cpu: &Cpu) {
  let v = cpu.get_h();
  cpu.set_b(v.into());
}

fn op_0045(cpu: &Cpu) {
  let v = cpu.get_l();
  cpu.set_b(v.into());
}

fn op_0046(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_hl());
  cpu.set_b(v.into());
}

fn op_0047(cpu: &Cpu) {
  let v = cpu.get_a();
  cpu.set_b(v.into());
}

fn op_0048(cpu: &Cpu) {
  let v = cpu.get_b();
  cpu.set_c(v.into());
}

fn op_0049(cpu: &Cpu) {
  let v = cpu.get_c();
  cpu.set_c(v.into());
}

fn op_004a(cpu: &Cpu) {
  let v = cpu.get_d();
  cpu.set_c(v.into());
}

fn op_004b(cpu: &Cpu) {
  let v = cpu.get_e();
  cpu.set_c(v.into());
}

fn op_004c(cpu: &Cpu) {
  let v = cpu.get_h();
  cpu.set_c(v.into());
}

fn op_004d(cpu: &Cpu) {
  let v = cpu.get_l();
  cpu.set_c(v.into());
}

fn op_004e(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_hl());
  cpu.set_c(v.into());
}

fn op_004f(cpu: &Cpu) {
  let v = cpu.get_a();
  cpu.set_c(v.into());
}

fn op_0050(cpu: &Cpu) {
  let v = cpu.get_b();
  cpu.set_d(v.into());
}

fn op_0051(cpu: &Cpu) {
  let v = cpu.get_c();
  cpu.set_d(v.into());
}

fn op_0052(cpu: &Cpu) {
  let v = cpu.get_d();
  cpu.set_d(v.into());
}

fn op_0053(cpu: &Cpu) {
  let v = cpu.get_e();
  cpu.set_d(v.into());
}

fn op_0054(cpu: &Cpu) {
  let v = cpu.get_h();
  cpu.set_d(v.into());
}

fn op_0055(cpu: &Cpu) {
  let v = cpu.get_l();
  cpu.set_d(v.into());
}

fn op_0056(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_hl());
  cpu.set_d(v.into());
}

fn op_0057(cpu: &Cpu) {
  let v = cpu.get_a();
  cpu.set_d(v.into());
}

fn op_0058(cpu: &Cpu) {
  let v = cpu.get_b();
  cpu.set_e(v.into());
}

fn op_0059(cpu: &Cpu) {
  let v = cpu.get_c();
  cpu.set_e(v.into());
}

fn op_005a(cpu: &Cpu) {
  let v = cpu.get_d();
  cpu.set_e(v.into());
}

fn op_005b(cpu: &Cpu) {
  let v = cpu.get_e();
  cpu.set_e(v.into());
}

fn op_005c(cpu: &Cpu) {
  let v = cpu.get_h();
  cpu.set_e(v.into());
}

fn op_005d(cpu: &Cpu) {
  let v = cpu.get_l();
  cpu.set_e(v.into());
}

fn op_005e(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_hl());
  cpu.set_e(v.into());
}

fn op_005f(cpu: &Cpu) {
  let v = cpu.get_a();
  cpu.set_e(v.into());
}

fn op_0060(cpu: &Cpu) {
  let v = cpu.get_b();
  cpu.set_h(v.into());
}

fn op_0061(cpu: &Cpu) {
  let v = cpu.get_c();
  cpu.set_h(v.into());
}

fn op_0062(cpu: &Cpu) {
  let v = cpu.get_d();
  cpu.set_h(v.into());
}

fn op_0063(cpu: &Cpu) {
  let v = cpu.get_e();
  cpu.set_h(v.into());
}

fn op_0064(cpu: &Cpu) {
  let v = cpu.get_h();
  cpu.set_h(v.into());
}

fn op_0065(cpu: &Cpu) {
  let v = cpu.get_l();
  cpu.set_h(v.into());
}

fn op_0066(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_hl());
  cpu.set_h(v.into());
}

fn op_0067(cpu: &Cpu) {
  let v = cpu.get_a();
  cpu.set_h(v.into());
}

fn op_0068(cpu: &Cpu) {
  let v = cpu.get_b();
  cpu.set_l(v.into());
}

fn op_0069(cpu: &Cpu) {
  let v = cpu.get_c();
  cpu.set_l(v.into());
}

fn op_006a(cpu: &Cpu) {
  let v = cpu.get_d();
  cpu.set_l(v.into());
}

fn op_006b(cpu: &Cpu) {
  let v = cpu.get_e();
  cpu.set_l(v.into());
}

fn op_006c(cpu: &Cpu) {
  let v = cpu.get_h();
  cpu.set_l(v.into());
}

fn op_006d(cpu: &Cpu) {
  let v = cpu.get_l();
  cpu.set_l(v.into());
}

fn op_006e(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_hl());
  cpu.set_l(v.into());
}

fn op_006f(cpu: &Cpu) {
  let v = cpu.get_a();
  cpu.set_l(v.into());
}

fn op_0070(cpu: &Cpu) {
  let v = cpu.get_b();
  mmu.set8(cpu.get_hl(), v.into());
}

fn op_0071(cpu: &Cpu) {
  let v = cpu.get_c();
  mmu.set8(cpu.get_hl(), v.into());
}

fn op_0072(cpu: &Cpu) {
  let v = cpu.get_d();
  mmu.set8(cpu.get_hl(), v.into());
}

fn op_0073(cpu: &Cpu) {
  let v = cpu.get_e();
  mmu.set8(cpu.get_hl(), v.into());
}

fn op_0074(cpu: &Cpu) {
  let v = cpu.get_h();
  mmu.set8(cpu.get_hl(), v.into());
}

fn op_0075(cpu: &Cpu) {
  let v = cpu.get_l();
  mmu.set8(cpu.get_hl(), v.into());
}

fn op_0076(cpu: &Cpu) {}

fn op_0077(cpu: &Cpu) {
  let v = cpu.get_a();
  mmu.set8(cpu.get_hl(), v.into());
}

fn op_0078(cpu: &Cpu) {
  let v = cpu.get_b();
  cpu.set_a(v.into());
}

fn op_0079(cpu: &Cpu) {
  let v = cpu.get_c();
  cpu.set_a(v.into());
}

fn op_007a(cpu: &Cpu) {
  let v = cpu.get_d();
  cpu.set_a(v.into());
}

fn op_007b(cpu: &Cpu) {
  let v = cpu.get_e();
  cpu.set_a(v.into());
}

fn op_007c(cpu: &Cpu) {
  let v = cpu.get_h();
  cpu.set_a(v.into());
}

fn op_007d(cpu: &Cpu) {
  let v = cpu.get_l();
  cpu.set_a(v.into());
}

fn op_007e(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_hl());
  cpu.set_a(v.into());
}

fn op_007f(cpu: &Cpu) {
  let v = cpu.get_a();
  cpu.set_a(v.into());
}

fn op_0080(cpu: &Cpu) {}

fn op_0081(cpu: &Cpu) {}

fn op_0082(cpu: &Cpu) {}

fn op_0083(cpu: &Cpu) {}

fn op_0084(cpu: &Cpu) {}

fn op_0085(cpu: &Cpu) {}

fn op_0086(cpu: &Cpu) {}

fn op_0087(cpu: &Cpu) {}

fn op_0088(cpu: &Cpu) {}

fn op_0089(cpu: &Cpu) {}

fn op_008a(cpu: &Cpu) {}

fn op_008b(cpu: &Cpu) {}

fn op_008c(cpu: &Cpu) {}

fn op_008d(cpu: &Cpu) {}

fn op_008e(cpu: &Cpu) {}

fn op_008f(cpu: &Cpu) {}

fn op_0090(cpu: &Cpu) {}

fn op_0091(cpu: &Cpu) {}

fn op_0092(cpu: &Cpu) {}

fn op_0093(cpu: &Cpu) {}

fn op_0094(cpu: &Cpu) {}

fn op_0095(cpu: &Cpu) {}

fn op_0096(cpu: &Cpu) {}

fn op_0097(cpu: &Cpu) {}

fn op_0098(cpu: &Cpu) {}

fn op_0099(cpu: &Cpu) {}

fn op_009a(cpu: &Cpu) {}

fn op_009b(cpu: &Cpu) {}

fn op_009c(cpu: &Cpu) {}

fn op_009d(cpu: &Cpu) {}

fn op_009e(cpu: &Cpu) {}

fn op_009f(cpu: &Cpu) {}

fn op_00a0(cpu: &Cpu) {}

fn op_00a1(cpu: &Cpu) {}

fn op_00a2(cpu: &Cpu) {}

fn op_00a3(cpu: &Cpu) {}

fn op_00a4(cpu: &Cpu) {}

fn op_00a5(cpu: &Cpu) {}

fn op_00a6(cpu: &Cpu) {}

fn op_00a7(cpu: &Cpu) {}

fn op_00a8(cpu: &Cpu) {}

fn op_00a9(cpu: &Cpu) {}

fn op_00aa(cpu: &Cpu) {}

fn op_00ab(cpu: &Cpu) {}

fn op_00ac(cpu: &Cpu) {}

fn op_00ad(cpu: &Cpu) {}

fn op_00ae(cpu: &Cpu) {}

fn op_00af(cpu: &Cpu) {}

fn op_00b0(cpu: &Cpu) {}

fn op_00b1(cpu: &Cpu) {}

fn op_00b2(cpu: &Cpu) {}

fn op_00b3(cpu: &Cpu) {}

fn op_00b4(cpu: &Cpu) {}

fn op_00b5(cpu: &Cpu) {}

fn op_00b6(cpu: &Cpu) {}

fn op_00b7(cpu: &Cpu) {}

fn op_00b8(cpu: &Cpu) {}

fn op_00b9(cpu: &Cpu) {}

fn op_00ba(cpu: &Cpu) {}

fn op_00bb(cpu: &Cpu) {}

fn op_00bc(cpu: &Cpu) {}

fn op_00bd(cpu: &Cpu) {}

fn op_00be(cpu: &Cpu) {}

fn op_00bf(cpu: &Cpu) {}

fn op_00c0(cpu: &Cpu) {}

fn op_00c1(cpu: &Cpu) {}

fn op_00c2(cpu: &Cpu) {}

fn op_00c3(cpu: &Cpu) {}

fn op_00c4(cpu: &Cpu) {}

fn op_00c5(cpu: &Cpu) {}

fn op_00c6(cpu: &Cpu) {}

fn op_00c7(cpu: &Cpu) {}

fn op_00c8(cpu: &Cpu) {}

fn op_00c9(cpu: &Cpu) {}

fn op_00ca(cpu: &Cpu) {}

fn op_00cb(cpu: &Cpu) {}

fn op_00cc(cpu: &Cpu) {}

fn op_00cd(cpu: &Cpu) {}

fn op_00ce(cpu: &Cpu) {}

fn op_00cf(cpu: &Cpu) {}

fn op_00d0(cpu: &Cpu) {}

fn op_00d1(cpu: &Cpu) {}

fn op_00d2(cpu: &Cpu) {}

fn op_00d4(cpu: &Cpu) {}

fn op_00d5(cpu: &Cpu) {}

fn op_00d6(cpu: &Cpu) {}

fn op_00d7(cpu: &Cpu) {}

fn op_00d8(cpu: &Cpu) {}

fn op_00d9(cpu: &Cpu) {}

fn op_00da(cpu: &Cpu) {}

fn op_00dc(cpu: &Cpu) {}

fn op_00de(cpu: &Cpu) {}

fn op_00df(cpu: &Cpu) {}

fn op_00e0(cpu: &Cpu) {
  let v = cpu.get_a();
  mmu.set8(cpu.get_0xff00+a8(), v.into());
}

fn op_00e1(cpu: &Cpu) {}

fn op_00e2(cpu: &Cpu) {
  let v = cpu.get_a();
  mmu.set8(cpu.get_0xff00+c(), v.into());
}

fn op_00e5(cpu: &Cpu) {}

fn op_00e6(cpu: &Cpu) {}

fn op_00e7(cpu: &Cpu) {}

fn op_00e8(cpu: &Cpu) {}

fn op_00e9(cpu: &Cpu) {}

fn op_00ea(cpu: &Cpu) {
  let v = cpu.get_a();
  mmu.set8(mmu.get16(cpu.get_pc().wrapping_add(1)), v.into());
}

fn op_00ee(cpu: &Cpu) {}

fn op_00ef(cpu: &Cpu) {}

fn op_00f0(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_0xff00+a8());
  cpu.set_a(v.into());
}

fn op_00f1(cpu: &Cpu) {}

fn op_00f2(cpu: &Cpu) {
  let v = mmu.get8(cpu.get_0xff00+c());
  cpu.set_a(v.into());
}

fn op_00f3(cpu: &Cpu) {}

fn op_00f5(cpu: &Cpu) {}

fn op_00f6(cpu: &Cpu) {}

fn op_00f7(cpu: &Cpu) {}

fn op_00f8(cpu: &Cpu) {}

fn op_00f9(cpu: &Cpu) {
  let v = cpu.get_hl();
  cpu.set_sp(v.into());
}

fn op_00fa(cpu: &Cpu) {
  let v = mmu.get8(mmu.get16(cpu.get_pc().wrapping_add(1)));
  cpu.set_a(v.into());
}

fn op_00fb(cpu: &Cpu) {}

fn op_00fe(cpu: &Cpu) {}

fn op_00ff(cpu: &Cpu) {}

fn op_cb00(cpu: &Cpu) {}

fn op_cb01(cpu: &Cpu) {}

fn op_cb02(cpu: &Cpu) {}

fn op_cb03(cpu: &Cpu) {}

fn op_cb04(cpu: &Cpu) {}

fn op_cb05(cpu: &Cpu) {}

fn op_cb06(cpu: &Cpu) {}

fn op_cb07(cpu: &Cpu) {}

fn op_cb08(cpu: &Cpu) {}

fn op_cb09(cpu: &Cpu) {}

fn op_cb0a(cpu: &Cpu) {}

fn op_cb0b(cpu: &Cpu) {}

fn op_cb0c(cpu: &Cpu) {}

fn op_cb0d(cpu: &Cpu) {}

fn op_cb0e(cpu: &Cpu) {}

fn op_cb0f(cpu: &Cpu) {}

fn op_cb10(cpu: &Cpu) {}

fn op_cb11(cpu: &Cpu) {}

fn op_cb12(cpu: &Cpu) {}

fn op_cb13(cpu: &Cpu) {}

fn op_cb14(cpu: &Cpu) {}

fn op_cb15(cpu: &Cpu) {}

fn op_cb16(cpu: &Cpu) {}

fn op_cb17(cpu: &Cpu) {}

fn op_cb18(cpu: &Cpu) {}

fn op_cb19(cpu: &Cpu) {}

fn op_cb1a(cpu: &Cpu) {}

fn op_cb1b(cpu: &Cpu) {}

fn op_cb1c(cpu: &Cpu) {}

fn op_cb1d(cpu: &Cpu) {}

fn op_cb1e(cpu: &Cpu) {}

fn op_cb1f(cpu: &Cpu) {}

fn op_cb20(cpu: &Cpu) {}

fn op_cb21(cpu: &Cpu) {}

fn op_cb22(cpu: &Cpu) {}

fn op_cb23(cpu: &Cpu) {}

fn op_cb24(cpu: &Cpu) {}

fn op_cb25(cpu: &Cpu) {}

fn op_cb26(cpu: &Cpu) {}

fn op_cb27(cpu: &Cpu) {}

fn op_cb28(cpu: &Cpu) {}

fn op_cb29(cpu: &Cpu) {}

fn op_cb2a(cpu: &Cpu) {}

fn op_cb2b(cpu: &Cpu) {}

fn op_cb2c(cpu: &Cpu) {}

fn op_cb2d(cpu: &Cpu) {}

fn op_cb2e(cpu: &Cpu) {}

fn op_cb2f(cpu: &Cpu) {}

fn op_cb30(cpu: &Cpu) {}

fn op_cb31(cpu: &Cpu) {}

fn op_cb32(cpu: &Cpu) {}

fn op_cb33(cpu: &Cpu) {}

fn op_cb34(cpu: &Cpu) {}

fn op_cb35(cpu: &Cpu) {}

fn op_cb36(cpu: &Cpu) {}

fn op_cb37(cpu: &Cpu) {}

fn op_cb38(cpu: &Cpu) {}

fn op_cb39(cpu: &Cpu) {}

fn op_cb3a(cpu: &Cpu) {}

fn op_cb3b(cpu: &Cpu) {}

fn op_cb3c(cpu: &Cpu) {}

fn op_cb3d(cpu: &Cpu) {}

fn op_cb3e(cpu: &Cpu) {}

fn op_cb3f(cpu: &Cpu) {}

fn op_cb40(cpu: &Cpu) {}

fn op_cb41(cpu: &Cpu) {}

fn op_cb42(cpu: &Cpu) {}

fn op_cb43(cpu: &Cpu) {}

fn op_cb44(cpu: &Cpu) {}

fn op_cb45(cpu: &Cpu) {}

fn op_cb46(cpu: &Cpu) {}

fn op_cb47(cpu: &Cpu) {}

fn op_cb48(cpu: &Cpu) {}

fn op_cb49(cpu: &Cpu) {}

fn op_cb4a(cpu: &Cpu) {}

fn op_cb4b(cpu: &Cpu) {}

fn op_cb4c(cpu: &Cpu) {}

fn op_cb4d(cpu: &Cpu) {}

fn op_cb4e(cpu: &Cpu) {}

fn op_cb4f(cpu: &Cpu) {}

fn op_cb50(cpu: &Cpu) {}

fn op_cb51(cpu: &Cpu) {}

fn op_cb52(cpu: &Cpu) {}

fn op_cb53(cpu: &Cpu) {}

fn op_cb54(cpu: &Cpu) {}

fn op_cb55(cpu: &Cpu) {}

fn op_cb56(cpu: &Cpu) {}

fn op_cb57(cpu: &Cpu) {}

fn op_cb58(cpu: &Cpu) {}

fn op_cb59(cpu: &Cpu) {}

fn op_cb5a(cpu: &Cpu) {}

fn op_cb5b(cpu: &Cpu) {}

fn op_cb5c(cpu: &Cpu) {}

fn op_cb5d(cpu: &Cpu) {}

fn op_cb5e(cpu: &Cpu) {}

fn op_cb5f(cpu: &Cpu) {}

fn op_cb60(cpu: &Cpu) {}

fn op_cb61(cpu: &Cpu) {}

fn op_cb62(cpu: &Cpu) {}

fn op_cb63(cpu: &Cpu) {}

fn op_cb64(cpu: &Cpu) {}

fn op_cb65(cpu: &Cpu) {}

fn op_cb66(cpu: &Cpu) {}

fn op_cb67(cpu: &Cpu) {}

fn op_cb68(cpu: &Cpu) {}

fn op_cb69(cpu: &Cpu) {}

fn op_cb6a(cpu: &Cpu) {}

fn op_cb6b(cpu: &Cpu) {}

fn op_cb6c(cpu: &Cpu) {}

fn op_cb6d(cpu: &Cpu) {}

fn op_cb6e(cpu: &Cpu) {}

fn op_cb6f(cpu: &Cpu) {}

fn op_cb70(cpu: &Cpu) {}

fn op_cb71(cpu: &Cpu) {}

fn op_cb72(cpu: &Cpu) {}

fn op_cb73(cpu: &Cpu) {}

fn op_cb74(cpu: &Cpu) {}

fn op_cb75(cpu: &Cpu) {}

fn op_cb76(cpu: &Cpu) {}

fn op_cb77(cpu: &Cpu) {}

fn op_cb78(cpu: &Cpu) {}

fn op_cb79(cpu: &Cpu) {}

fn op_cb7a(cpu: &Cpu) {}

fn op_cb7b(cpu: &Cpu) {}

fn op_cb7c(cpu: &Cpu) {}

fn op_cb7d(cpu: &Cpu) {}

fn op_cb7e(cpu: &Cpu) {}

fn op_cb7f(cpu: &Cpu) {}

fn op_cb80(cpu: &Cpu) {}

fn op_cb81(cpu: &Cpu) {}

fn op_cb82(cpu: &Cpu) {}

fn op_cb83(cpu: &Cpu) {}

fn op_cb84(cpu: &Cpu) {}

fn op_cb85(cpu: &Cpu) {}

fn op_cb86(cpu: &Cpu) {}

fn op_cb87(cpu: &Cpu) {}

fn op_cb88(cpu: &Cpu) {}

fn op_cb89(cpu: &Cpu) {}

fn op_cb8a(cpu: &Cpu) {}

fn op_cb8b(cpu: &Cpu) {}

fn op_cb8c(cpu: &Cpu) {}

fn op_cb8d(cpu: &Cpu) {}

fn op_cb8e(cpu: &Cpu) {}

fn op_cb8f(cpu: &Cpu) {}

fn op_cb90(cpu: &Cpu) {}

fn op_cb91(cpu: &Cpu) {}

fn op_cb92(cpu: &Cpu) {}

fn op_cb93(cpu: &Cpu) {}

fn op_cb94(cpu: &Cpu) {}

fn op_cb95(cpu: &Cpu) {}

fn op_cb96(cpu: &Cpu) {}

fn op_cb97(cpu: &Cpu) {}

fn op_cb98(cpu: &Cpu) {}

fn op_cb99(cpu: &Cpu) {}

fn op_cb9a(cpu: &Cpu) {}

fn op_cb9b(cpu: &Cpu) {}

fn op_cb9c(cpu: &Cpu) {}

fn op_cb9d(cpu: &Cpu) {}

fn op_cb9e(cpu: &Cpu) {}

fn op_cb9f(cpu: &Cpu) {}

fn op_cba0(cpu: &Cpu) {}

fn op_cba1(cpu: &Cpu) {}

fn op_cba2(cpu: &Cpu) {}

fn op_cba3(cpu: &Cpu) {}

fn op_cba4(cpu: &Cpu) {}

fn op_cba5(cpu: &Cpu) {}

fn op_cba6(cpu: &Cpu) {}

fn op_cba7(cpu: &Cpu) {}

fn op_cba8(cpu: &Cpu) {}

fn op_cba9(cpu: &Cpu) {}

fn op_cbaa(cpu: &Cpu) {}

fn op_cbab(cpu: &Cpu) {}

fn op_cbac(cpu: &Cpu) {}

fn op_cbad(cpu: &Cpu) {}

fn op_cbae(cpu: &Cpu) {}

fn op_cbaf(cpu: &Cpu) {}

fn op_cbb0(cpu: &Cpu) {}

fn op_cbb1(cpu: &Cpu) {}

fn op_cbb2(cpu: &Cpu) {}

fn op_cbb3(cpu: &Cpu) {}

fn op_cbb4(cpu: &Cpu) {}

fn op_cbb5(cpu: &Cpu) {}

fn op_cbb6(cpu: &Cpu) {}

fn op_cbb7(cpu: &Cpu) {}

fn op_cbb8(cpu: &Cpu) {}

fn op_cbb9(cpu: &Cpu) {}

fn op_cbba(cpu: &Cpu) {}

fn op_cbbb(cpu: &Cpu) {}

fn op_cbbc(cpu: &Cpu) {}

fn op_cbbd(cpu: &Cpu) {}

fn op_cbbe(cpu: &Cpu) {}

fn op_cbbf(cpu: &Cpu) {}

fn op_cbc0(cpu: &Cpu) {}

fn op_cbc1(cpu: &Cpu) {}

fn op_cbc2(cpu: &Cpu) {}

fn op_cbc3(cpu: &Cpu) {}

fn op_cbc4(cpu: &Cpu) {}

fn op_cbc5(cpu: &Cpu) {}

fn op_cbc6(cpu: &Cpu) {}

fn op_cbc7(cpu: &Cpu) {}

fn op_cbc8(cpu: &Cpu) {}

fn op_cbc9(cpu: &Cpu) {}

fn op_cbca(cpu: &Cpu) {}

fn op_cbcb(cpu: &Cpu) {}

fn op_cbcc(cpu: &Cpu) {}

fn op_cbcd(cpu: &Cpu) {}

fn op_cbce(cpu: &Cpu) {}

fn op_cbcf(cpu: &Cpu) {}

fn op_cbd0(cpu: &Cpu) {}

fn op_cbd1(cpu: &Cpu) {}

fn op_cbd2(cpu: &Cpu) {}

fn op_cbd3(cpu: &Cpu) {}

fn op_cbd4(cpu: &Cpu) {}

fn op_cbd5(cpu: &Cpu) {}

fn op_cbd6(cpu: &Cpu) {}

fn op_cbd7(cpu: &Cpu) {}

fn op_cbd8(cpu: &Cpu) {}

fn op_cbd9(cpu: &Cpu) {}

fn op_cbda(cpu: &Cpu) {}

fn op_cbdb(cpu: &Cpu) {}

fn op_cbdc(cpu: &Cpu) {}

fn op_cbdd(cpu: &Cpu) {}

fn op_cbde(cpu: &Cpu) {}

fn op_cbdf(cpu: &Cpu) {}

fn op_cbe0(cpu: &Cpu) {}

fn op_cbe1(cpu: &Cpu) {}

fn op_cbe2(cpu: &Cpu) {}

fn op_cbe3(cpu: &Cpu) {}

fn op_cbe4(cpu: &Cpu) {}

fn op_cbe5(cpu: &Cpu) {}

fn op_cbe6(cpu: &Cpu) {}

fn op_cbe7(cpu: &Cpu) {}

fn op_cbe8(cpu: &Cpu) {}

fn op_cbe9(cpu: &Cpu) {}

fn op_cbea(cpu: &Cpu) {}

fn op_cbeb(cpu: &Cpu) {}

fn op_cbec(cpu: &Cpu) {}

fn op_cbed(cpu: &Cpu) {}

fn op_cbee(cpu: &Cpu) {}

fn op_cbef(cpu: &Cpu) {}

fn op_cbf0(cpu: &Cpu) {}

fn op_cbf1(cpu: &Cpu) {}

fn op_cbf2(cpu: &Cpu) {}

fn op_cbf3(cpu: &Cpu) {}

fn op_cbf4(cpu: &Cpu) {}

fn op_cbf5(cpu: &Cpu) {}

fn op_cbf6(cpu: &Cpu) {}

fn op_cbf7(cpu: &Cpu) {}

fn op_cbf8(cpu: &Cpu) {}

fn op_cbf9(cpu: &Cpu) {}

fn op_cbfa(cpu: &Cpu) {}

fn op_cbfb(cpu: &Cpu) {}

fn op_cbfc(cpu: &Cpu) {}

fn op_cbfd(cpu: &Cpu) {}

fn op_cbfe(cpu: &Cpu) {}

fn op_cbff(cpu: &Cpu) {}

