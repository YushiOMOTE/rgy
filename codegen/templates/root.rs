{% import "ops.rs" as macros %}

use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::alu;
use std::collections::HashMap;
use lazy_static::lazy_static;
use log::*;

lazy_static! {
    static ref MNEMONICS: HashMap<u16, &'static str> = {
        let mut m = HashMap::new();
        {%- for i in insts -%}
        m.insert(0x{{i.code|hex}}, "{{i.operator}} {{i.operands|join(sep=",")}}");
        {%- endfor -%}
        m
    };
}

{% for i in insts %}
/// {{i.operator}} {{i.operands | join(sep=",")}}
#[allow(unused_variables)]
fn op_{{i.code | hex}}(arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    {%- if i.operator == "nop" -%}

    {{ macros::nop(i=i) }}

    {%- elif i.operator == "inc" -%}

    {%- if i.bits == 8 -%}
    {{ macros::inc8(i=i) }}
    {%- else -%}
    {{ macros::inc16(i=i) }}
    {%- endif -%}

    {%- elif i.operator == "dec" -%}

    {%- if i.bits == 8 -%}
    {{ macros::dec8(i=i) }}
    {%- else -%}
    {{ macros::dec16(i=i) }}
    {%- endif -%}

    {%- elif i.operator == "ld" -%}

    {{ macros::ld(i=i) }}

    {%- elif i.operator == "ldd" -%}

    {{ macros::ld(i=i) }}
    cpu.set_hl(cpu.get_hl().wrapping_sub(1));

    {%- elif i.operator == "ldi" -%}

    {{ macros::ld(i=i) }}
    cpu.set_hl(cpu.get_hl().wrapping_add(1));

    {%- elif i.operator == "ldhl" -%}

    {{ macros::ldhl(i=i) }}

    {%- elif i.operator == "add" -%}

    {%- if i.code == 232 -%}
    {{ macros::addsp(i=i) }}
    {%- else -%}
    {%- if i.bits == 8 -%}
    {{ macros::add8(i=i) }}
    {%- else -%}
    {{ macros::add16(i=i) }}
    {%- endif -%}
    {%- endif -%}

    {%- elif i.operator == "sub" -%}

    {{ macros::sub(i=i) }}

    {%- elif i.operator == "adc" -%}
    {{ macros::adc(i=i) }}

    {%- elif i.operator == "sbc" -%}
    {{ macros::sbc(i=i) }}

    {%- elif i.operator == "and" -%}
    {{ macros::and(i=i) }}

    {%- elif i.operator == "or" -%}
    {{ macros::or(i=i) }}

    {%- elif i.operator == "xor" -%}
    {{ macros::xor(i=i) }}

    {%- elif i.operator == "cp" -%}
    {{ macros::cp(i=i) }}

    {%- elif i.operator == "push" -%}
    {{ macros::push(i=i) }}

    {%- elif i.operator == "pop" -%}
    {{ macros::pop(i=i) }}

    {%- elif i.operator == "swap" -%}
    {{ macros::swap(i=i) }}

    {%- elif i.operator == "daa" -%}
    {{ macros::daa(i=i) }}

    {%- elif i.operator == "cpl" -%}
    {{ macros::cpl(i=i) }}
    {%- elif i.operator == "ccf" -%}
    {{ macros::ccf(i=i) }}
    {%- elif i.operator == "scf" -%}
    {{ macros::scf(i=i) }}

    {%- elif i.operator == "ei" -%}
    {{ macros::ei(i=i) }}
    {%- elif i.operator == "di" -%}
    {{ macros::di(i=i) }}
    {%- elif i.operator == "halt" -%}
    {{ macros::halt(i=i) }}
    {%- elif i.operator == "stop" -%}
    {{ macros::stop(i=i) }}

    {%- elif i.operator == "rlc" -%}
    {{ macros::rlc(i=i) }}
    {%- elif i.operator == "rlca" -%}
    {{ macros::rlca(i=i) }}
    {%- elif i.operator == "rl" -%}
    {{ macros::rl(i=i) }}
    {%- elif i.operator == "rla" -%}
    {{ macros::rla(i=i) }}

    {%- elif i.operator == "rrc" -%}
    {{ macros::rrc(i=i) }}
    {%- elif i.operator == "rrca" -%}
    {{ macros::rrca(i=i) }}
    {%- elif i.operator == "rr" -%}
    {{ macros::rr(i=i) }}
    {%- elif i.operator == "rra" -%}
    {{ macros::rra(i=i) }}

    {%- elif i.operator == "sla" -%}
    {{ macros::sla(i=i) }}
    {%- elif i.operator == "sra" -%}
    {{ macros::sra(i=i) }}
    {%- elif i.operator == "srl" -%}
    {{ macros::srl(i=i) }}

    {%- elif i.operator == "bit" -%}
    {{ macros::bit(i=i) }}
    {%- elif i.operator == "set" -%}
    {{ macros::set(i=i) }}
    {%- elif i.operator == "res" -%}
    {{ macros::res(i=i) }}

    {%- elif i.operator == "jr" -%}

    {%- if i.time | is_cond == true -%}
    {{ macros::jrif(i=i) }}
    {%- else -%}
    {{ macros::jr(i=i) }}
    {%- endif -%}

    {%- elif i.operator == "jp" -%}

    {%- if i.time | is_cond == true -%}
    {{ macros::jpif(i=i) }}
    {%- else -%}
    {{ macros::jp(i=i) }}
    {%- endif -%}

    {%- elif i.operator == "call" -%}

    {%- if i.time | is_cond == true -%}
    {{ macros::callif(i=i) }}
    {%- else -%}
    {{ macros::call(i=i) }}
    {%- endif -%}
    {%- elif i.operator == "rst" -%}
    {{ macros::rst(i=i) }}

    {%- elif i.operator == "ret" -%}

    {%- if i.time | is_cond == true -%}
    {{ macros::retif(i=i) }}
    {%- else -%}
    {{ macros::ret(i=i) }}
    {%- endif -%}

    {%- elif i.operator == "reti" -%}
    {{ macros::reti(i=i) }}
    {%- endif -%}

    {{ i.z | setflag(flg="z") }}
    {{ i.n | setflag(flg="n") }}
    {{ i.h | setflag(flg="h") }}
    {{ i.c | setflag(flg="c") }}


    ({{i.time | untuple}}, {{i.size}})
}
{% endfor %}

pub fn mnem(code: u16) -> &'static str {
    MNEMONICS.get(&code).unwrap_or(&"(unknown opcode)")
}

pub fn decode(code: u16, arg: u16, cpu: &mut Cpu, mmu: &mut Mmu) -> (usize, usize) {
    trace!("{:04x}: {:04x}: {}", cpu.get_pc(), code, mnem(code));

    match code {
        {%- for i in insts -%}
        0x{{i.code | hex}} => op_{{i.code | hex}}(arg, cpu, mmu),
        {%- endfor -%}
        _ => panic!("Invalid opcode: {:04x}: {:04x}", cpu.get_pc(), code),
    }
}
