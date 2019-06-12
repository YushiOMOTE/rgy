{% macro nop(i) %}
{% endmacro %}

{% macro inc16(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }}.wrapping_add(1);
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro dec16(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }}.wrapping_sub(1);
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro inc8(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }};
  let (v, h, c, z) = alu::add8(v, 1, false);
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro dec8(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }};
  let (v, h, c, z) = alu::sub8(v, 1, false);
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro ld(i) %}
  let v = {{ i.operands[1] | getter(bits=i.bits) }};
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro ldhl(i) %}
  let p = {{ i.operands[0] | getter(bits=i.bits) }};
  let q = {{ i.operands[1] | getter(bits=i.bits) }};
  let (v, h, c, z) = alu::add16e(p, q, false);
  cpu.set_hl(v);
{% endmacro %}

{% macro add8(i) %}
  let p = {{ i.operands[0] | getter(bits=i.bits) }};
  let q = {{ i.operands[1] | getter(bits=i.bits) }};
  let (v, h, c, z) = alu::add8(p, q, false);
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro add16(i) %}
  let p = {{ i.operands[0] | getter(bits=i.bits) }};
  let q = {{ i.operands[1] | getter(bits=i.bits) }};
  let (v, h, c, z) = alu::add16(p, q, false);
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro addsp(i) %}
  let p = {{ i.operands[0] | getter(bits=i.bits) }};
  let q = {{ i.operands[1] | getter(bits=i.bits) }};
  let (v, h, c, z) = alu::add16e(p, q, false);
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro sub(i) %}
  let p = cpu.get_a();
  let q = {{ i.operands[0] | getter(bits=i.bits) }};
  let (v, h, c, z) = alu::sub8(p, q, false);
  cpu.set_a(v);
{% endmacro %}

{% macro adc(i) %}
  let p = cpu.get_a();
  let q = {{ i.operands[1] | getter(bits=i.bits) }};
  let (v, h, c, z) = alu::add8(p, q, cpu.get_cf());
  cpu.set_a(v);
{% endmacro %}

{% macro sbc(i) %}
  let p = cpu.get_a();
  let q = {{ i.operands[1] | getter(bits=i.bits) }};
  let (v, h, c, z) = alu::sub8(p, q, cpu.get_cf());
  cpu.set_a(v);
{% endmacro %}

{% macro and(i) %}
  cpu.set_a(cpu.get_a() & {{ i.operands[0] | getter(bits=i.bits) }});
  let z = cpu.get_a() == 0;
{% endmacro %}

{% macro or(i) %}
  cpu.set_a(cpu.get_a() | {{ i.operands[0] | getter(bits=i.bits) }});
  let z = cpu.get_a() == 0;
{% endmacro %}

{% macro xor(i) %}
  cpu.set_a(cpu.get_a() ^ {{ i.operands[0] | getter(bits=i.bits) }});
  let z = cpu.get_a() == 0;
{% endmacro %}

{% macro cp(i) %}
  let p = cpu.get_a();
  let q = {{ i.operands[0] | getter(bits=i.bits) }};
  let (_, h, c, z) = alu::sub8(p, q, false);
{% endmacro %}

{% macro push(i) %}
  cpu.push(mmu, {{ i.operands[0] | getter(bits=i.bits) }});
{% endmacro %}

{% macro pop(i) %}
  let v = cpu.pop(mmu);
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro swap(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }};
  let v = v.rotate_left(4);
  {{ i.operands[0] | setter(bits=i.bits) }}v);
  let z = v == 0;
{% endmacro %}

{% macro daa(i) %}
  let mut adj = 0;

  let v = cpu.get_a() as usize;

  if cpu.get_hf() || (!cpu.get_nf() && (v & 0xf) > 9) {
      adj |= 0x6;
  }

  let c = if cpu.get_cf() || (!cpu.get_nf() && v > 0x99) {
      adj |= 0x60;
      true
  } else {
      false
  };

  let v = if cpu.get_nf() { v - adj } else { v + adj };
  let v = (v & 0xff) as u8;
  let z = v == 0;

  cpu.set_a(v);
{% endmacro %}

{% macro cpl(i) %}
  cpu.set_a(cpu.get_a() ^ 0xff);
{% endmacro %}

{% macro ccf(i) %}
  let c = !cpu.get_cf();
{% endmacro %}

{% macro scf(i) %}
  cpu.set_cf(true);
{% endmacro %}

{% macro halt(i) %}
  cpu.halt();
{% endmacro %}

{% macro stop(i) %}
  cpu.stop();
{% endmacro %}

{% macro di(i) %}
  cpu.disable_interrupt();
{% endmacro %}

{% macro ei(i) %}
  cpu.enable_interrupt();
{% endmacro %}

{% macro rlc(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }};
  let c = v & 0x80 != 0;
  let v = v.rotate_left(1);
  let z = v == 0;
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro rlca(i) %}
  let v = cpu.get_a();
  let c = v & 0x80 != 0;
  let v = v.rotate_left(1);
  let z = v == 0;
  cpu.set_a(v);
{% endmacro %}

