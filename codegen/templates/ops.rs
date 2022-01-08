{% macro nop(i) %}
{% endmacro %}

{% macro inc16(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }}.wrapping_add(1);
  {{ i.operands[0] | setter(bits=i.bits) }}v);
  self.step(4);
{% endmacro %}

{% macro dec16(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }}.wrapping_sub(1);
  {{ i.operands[0] | setter(bits=i.bits) }}v);
  self.step(4);
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
  {% if i.operands[0] == "sp" and i.operands[1] == "hl" %}
  self.step(4);
  {% endif %}
{% endmacro %}

{% macro ldhl(i) %}
  let p = {{ i.operands[0] | getter(bits=i.bits) }};
  let q = {{ i.operands[1] | getter(bits=i.bits) }};
  let (v, h, c, z) = alu::add16e(p, q, false);
  self.set_hl(v);
  self.step(4);
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
  self.step(4);
{% endmacro %}

{% macro addsp(i) %}
  let p = {{ i.operands[0] | getter(bits=i.bits) }};
  let q = {{ i.operands[1] | getter(bits=i.bits) }};
  let (v, h, c, z) = alu::add16e(p, q, false);
  {{ i.operands[0] | setter(bits=i.bits) }}v);
  self.step(8);
{% endmacro %}

{% macro sub(i) %}
  let p = self.get_a();
  let q = {{ i.operands[0] | getter(bits=i.bits) }};
  let (v, h, c, z) = alu::sub8(p, q, false);
  self.set_a(v);
{% endmacro %}

{% macro adc(i) %}
  let p = self.get_a();
  let q = {{ i.operands[1] | getter(bits=i.bits) }};
  let (v, h, c, z) = alu::add8(p, q, self.get_cf());
  self.set_a(v);
{% endmacro %}

{% macro sbc(i) %}
  let p = self.get_a();
  let q = {{ i.operands[1] | getter(bits=i.bits) }};
  let (v, h, c, z) = alu::sub8(p, q, self.get_cf());
  self.set_a(v);
{% endmacro %}

{% macro and(i) %}
  let v = self.get_a() & {{ i.operands[0] | getter(bits=i.bits) }};
  self.set_a(v);
  let z = self.get_a() == 0;
{% endmacro %}

{% macro or(i) %}
  let v = self.get_a() | {{ i.operands[0] | getter(bits=i.bits) }};
  self.set_a(v);
  let z = self.get_a() == 0;
{% endmacro %}

{% macro xor(i) %}
  let v = self.get_a() ^ {{ i.operands[0] | getter(bits=i.bits) }};
  self.set_a(v);
  let z = self.get_a() == 0;
{% endmacro %}

{% macro cp(i) %}
  let p = self.get_a();
  let q = {{ i.operands[0] | getter(bits=i.bits) }};
  let (_, h, c, z) = alu::sub8(p, q, false);
{% endmacro %}

{% macro push(i) %}
  self.push({{ i.operands[0] | getter(bits=i.bits) }});
  self.step(4);
{% endmacro %}

{% macro pop(i) %}
  let v = self.pop();
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
{% endmacro %}

{% macro cpl(i) %}
  self.set_a(self.get_a() ^ 0xff);
{% endmacro %}

{% macro ccf(i) %}
  let c = !self.get_cf();
{% endmacro %}

{% macro scf(i) %}
  self.set_cf(true);
{% endmacro %}

{% macro halt(i) %}
  self.halt();
{% endmacro %}

{% macro stop(i) %}
  self.stop();
{% endmacro %}

{% macro di(i) %}
  self.di();
{% endmacro %}

{% macro ei(i) %}
  self.ei();
{% endmacro %}

{% macro rlc(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }};
  let c = v & 0x80 != 0;
  let v = v.rotate_left(1);
  let z = v == 0;
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro rlca(i) %}
  let v = self.get_a();
  let c = v & 0x80 != 0;
  let v = v.rotate_left(1);
  let z = v == 0;
  self.set_a(v);
{% endmacro %}

{% macro rl(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }};
  let c = v & 0x80 != 0;
  let v = v.wrapping_shl(1);
  let v = v | if self.get_cf() { 1 } else { 0 };
  let z = v == 0;
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro rla(i) %}
  let v = self.get_a();
  let c = v & 0x80 != 0;
  let v = v.wrapping_shl(1);
  let v = v | if self.get_cf() { 1 } else { 0 };
  self.set_a(v);
{% endmacro %}

{% macro rrc(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }};
  let c = v & 1 != 0;
  let v = v.rotate_right(1);
  let z = v == 0;
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro rrca(i) %}
  let v = self.get_a();
  let c = v & 1 != 0;
  let v = v.rotate_right(1);
  let z = v == 0;
  self.set_a(v);
{% endmacro %}

{% macro rr(i) %}
  let v = {{ i.operands[0] | getter(bits=i.bits) }};
  let c = v & 1 != 0;
  let v = v.wrapping_shr(1);
  let v = v | if self.get_cf() { 0x80 } else { 0 };
  let z = v == 0;
  {{ i.operands[0] | setter(bits=i.bits) }}v);
{% endmacro %}

{% macro rra(i) %}
  let v = self.get_a();
  let c = v & 1 != 0;
  let v = v.wrapping_shr(1);
  let v = v | if self.get_cf() { 0x80 } else { 0 };
  self.set_a(v);
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
  let pc = self.get_pc().wrapping_add(alu::signed(p));
  self.jump(pc);
{% endmacro %}

{% macro jrif(i) %}
  let flg = {{ i.operands[0] | getter(bits=i.bits) }};
  let p = {{ i.operands[1] | getter(bits=i.bits) }};
  if flg {
    let pc = self.get_pc().wrapping_add(alu::signed(p));
    self.jump(pc);
    return {{ i.time[0] }}
  }
{% endmacro %}

{% macro jp(i) %}
  let pc = {{ i.operands[0] | getter(bits=16) }};
  {% if i.operands[0] == "hl" %}
  self.set_pc(pc);
  {% else %}
  self.jump(pc);
  {% endif %}
{% endmacro %}

{% macro jpif(i) %}
  let flg = {{ i.operands[0] | getter(bits=16) }};
  let pc = {{ i.operands[1] | getter(bits=i.bits) }};
  if flg {
    self.jump(pc);
    return {{ i.time[0] }}
  }
{% endmacro %}

{% macro call(i) %}
  let pc = {{ i.operands[0] | getter(bits=i.bits) }};
  self.push(self.get_pc());
  self.jump(pc);
{% endmacro %}

{% macro callif(i) %}
  let flg = {{ i.operands[0] | getter(bits=i.bits) }};
  let pc = {{ i.operands[1] | getter(bits=i.bits) }};
  if flg {
    self.push(self.get_pc());
    self.jump(pc);
    return {{ i.time[0] }}
  }
{% endmacro %}

{% macro rst(i) %}
  let pc = {{ i.operands[0] }}u16;
  self.push(self.get_pc());
  self.jump(pc);
{% endmacro %}

{% macro ret(i) %}
  let pc = self.pop();
  self.jump(pc);
{% endmacro %}

{% macro retif(i) %}
  let flg = {{ i.operands[0] | getter(bits=i.bits) }};
  self.step(4);
  if flg {
    let pc = self.pop();
    self.jump(pc);
    return {{ i.time[0] }}
  }
{% endmacro %}

{% macro reti(i) %}
  let pc = self.pop();
  self.jump(pc);
  self.enable_interrupt();
{% endmacro %}
