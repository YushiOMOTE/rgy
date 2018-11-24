{% import "ops.rs" as macros %}

{% for i in insts %}
fn op_{{i.code | hex}}(cpu: &Cpu) {
  {%- if i.operator == "ld" -%}
    {{ macros::ld(i=i) }}
  {%- endif -%}
}
{% endfor %}
