{% macro ld(i) %}
  let v = {{ i.operands[1] | getter(bits=i.bits) }};
  {{ i.operands[0] | setter(bits=i.bits) }}(v.into());
{% endmacro ld %}