{% macro rl(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }};
  let c = v & 0x80 != 0;
  let v = v.wrapping_shl(1);
  let v = v | if cpu.get_cf() { 1 } else { 0 };
  let z = v == 0;
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro rla(i) %}
  let v = cpu.get_a();
  let c = v & 0x80 != 0;
  let v = v.wrapping_shl(1);
  let v = v | if cpu.get_cf() { 1 } else { 0 };
  cpu.set_a(v);
{% endmacro %}

{% macro rrc(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }};
  let c = v & 1 != 0;
  let v = v.rotate_right(1);
  let z = v == 0;
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro rrca(i) %}
  let v = cpu.get_a();
  let c = v & 1 != 0;
  let v = v.rotate_right(1);
  let z = v == 0;
  cpu.set_a(v);
{% endmacro %}

{% macro rr(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }};
  let c = v & 1 != 0;
  let v = v.wrapping_shr(1);
  let v = v | if cpu.get_cf() { 0x80 } else { 0 };
  let z = v == 0;
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro rra(i) %}
  let v = cpu.get_a();
  let c = v & 1 != 0;
  let v = v.wrapping_shr(1);
  let v = v | if cpu.get_cf() { 0x80 } else { 0 };
  cpu.set_a(v);
{% endmacro %}

{% macro sla(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }};
  let c = v & 0x80 != 0;
  let v = v.wrapping_shl(1);
  let z = v == 0;
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro sra(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }};
  let c = v & 1 != 0;
  let msb = v & 0x80;
  let v = v.wrapping_shr(1);
  let v = v | msb;
  let z = v == 0;
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro srl(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }};
  let c = v & 1 != 0;
  let v = v.wrapping_shr(1);
  let z = v == 0;
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro bit(i) %}
  let p = {{ i.operands[0] | getter(bits=i.bits) }};
  let q = {{ i.operands[1] | getter(bits=i.bits) }};
  let z = q & (1 << p) == 0;
{% endmacro %}

{% macro set(i) %}
  let p = {{ i.operands[0] | getter(bits=i.bits) }};
  let q = {{ i.operands[1] | getter(bits=i.bits) }};
  {{ i.operands[1] | setter(bits=i.bits) }}q | (1 << p));
{% endmacro %}

{% macro res(i) %}
  let p = {{ i.operands[0] | getter(bits=i.bits) }};
  let q = {{ i.operands[1] | getter(bits=i.bits) }};
  {{ i.operands[1] | setter(bits=i.bits) }}q & !(1 << p));
{% endmacro %}

{% macro jr(i) %}
  let p = {{ i.operands[0] | getter(bits=i.bits) }};
  let pc = cpu.get_pc().wrapping_add(alu::signed(p));
  cpu.set_pc(pc);
{% endmacro %}

{% macro jrif(i) %}
  let flg = {{ i.operands[0] | getter(bits=i.bits) }};
  if flg {
    let p = {{ i.operands[1] | getter(bits=i.bits) }};
    let pc = cpu.get_pc().wrapping_add(alu::signed(p));
    cpu.set_pc(pc);
    return ({{ i.time[0] }}, {{ i.size }})
  }
{% endmacro %}

{% macro jp(i) %}
  let pc = {{ i.operands[0] | getter(bits=16) }};
  cpu.set_pc(pc.wrapping_sub({{ i.size }}));
{% endmacro %}

{% macro jpif(i) %}
  let flg = {{ i.operands[0] | getter(bits=16) }};
  if flg {
    let pc = {{ i.operands[1] | getter(bits=i.bits) }};
    cpu.set_pc(pc);
    return ({{ i.time[0] }}, 0)
  }
{% endmacro %}


{% macro call(i) %}
  cpu.push(mmu, cpu.get_pc().wrapping_add({{ i.size }}));
  cpu.set_pc({{ i.operands[0] | getter(bits=i.bits) }}.wrapping_sub({{i.size}}));
{% endmacro %}

{% macro callif(i) %}
  let flg = {{ i.operands[0] | getter(bits=i.bits) }};
  if flg {
    cpu.push(mmu, cpu.get_pc().wrapping_add({{ i.size }}));
    cpu.set_pc({{ i.operands[1] | getter(bits=i.bits) }});
    return ({{ i.time[0] }}, 0)
  }
{% endmacro %}

{% macro rst(i) %}
  cpu.push(mmu, cpu.get_pc().wrapping_add({{ i.size }}));
  cpu.set_pc({{ i.operands[0] }}u16.wrapping_sub({{i.size}}));
{% endmacro %}

{% macro ret(i) %}
  let pc = cpu.pop(mmu).wrapping_sub({{i.size}});
  cpu.set_pc(pc);
{% endmacro %}

{% macro retif(i) %}
  let flg = {{ i.operands[0] | getter(bits=i.bits) }};
  if flg {
    let pc = cpu.pop(mmu);
    cpu.set_pc(pc);
    return ({{ i.time[0] }}, 0)
  }
{% endmacro %}

{% macro reti(i) %}
  let pc = cpu.pop(mmu).wrapping_sub({{i.size}});
  cpu.set_pc(pc);
  cpu.enable_interrupt();
{% endmacro %}
