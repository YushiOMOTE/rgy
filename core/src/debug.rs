use crate::inst::mnem;
use crate::cpu::Cpu;
use crate::mmu::{MemHandler, MemRead, MemWrite, Mmu};
use log::*;

use std::time::Instant;
use std::string::ToString;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use std::collections::HashSet;
use rustyline::error::ReadlineError;
use rustyline::Editor;

struct CmdError(String);

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CmdError {
    fn new<T: ToString>(s: T) -> CmdError {
        CmdError(s.to_string())
    }
}

impl From<std::io::Error> for CmdError {
    fn from(e: std::io::Error) -> CmdError {
        CmdError(e.to_string())
    }
}

impl From<std::num::ParseIntError> for CmdError {
    fn from(e: std::num::ParseIntError) -> CmdError {
        CmdError(e.to_string())
    }
}

type CmdResult<T> = std::result::Result<T, CmdError>;

pub struct Resource<'a> {
    cpu: &'a Cpu,
    mmu: &'a Mmu,
}

impl<'a> Resource<'a> {
    pub fn new(cpu: &'a Cpu, mmu: &'a Mmu) -> Resource<'a> {
        Resource { cpu, mmu }
    }
}

pub struct Debugger {
    inner: Rc<RefCell<Inner>>,
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            inner: Rc::new(RefCell::new(Inner::new())),
        }
    }

    pub fn handler(&self) -> DebugMemHandler {
        DebugMemHandler::new(self.inner.clone())
    }

    pub fn init(&self, res: &Resource) {
        self.inner.borrow_mut().init(res)
    }

    pub fn on_decode(&self, res: &Resource) {
        self.inner.borrow_mut().on_decode(res)
    }
}

struct Inner {
    breaks: HashSet<u16>,
    rd_watches: HashSet<u16>,
    wr_watches: HashSet<u16>,
    prompt: bool,
    stepping: bool,
}

impl Inner {
    fn new() -> Self {
        Self {
            breaks: HashSet::new(),
            rd_watches: HashSet::new(),
            wr_watches: HashSet::new(),
            prompt: false,
            stepping: false,
        }
    }

    fn init(&mut self, res: &Resource) {
        println!("Entering debug shell...");

        self.prompt(res)
    }

    fn on_decode(&mut self, res: &Resource) {
        if self.check_break(res) {
            self.do_break(res);
        }
    }

    fn check_break(&self, res: &Resource) -> bool {
        if self.prompt {
            false
        } else if self.stepping {
            true
        } else {
            let pc = res.cpu.get_pc();

            self.breaks.contains(&pc)
        }
    }

    fn do_break(&mut self, res: &Resource) {
        let (code, _) = res.cpu.fetch(res.mmu);

        println!(
            "Break at {:04x}: {:04x}: {}",
            res.cpu.get_pc(),
            code,
            mnem(code)
        );

        self.prompt(res)
    }

    fn prompt(&mut self, res: &Resource) {
        self.prompt = true;

        let mut rl = Editor::<()>::new();

        if rl.load_history("history.txt").is_err() {
            println!("No previous history");
        }

        let abort = loop {
            let readline = rl.readline(">> ");

            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_ref());

                    match self.handle(&line, res) {
                        Ok(end) => if end {
                            break false;
                        } else {
                            continue;
                        },
                        Err(e) => {
                            println!("Command error: {}", e);
                            continue;
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("Abort");
                    break true;
                }
                Err(ReadlineError::Eof) => {
                    println!("Resume");
                    break false;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break true;
                }
            }
        };

        let _ = rl.save_history("history.txt");

        if abort {
            std::process::exit(1);
        }

