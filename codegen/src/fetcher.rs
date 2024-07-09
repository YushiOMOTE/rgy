use pest::Parser;
#[derive(Parser)]
#[grammar = "inst.pest"]
struct InstParser;

use regex::Regex;
use scraper::element_ref::ElementRef;
use scraper::{Html, Selector};
use std::collections::HashMap;
// use std::path::PathBuf;
use crate::format::{Instruction, Time};
use std::fs::File;
use std::io::prelude::*;

use crate::{Error, Fetch, Result};

lazy_static! {
    static ref ALT: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("LD A,(C)", "LD A,(FF00h+C)");
        m.insert("LD (C),A", "LD (FF00h+C),A");
        m.insert("LDH A,(a8)", "LD A,(FF00h+a8)");
        m.insert("LDH (a8),A", "LD (FF00h+a8),A");
        m.insert("LD A,(HL+)", "LDI A,(HL)");
        m.insert("LD (HL+),A", "LDI (HL),A");
        m.insert("LD A,(HL-)", "LDD A,(HL)");
        m.insert("LD (HL-),A", "LDD (HL),A");
        m.insert("LD HL,SP+r8", "LDHL SP,r8");
        m
    };
}

fn alter(s: &str) -> String {
    ALT.iter().fold(s.to_string(), |s, (k, v)| s.replace(k, v))
}

fn modify(code: u16, s: &str) -> String {
    let s = s.to_lowercase();

    if s == "c" && (code == 0x38 || code == 0xd8 || code == 0xda || code == 0xdc) {
        // Special cases: conditional jump and returns
        "cf".to_string()
    } else {
        let re = Regex::new(r"(?P<v>[0-9a-zA-Z]+)h").expect("Invalid regex");
        re.replace_all(&s, "0x$v").to_string()
    }
}

lazy_static! {
    static ref SUFFIX: HashMap<&'static str, usize> = {
        let mut suffix = HashMap::new();
        suffix.insert("#ff99cc", 0);
        suffix.insert("#ffcc99", 0);
        suffix.insert("#ccccff", 8);
        suffix.insert("#ccffcc", 16);
        suffix.insert("#ffff99", 8);
        suffix.insert("#ffcccc", 16);
        suffix.insert("#80ffff", 8);
        suffix
    };
}

fn parse_time(s: &str) -> Time {
    if s.contains("/") {
        let mut nums = s.split("/");
        Time::Two(
            nums.next()
                .expect("Incomplete time")
                .parse()
                .expect("Bad time"),
            nums.next()
                .expect("Incomplete time")
                .parse()
                .expect("Bad time"),
        )
    } else {
        Time::One(s.parse().expect("Bad time"))
    }
}

fn parse_table(table: ElementRef, op_prefix: u16) -> Vec<Instruction> {
    let mut vec = Vec::new();

    let mut x = 0;
    let mut y = 0;

    let sel = Selector::parse("td").expect("Select failed");

    for item in table.select(&sel) {
        let bits = *SUFFIX
            .get(item.value().attr("bgcolor").unwrap_or(""))
            .unwrap_or(&0);

        let s = alter(&item.inner_html());

        let code = ((y - 1) << 4 | (x - 1)) as u16 | (op_prefix << 8);

        x += 1;
        if x % 17 == 0 {
            y += 1;
            x = 0;
        }

        let mut p = match InstParser::parse(Rule::Instruction, &s) {
            Ok(p) => p,
            Err(e) => {
                debug!("Skipping: {}", e);
                continue;
            }
        };

        let mnem = p.next().expect("No mnemonic");

        let mut ops = mnem.into_inner();
        let operator = ops.next().expect("No operator").as_str().to_lowercase();
        let operands = ops.map(|p| modify(code, p.as_str())).collect::<Vec<_>>();

        let size: usize = p
            .next()
            .expect("No size")
            .as_str()
            .parse()
            .expect("Bad size");
        let time = parse_time(p.next().expect("No time").as_str());
        let flag = p.next().expect("No flag");

        let mut flag = flag.into_inner();
        let z = flag.next().expect("No z flag").as_str().into();
        let n = flag.next().expect("No n flag").as_str().into();
        let h = flag.next().expect("No h flag").as_str().into();
        let c = flag.next().expect("No c flag").as_str().into();

        info!(
            "{:02x}: {} {:?}: bits: {}, size: {}, time: {:?}, flags: z[{}],n[{}],h[{}],c[{}]",
            code, operator, operands, bits, size, time, z, n, h, c
        );

        vec.push(Instruction {
            code,
            operator,
            operands,
            bits,
            size,
            time,
            z,
            n,
            h,
            c,
        })
    }

    vec
}

pub fn run(opt: &Fetch) -> Result<()> {
    let doc = reqwest::blocking::get("http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html")
        .unwrap()
        .text()
        .unwrap();

    let doc = Html::parse_document(&doc);

    let sel = Selector::parse("table").map_err(|_| Error("Select failed".into()))?;
    let mut it = doc.select(&sel);

    let mut insts = Vec::new();

    if let Some(table) = it.next() {
        insts.extend(parse_table(table, 0));
    }
    if let Some(table) = it.next() {
        insts.extend(parse_table(table, 0xcb));
    }

    println!("Downloaded {} instructions", insts.len());

    let insts = serde_yaml::to_string(&insts).expect("Pack error");

    let mut file = File::create(&opt.output)?;
    file.write_all(insts.as_bytes())?;

    Ok(())
}
