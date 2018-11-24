use crate::{Generate, Error, Result};

use serde_yaml;
use tera::{Context, to_value, Value};
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use crate::format::Instruction;

fn eval_getter(s: &str, b: usize) -> String {
    if s.starts_with("(") {
        format!("cpu.mc.as{}({}).get()", b, eval_getter(&s[1..s.len()-1], b))
    } else {
        format!("cpu.{}.get()", s)
    }
}

pub fn getter(value: Value, map: HashMap<String, Value>) -> tera::Result<Value> {
    let v = try_get_value!("arg", "value", String, value);
    let b = try_get_value!("arg", "bits", usize, map.get("bits").unwrap());
    Ok(to_value(&eval_getter(&v, b)).unwrap())
}

fn eval_setter(s: &str, b: usize) -> String {
    if s.starts_with("(") {
        format!("cpu.mc.as{}({}).set", b, eval_getter(&s[1..s.len()-1], b))
    } else {
        format!("cpu.{}.set", s)
    }
}

pub fn setter(value: Value, map: HashMap<String, Value>) -> tera::Result<Value> {
    let v = try_get_value!("setter", "value", String, value);
    let b = try_get_value!("setter", "bits", usize, map.get("bits").unwrap());
    Ok(to_value(&eval_setter(&v, b)).unwrap())
}

pub fn hex(value: Value, _: HashMap<String, Value>) -> tera::Result<Value> {
    let v = try_get_value!("hex", "value", u16, value);
    Ok(to_value(&format!("{:04x}", v)).unwrap())
}

pub fn run(opt: &Generate) -> Result<()> {
    let mut tera = compile_templates!(&format!("{}/**/*", opt.template.to_str().unwrap_or("templates") ));
    tera.register_filter("hex", hex);
    tera.register_filter("getter", getter);
    tera.register_filter("setter", setter);

    let mut context = Context::new();

    let file = File::open(&opt.oplist).expect("Op list not found");
    let insts: Vec<Instruction> = serde_yaml::from_reader(file).expect("Unpack error");

    context.insert("insts", &insts);

    let output = match tera.render("root.rs", &context) {
        Ok(output) => output,
        Err(e) => {
            println!("Error: {}", e);
            for e in e.iter().skip(1) {
                println!("Reason: {}", e);
            }
            return Err(Error("Render error".into()))
        }
    };

    let mut file = File::create(&opt.output).expect("No output");
    file.write_all(output.as_bytes())?;

    Ok(())
}