        self.prompt = false;
    }

    fn handle(&mut self, line: &str, res: &Resource) -> CmdResult<bool> {
        let (cmd, args) = parse(line)?;

        if cmd.is_empty() {
            return Ok(false);
        }

        match cmd {
            "ab" => self.add_break(parse_addr(args)?),
            "rb" => self.remove_break(parse_addr(args)?),
            "lb" => self.list_breaks(),
            "arw" => self.add_rdwatch(parse_addr(args)?),
            "aww" => self.add_wrwatch(parse_addr(args)?),
            "rrw" => self.remove_rdwatch(parse_addr(args)?),
            "rww" => self.remove_wrwatch(parse_addr(args)?),
            "d" => self.dump(res),
            "s" => self.stack(res),
            "q" => self.quit(),
            "n" => self.step(),
            "c" => self.resume(),
            _ => Err(CmdError::new("Unknown command")),
        }
    }

    fn add_break(&mut self, addr: u16) -> CmdResult<bool> {
        if self.breaks.insert(addr) {
            println!("Set break point at {:04x}", addr);
        } else {
            println!("Break point already set at {:04x}", addr);
        }

        Ok(false)
    }

    fn remove_break(&mut self, addr: u16) -> CmdResult<bool> {
        if self.breaks.remove(&addr) {
            println!("Remove break point at {:04x}", addr);
        } else {
            println!("Break point isn't set at {:04x}", addr);
        }

        Ok(false)
    }

    fn list_breaks(&self) -> CmdResult<bool> {
        println!("Break points: ");

        for addr in self.breaks.iter() {
            println!("* {:04x}", addr);
        }

        Ok(false)
    }

    fn add_rdwatch(&mut self, addr: u16) -> CmdResult<bool> {
        unimplemented!()
    }

    fn remove_rdwatch(&mut self, addr: u16) -> CmdResult<bool> {
        unimplemented!()
    }

    fn add_wrwatch(&mut self, addr: u16) -> CmdResult<bool> {
        unimplemented!()
    }

    fn remove_wrwatch(&mut self, addr: u16) -> CmdResult<bool> {
        unimplemented!()
    }

    fn dump(&self, res: &Resource) -> CmdResult<bool> {
        println!("{}", res.cpu);

        Ok(false)
    }

    fn stack(&self, res: &Resource) -> CmdResult<bool> {
        let sp = res.cpu.get_sp();

        for i in 0..10 {
            let (p, of) = sp.overflowing_add(i * 2);
            if of {
                break;
            }
            println!("{}: {:04x} [{:04x}]", i, p, res.mmu.get16(p));
        }

        Ok(false)
    }

    fn step(&mut self) -> CmdResult<bool> {
        self.stepping = true;

        Ok(true)
    }

    fn resume(&mut self) -> CmdResult<bool> {
        self.stepping = false;

        println!("Resume");

        Ok(true)
    }

    fn quit(&self) -> CmdResult<bool> {
        println!("Quit");

        std::process::exit(1)
    }

    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        MemRead::PassThrough
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        MemWrite::PassThrough
    }
}

fn parse_addr(args: Vec<&str>) -> CmdResult<u16> {
    let i = u16::from_str_radix(args.get(0).ok_or(CmdError::new("No arg"))?, 16)?;

    Ok(i)
}

fn parse<'a>(cmd: &'a str) -> CmdResult<(&'a str, Vec<&'a str>)> {
    let mut it = cmd.split(" ");
    let cmd = it.next().ok_or(CmdError::new("No command"))?;

    Ok((cmd, it.collect()))
}

pub struct DebugMemHandler {
    inner: Rc<RefCell<Inner>>,
}

impl DebugMemHandler {
    fn new(inner: Rc<RefCell<Inner>>) -> DebugMemHandler {
        DebugMemHandler { inner }
    }
}

impl MemHandler for DebugMemHandler {
    fn on_read(&self, mmu: &Mmu, addr: u16) -> MemRead {
        // Don't hook if it's already hooked
        match self.inner.try_borrow_mut() {
            Ok(mut inner) => inner.on_read(mmu, addr),
            Err(_) => MemRead::PassThrough,
        }
    }

    fn on_write(&self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        // Don't hook if it's already hooked
        match self.inner.try_borrow_mut() {
            Ok(mut inner) => inner.on_write(mmu, addr, value),
            Err(_) => MemWrite::PassThrough,
        }
    }
}

pub struct Perf {
    counter: u64,
    last: Instant,
}

impl Perf {
    pub fn new() -> Perf {
        Perf {
            counter: 0,
            last: Instant::now(),
        }
    }

    pub fn count(&mut self) {
        let sample = 10000000;

        self.counter += 1;

        if self.counter % sample == 0 {
            let now = Instant::now();
            let df = now - self.last;
            let df = df.as_secs() * 1000000 + df.subsec_micros() as u64;

            debug!("{} ips", sample * 1000000 / df);

            self.last = now;
        }
    }
}
