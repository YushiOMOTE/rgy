use crate::{Error, Generate, Result};

use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use tera::{to_value, Context, Value};

use crate::format::Instruction;

fn is_num(s: &str) -> bool {
    match s.trim().parse::<usize>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn setflag(value: Value, map: HashMap<String, Value>) -> tera::Result<Value> {
    let v = try_get_value!("setflag", "value", String, value);
    let f = try_get_value!("setflag", "flg", String, map.get("flg").unwrap());
    if v == "-" {
        Ok(to_value("").unwrap())
    } else if v == "0" {
        Ok(to_value(format!("self.set_{}f(false);", f)).unwrap())
    } else if v == "1" {
        Ok(to_value(format!("self.set_{}f(true);", f)).unwrap())
    } else {
        Ok(to_value(format!("self.set_{}f({});", f, v.to_lowercase())).unwrap())
    }
}

fn eval_getter(s: &str, b: usize) -> String {
    if s == "nz" {
        format!("!self.get_zf()")
    } else if s == "nc" {
        format!("!self.get_cf()")
    } else if s == "z" {
        format!("self.get_zf()")
    } else if s == "cf" {
        format!("self.get_cf()")
    } else if s == "d8" || s == "a8" || s == "r8" {
        "self.fetch8()".into()
    } else if s == "d16" || s == "a16" {
        "self.fetch16()".into()
    } else if s.starts_with("0x") {
        let mut expr = s.split("+");
        let offset = expr.next().expect("No offset");
        let arg = expr.next().expect("No arg");
        format!("{}+{} as u16", offset, eval_getter(&arg, b))
    } else if is_num(s) {
        format!("{}", s)
    } else if s.starts_with("(") {
        format!(
            "{{ let x = {}; self.get{}(x) }}",
            eval_getter(&s[1..s.len() - 1], b),
            b,
        )
    } else {
        format!("self.get_{}()", s)
    }
}

pub fn getter(value: Value, map: HashMap<String, Value>) -> tera::Result<Value> {
    let v = try_get_value!("arg", "value", String, value);
    let b = try_get_value!("arg", "bits", usize, map.get("bits").unwrap());
    Ok(to_value(&eval_getter(&v, b)).unwrap())
}

fn eval_setter(s: &str, b: usize) -> String {
    if s.starts_with("(") {
        format!(
            "let x = {}; self.set{}(x, ",
            eval_getter(&s[1..s.len() - 1], b),
            b,
        )
    } else {
        format!("self.set_{}(", s)
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

pub fn untuple(value: Value, _: HashMap<String, Value>) -> tera::Result<Value> {
    let tuple = match &value {
        Value::Number(_) => false,
        Value::Array(_) => true,
        _ => false,
    };

    if tuple {
        let v = try_get_value!("untuple", "value", Vec<usize>, value);
        Ok(to_value(&format!("{}", v[v.len() - 1])).unwrap())
    } else {
        let v = try_get_value!("untuple", "value", usize, value);
        Ok(to_value(&format!("{}", v)).unwrap())
    }
}

pub fn is_cond(value: Value, _: HashMap<String, Value>) -> tera::Result<Value> {
    let b = match &value {
        Value::Number(_) => false,
        Value::Array(_) => true,
        _ => false,
    };
    Ok(to_value(b).unwrap())
}

pub fn run(opt: &Generate) -> Result<()> {
    let mut tera = compile_templates!(&format!(
        "{}/**/*",
        opt.template.to_str().unwrap_or("templates")
    ));
    tera.register_filter("hex", hex);
    tera.register_filter("getter", getter);
    tera.register_filter("setter", setter);
    tera.register_filter("untuple", untuple);
    tera.register_filter("setflag", setflag);
    tera.register_filter("is_cond", is_cond);

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
            return Err(Error("Render error".into()));
        }
    };

    let process = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't spawn rustfmt");

    process
        .stdin
        .unwrap()
        .write_all(output.as_bytes())
        .expect("Couldn't write to rustfmt");

    let mut formatted = String::new();

    process
        .stdout
        .unwrap()
        .read_to_string(&mut formatted)
        .expect("Couldn't read rustfmt");

    let mut file = File::create(&opt.output).expect("No output");
    file.write_all(formatted.as_bytes())?;

    Ok(())
}
