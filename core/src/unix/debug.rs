use core::cpu::Cpu;
use core::device::IoHandler;
use core::inst::mnem;
use core::mmu::{MemRead, MemWrite, Mmu};

use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::string::ToString;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use signal_hook;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use structopt::StructOpt;

use lazy_static::lazy_static;

#[derive(Debug)]
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

pub struct Debugger {
    breaks: HashSet<u16>,
    rd_watches: HashSet<u16>,
    wr_watches: HashSet<u16>,
    prompt: bool,
    stepping: bool,
    cpu_state: Cpu,
    signal: Signal,
    exec_path: VecDeque<u16>,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            breaks: HashSet::new(),
            rd_watches: HashSet::new(),
            wr_watches: HashSet::new(),
            prompt: false,
            stepping: false,
            cpu_state: Cpu::new(),
            signal: Signal::new(),
            exec_path: VecDeque::new(),
        }
    }

    fn add_exec_path(&mut self, pc: u16) {
        if let Some(p) = self.exec_path.back() {
            if pc == *p {
                return;
            }
        }

        self.exec_path.push_back(pc);
    }

    fn check_break(&self, pc: u16, _mmu: &Mmu) -> bool {
        if self.prompt {
            false
        } else if self.stepping {
            true
        } else {
            self.breaks.contains(&pc)
        }
    }

    fn do_break(&mut self, msg: &str, mmu: &Mmu) {
        let (code, _) = self.cpu_state.fetch(mmu);

        println!(
            "{} at {:04x}: {:04x}: {}",
            msg,
            self.cpu_state.get_pc(),
            code,
            mnem(code)
        );

        self.prompt(mmu)
    }

    fn prompt(&mut self, mmu: &Mmu) {
        self.prompt = true;

        let mut rl = Editor::<()>::new();

        if rl.load_history("history.txt").is_err() {
            println!("No previous history");
        }

        let abort = loop {
            let readline = rl.readline(">> ");

            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());

                    match exec_cmd(self, mmu, &line) {
                        Ok(end) => {
                            if end {
                                break false;
                            } else {
                                continue;
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
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
}

impl core::debug::Debugger for Debugger {
    fn init(&mut self, mmu: &Mmu) {
        println!("Entering debug shell...");

        self.prompt(mmu)
    }

    fn take_cpu_snapshot(&mut self, cpu: Cpu) {
        self.cpu_state = cpu;
    }

    fn on_decode(&mut self, mmu: &Mmu) {
        let pc = self.cpu_state.get_pc();

        self.add_exec_path(pc);

        if self.check_break(pc, mmu) {
            self.do_break("Break", mmu);
        }
    }

    fn check_signal(&mut self) {
        if self.signal.signaled() {
            println!("Signaled.");
            self.stepping = true;
        }
    }
}

impl IoHandler for Debugger {
    fn on_read(&mut self, mmu: &Mmu, addr: u16) -> MemRead {
        if self.rd_watches.contains(&addr) {
            self.do_break(&format!("Reading {:04x}", addr), mmu);
        }

        MemRead::PassThrough
    }

    fn on_write(&mut self, mmu: &Mmu, addr: u16, value: u8) -> MemWrite {
        if self.wr_watches.contains(&addr) {
            self.do_break(&format!("Writing {:02x} to {:04x}", value, addr), mmu);
        }

        MemWrite::PassThrough
    }
}

fn exec_cmd(inner: &mut Debugger, mmu: &Mmu, line: &str) -> CmdResult<bool> {
    let cmd = match line.split_whitespace().next() {
        Some(cmd) => cmd,
        None => return Ok(false),
    };

    match find_cmd(cmd) {
        Some(cmd) => (cmd.handler)(inner, mmu, line),
        None => Err(CmdError::new(format!("Command not found: {}", line))),
    }
}

fn find_cmd(s: &str) -> Option<&'static CmdInfo> {
    for cmd in COMMANDS.iter() {
        if cmd.name == s || cmd.short == Some(s) {
            return Some(cmd);
        }
    }

    None
}

struct CmdInfo {
    name: &'static str,
    short: Option<&'static str>,
    desc: &'static str,
    handler: Box<dyn Fn(&mut Debugger, &Mmu, &str) -> CmdResult<bool> + Send + Sync + 'static>,
}

trait CmdHandler: StructOpt + Sized {
    fn handle(&self, inner: &mut Debugger, mmu: &Mmu) -> CmdResult<bool>;

    fn parse(inner: &mut Debugger, mmu: &Mmu, s: &str) -> CmdResult<bool> {
        let s = s.split_whitespace();
        match Self::from_iter_safe(s) {
            Ok(p) => p.handle(inner, mmu),
            Err(e) => Err(CmdError::new(e)),
        }
    }
}

macro_rules! cc {
    ($vec: ident, $name: expr, $short: expr, $desc: expr, $handler: tt) => {
        $vec.push(CmdInfo {
            name: $name,
            desc: $desc,
            short: $short,
            handler: Box::new(|inner, mmu, line| $handler::parse(inner, mmu, line)),
        });
    };
}

lazy_static! {
    static ref COMMANDS: Vec<CmdInfo> = {
        let mut m = Vec::new();
        cc!(m, "break", Some("b"), "Manage break points.", CmdBreak);
        cc!(m, "watch", Some("w"), "Manage memory watches.", CmdWatch);
        cc!(
            m,
            "help",
            Some("h"),
            "Show the list of commands available.",
            CmdHelp
        );
        cc!(m, "quit", None, "Quit this emulator.", CmdQuit);
        cc!(m, "cont", Some("c"), "Continue execution.", CmdContinue);
        cc!(m, "step", Some("n"), "Step execution.", CmdStep);
        cc!(m, "dump", Some("d"), "Dump information.", CmdDump);
        m
    };
}

fn parse_addr(s: &str) -> CmdResult<u16> {
    u16::from_str_radix(s, 16).map_err(|e| CmdError::new(e))
}

#[derive(StructOpt, Debug)]
#[structopt(name = "break", about = "Manage break points.")]
enum CmdBreak {
    /// Add a break point
    #[structopt(name = "add")]
    Add {
        /// Address in hex
        #[structopt(name = "addr", parse(try_from_str = "parse_addr"))]
        addr: u16,
    },
    /// Remove a break point
    #[structopt(name = "remove")]
    Remove {
        /// Address in hex
        #[structopt(name = "addr", parse(try_from_str = "parse_addr"))]
        addr: u16,
    },
    /// List break points
    #[structopt(name = "list")]
    List,
}

impl CmdHandler for CmdBreak {
    fn handle(&self, inner: &mut Debugger, _mmu: &Mmu) -> CmdResult<bool> {
        match self {
            CmdBreak::Add { addr } => {
                if inner.breaks.insert(*addr) {
                    println!("Set break point at {:04x}", addr);
                } else {
                    println!("Break point already set at {:04x}", addr);
                }
            }
            CmdBreak::Remove { addr } => {
                if inner.breaks.remove(&addr) {
                    println!("Remove break point at {:04x}", addr);
                } else {
                    println!("Break point isn't set at {:04x}", addr);
                }
            }
            CmdBreak::List => {
                println!("Break points: ");

                for addr in inner.breaks.iter() {
                    println!("* {:04x}", addr);
                }
            }
        }

        Ok(false)
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "help", about = "Show the list of available commands.")]
struct CmdHelp {}

impl CmdHandler for CmdHelp {
    fn handle(&self, _inner: &mut Debugger, _mmu: &Mmu) -> CmdResult<bool> {
        println!("List of available commands:");

        for cmd in COMMANDS.iter() {
            println!(
                "{:>8}: {} {}",
                cmd.name,
                cmd.desc,
                if let Some(short) = cmd.short {
                    format!("(short: {})", short)
                } else {
                    "".into()
                }
            );
        }

        Ok(false)
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "quit", about = "Quit this emulator.")]
struct CmdQuit {}

impl CmdHandler for CmdQuit {
    fn handle(&self, _inner: &mut Debugger, _mmu: &Mmu) -> CmdResult<bool> {
        println!("Quit.");

        std::process::exit(1)
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "cont", about = "Continue execution.")]
struct CmdContinue {}

impl CmdHandler for CmdContinue {
    fn handle(&self, inner: &mut Debugger, _mmu: &Mmu) -> CmdResult<bool> {
        println!("Continue.");

        inner.stepping = false;

        Ok(true)
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "step", about = "Step execution.")]
struct CmdStep {}

impl CmdHandler for CmdStep {
    fn handle(&self, inner: &mut Debugger, _mmu: &Mmu) -> CmdResult<bool> {
        println!("Step.");

        inner.stepping = true;

        Ok(true)
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "dump", about = "Dump information.")]
enum CmdDump {
    /// Dump cpu state
    #[structopt(name = "cpu")]
    Cpu,
    /// Dump stack
    #[structopt(name = "stack")]
    Stack {
        /// The size of stack to dump
        #[structopt(name = "size", default_value = "10")]
        size: u16,
    },
    /// Dump memory
    #[structopt(name = "mem")]
    Mem {
        /// The start of the memory region to dump
        #[structopt(name = "from", parse(try_from_str = "parse_addr"))]
        from: u16,
        /// The end of the memory region to dump (inclusive)
        #[structopt(name = "to", parse(try_from_str = "parse_addr"))]
        to: u16,
    },
    /// Execution path
    #[structopt(name = "path")]
    Path {
        /// The number of step to dump
        #[structopt(name = "size", default_value = "10")]
        size: usize,
    },
}

impl CmdHandler for CmdDump {
    fn handle(&self, inner: &mut Debugger, mmu: &Mmu) -> CmdResult<bool> {
        match self {
            CmdDump::Cpu => {
                println!("{}", inner.cpu_state);
            }
            CmdDump::Stack { size } => {
                let sp = inner.cpu_state.get_sp();

                for i in 0..*size {
                    let (p, of) = sp.overflowing_add(i * 2);
                    if of {
                        break;
                    }
                    println!("{}: {:04x} [{:04x}]", i + 1, p, mmu.get16(p));
                }
            }
            CmdDump::Mem { from, to } => {
                print!("      ");
                for i in 0..16 {
                    if i % 2 == 0 {
                        print!("{:02x}", i)
                    } else {
                        print!("{:02x} ", i)
                    }
                }
                println!();

                let pad = from % 16;

                if pad != 0 {
                    print!("{:04x}: ", from - pad);
                    for i in 0..pad {
                        if i % 2 == 0 {
                            print!("  ");
                        } else {
                            print!("   ");
                        }
                    }
                }

                for i in *from..=*to {
                    if i % 16 == 0 {
                        print!("{:04x}: ", i);
                    }

                    let b = mmu.get8(i);

                    if i % 2 == 0 {
                        print!("{:02x}", b);
                    } else {
                        print!("{:02x} ", b);
                    }

                    if i % 16 == 15 {
                        println!()
                    }
                }

                if to % 16 != 15 {
                    println!()
                }
            }
            CmdDump::Path { size } => {
                for (i, pc) in inner.exec_path.iter().rev().take(*size).enumerate() {
                    let mut cpu = inner.cpu_state.clone();
                    cpu.set_pc(*pc);
                    let (code, _) = cpu.fetch(mmu);

                    println!("-{}: {:04x}: {}", i, pc, mnem(code));
                }
            }
        }

        Ok(false)
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "watch", about = "Manage watch points.")]
enum CmdWatch {
    /// Add a watch point
    #[structopt(name = "add")]
    Add {
        /// Address in hex
        #[structopt(name = "addr", parse(try_from_str = "parse_addr"))]
        addr: u16,
        /// Add watch only for read access
        #[structopt(long = "readonly", short = "r")]
        readonly: bool,
        /// Add watch only for write access
        #[structopt(long = "writeonly", short = "w")]
        writeonly: bool,
    },
    /// Remove a watch point
    #[structopt(name = "remove")]
    Remove {
        /// Address in hex
        #[structopt(name = "addr", parse(try_from_str = "parse_addr"))]
        addr: u16,
        /// Remove watch only for read access
        #[structopt(long = "readonly", short = "r")]
        readonly: bool,
        /// Remove watch only for write access
        #[structopt(long = "writeonly", short = "w")]
        writeonly: bool,
    },
    /// List watch points
    #[structopt(name = "list")]
    List,
}

impl CmdHandler for CmdWatch {
    fn handle(&self, inner: &mut Debugger, _mmu: &Mmu) -> CmdResult<bool> {
        match self {
            CmdWatch::Add {
                addr,
                readonly,
                writeonly,
            } => {
                if *readonly && *writeonly {
                    println!("Nothing set because both readonly and writeonly are set");
                    return Ok(false);
                }
                if !writeonly {
                    if inner.rd_watches.insert(*addr) {
                        println!("Set read watch at {:04x}", addr);
                    } else {
                        println!("Read watch already set at {:04x}", addr);
                    }
                }
                if !readonly {
                    if inner.wr_watches.insert(*addr) {
                        println!("Set write watch at {:04x}", addr);
                    } else {
                        println!("Write watch already set at {:04x}", addr);
                    }
                }
            }
            CmdWatch::Remove {
                addr,
                readonly,
                writeonly,
            } => {
                if *readonly && *writeonly {
                    println!("Nothing unset because both readonly and writeonly are set");
                    return Ok(false);
                }
                if !writeonly {
                    if inner.rd_watches.remove(&addr) {
                        println!("Remove read watch at {:04x}", addr);
                    } else {
                        println!("Read watch is already unset at {:04x}", addr);
                    }
                }
                if !readonly {
                    if inner.wr_watches.remove(&addr) {
                        println!("Remove writeonly watch at {:04x}", addr);
                    } else {
                        println!("Write watch is already unset at {:04x}", addr);
                    }
                }
            }
            CmdWatch::List => {
                println!("Watch points: ");

                for addr in inner.rd_watches.union(&inner.wr_watches) {
                    let wr = if inner.wr_watches.contains(addr) {
                        'w'
                    } else {
                        '_'
                    };
                    let rd = if inner.rd_watches.contains(addr) {
                        'r'
                    } else {
                        '_'
                    };

                    println!("* {:04x} ({}{})", addr, rd, wr);
                }
            }
        }

        Ok(false)
    }
}

struct Signal {
    sig: Arc<AtomicBool>,
}

impl Signal {
    fn new() -> Signal {
        let sig = Arc::new(AtomicBool::new(false));
        signal_hook::flag::register(signal_hook::SIGINT, sig.clone())
            .expect("Couldn't hook signal");
        Signal { sig }
    }

    fn signaled(&self) -> bool {
        self.sig.swap(false, Ordering::Relaxed)
    }
}
