use std::process::exit;

use crate::read::Program;

const NA_C: i32 = -5;
const NA_N: i32 = -6;
const NA_R: i32 = -7;

#[derive(Debug)]
pub struct Processor {
    pub proc: Register,
    pub counter: usize,
    pub past: usize,
    pub thread: Program,

    uia: Register,
    sia: Register,

    uib: Register,
    sib: Register,

    uic: Register,
    sic: Register,

    uid: Register,
    sid: Register,
}

impl Processor {
    pub fn new() -> Self {
        #[cfg(feature = "verbose")]
        eprintln!("Processor::new()");

        Self {
            proc: Register::new_process(),
            counter: 1,
            past: 0,
            thread: Program::new(),

            uia: Register::new_unsigned(),
            sia: Register::new_signed(),

            uib: Register::new_unsigned(),
            sib: Register::new_signed(),

            uic: Register::new_unsigned(),
            sic: Register::new_signed(),

            uid: Register::new_unsigned(),
            sid: Register::new_signed(),
        }
    }

    pub fn run(&mut self, program: Program) {
        #[cfg(feature = "verbose")]
        eprintln!("Processor::run()");

        self.thread = program;

        self.goto("main:");

        while self.counter <= self.thread.thread_len {
            let command = self.thread.thread[self.counter - 1].clone();
            #[cfg(feature = "verbose")]
            eprintln!("command: {:?}", command);
            match command.as_slice() {
                [cmd, dest, v, n] => self.four(&cmd, &dest, &v, &n),
                [cmd, dest, v] => self.three(&cmd, &dest, &v),
                [cmd, dest] => self.two(&cmd, &dest),
                [cmd] => self.one(&cmd),
                _ => {
                    eprintln!("NaC:\t{:?}", command);
                    exit(NA_C);
                }
            }
            self.jump(self.counter + 1);
        }
    }

    fn one(&mut self, cmd: &str) {
        if cmd.contains(':') {
            return;
        }

        #[cfg(feature = "verbose")]
        eprintln!("Processor::one({})", cmd);

        match cmd {
            "return" => self.jump(self.past),
            "exit" => exit(self.proc.process_val()),
            "bunny" => self.printl(crate::BUNNY),
            "printl" => self.printl(""),
            _ => {
                eprintln!("NaC:\t{}", cmd);
                exit(NA_C);
            }
        }
    }

    fn two(&mut self, cmd: &str, dest: &str) {
        #[cfg(feature = "verbose")]
        eprintln!("Processor::two({}, {})", cmd, dest);

        match cmd {

            "view" => self.view(dest),
            "jump" => self.jump_str(dest),
            "goto" => self.goto(dest),
            "print" => self.print(dest),
            "printl" => self.printl(dest),
            "del" => self.del(dest),
            _ => {
                eprintln!("NaC:\t{}", cmd);
                exit(NA_C);
            }
        }
    }

    fn three(&mut self, cmd: &str, dest: &str, v: &str) {
        #[cfg(feature = "verbose")]
        eprintln!("Processor::three({}, {}, {})", cmd, dest, v);

        match cmd {
            "set" => self.set(&dest, &v),
            "add" => self.add(&dest, &v),
            "sub" => self.sub(dest, v),
            "move" => self.r#move(&dest, &v),
            "print" => self.print(&format!("{} {}", dest, v)),
            "printl" => self.print(&format!("{} {}\n", dest, v)),
            _ => {
                eprintln!("NaC:\t{}", cmd);
                exit(NA_C);
            }
        }
    }

    fn four(&mut self, cmd: &str, dest: &str, v: &str, n: &str) {
        #[cfg(feature = "verbose")]
        eprintln!("Processor::four({}, {}, {}, {})", cmd, dest, v, n);

        match cmd {
            "jump_eq" => self.ife(dest, v, n),
            "jump_neq" => self.ifn(dest, v, n),
            "print" => self.print(&format!("{} {} {}", dest, v, n)),
            "printl" => self.print(&format!("{} {} {}\n", dest, v, n)),
            _ => {
                eprintln!("NaC:\t{}", cmd);
                exit(NA_C);
            }
        }
    }

    fn ife(&mut self, dest: &str, v: &str, to: &str) {
        #[cfg(feature = "verbose")]
        eprintln!("Processor::ife({}, {}, {})", dest, v, to);

        let b = if dest.contains("ui") && v.contains("ui") {
            let n = match dest {
                "uia" => self.uia.value.numu8(),
                "uib" => self.uib.value.numu8(),
                "uic" => self.uic.value.numu8(),
                "uid" => self.uid.value.numu8(),
                _ => {
                    eprintln!("NaR\t{}", dest);
                    exit(NA_R);
                }
            };

            let nn = match v {
                "uia" => self.uia.value.numu8(),
                "uib" => self.uib.value.numu8(),
                "uic" => self.uic.value.numu8(),
                "uid" => self.uid.value.numu8(),
                _ => {
                    eprintln!("NaR\t{}", dest);
                    exit(NA_R);
                }
            };

            n == nn
        } else if dest.contains("si") && v.contains("si") {
            let n = match dest {
                "sia" => self.sia.value.numi8(),
                "sib" => self.sib.value.numi8(),
                "sic" => self.sic.value.numi8(),
                "sid" => self.sid.value.numi8(),
                _ => {
                    eprintln!("NaR\t{}", dest);
                    exit(NA_R);
                }
            };

            let nn = match v {
                "sia" => self.sia.value.numi8(),
                "sib" => self.sib.value.numi8(),
                "sic" => self.sic.value.numi8(),
                "sid" => self.sid.value.numi8(),
                _ => {
                    eprintln!("NaR\t{}", dest);
                    exit(NA_R);
                }
            };
            n == nn
        } else {
            eprintln!("NaR:\t{}/{}", dest, v);
            exit(NA_R);
        };

        if b {
            let jump = match to.parse::<usize>() {
                Ok(aa) => aa - 1,
                Err(_) => {
                    eprintln!("NaN:\t{}", to);
                    exit(NA_N);
                }
            };

            self.jump(jump);
        }
    }

    fn ifn(&mut self, dest: &str, v: &str, to: &str) {
        #[cfg(feature = "verbose")]
        eprintln!("Processor::ifn({}, {}, {})", dest, v, to);

        let b = if dest.contains("ui") && v.contains("ui") {
            let n = match dest {
                "uia" => self.uia.value.numu8(),
                "uib" => self.uib.value.numu8(),
                "uic" => self.uic.value.numu8(),
                "uid" => self.uid.value.numu8(),
                _ => {
                    eprintln!("NaR\t{}", dest);
                    exit(NA_R);
                }
            };

            let nn = match v {
                "uia" => self.uia.value.numu8(),
                "uib" => self.uib.value.numu8(),
                "uic" => self.uic.value.numu8(),
                "uid" => self.uid.value.numu8(),
                _ => {
                    eprintln!("NaR\t{}", dest);
                    exit(NA_R);
                }
            };

            n == nn
        } else if dest.contains("si") && v.contains("si") {
            let n = match dest {
                "sia" => self.sia.value.numi8(),
                "sib" => self.sib.value.numi8(),
                "sic" => self.sic.value.numi8(),
                "sid" => self.sid.value.numi8(),
                _ => {
                    eprintln!("NaR\t{}", dest);
                    exit(NA_R);
                }
            };

            let nn = match v {
                "sia" => self.sia.value.numi8(),
                "sib" => self.sib.value.numi8(),
                "sic" => self.sic.value.numi8(),
                "sid" => self.sid.value.numi8(),
                _ => {
                    eprintln!("NaR\t{}", dest);
                    exit(NA_R);
                }
            };
            n == nn
        } else {
            eprintln!("NaR:\t{}/{}", dest, v);
            exit(NA_R);
        };

        if !b {
            let jump = match to.parse::<usize>() {
                Ok(aa) => aa - 1,
                Err(_) => {
                    eprintln!("NaN:\t{}", to);
                    exit(NA_N);
                }
            };

            self.jump(jump);
        }
    }

    fn set(&mut self, dest: &str, v: &str) {
        #[cfg(feature = "verbose")]
        eprintln!("Processor::set({}, {})", dest, v);

        match dest {
            "proc" => self.proc.set_process(v),

            "uia" => self.uia.set_u8_str(v),
            "sia" => self.sia.set_i8_str(v),

            "uib" => self.uib.set_u8_str(v),
            "sib" => self.sib.set_i8_str(v),

            "uic" => self.uic.set_u8_str(v),
            "sic" => self.sic.set_i8_str(v),

            "uid" => self.uid.set_u8_str(v),
            "sid" => self.sid.set_i8_str(v),

            _ => {
                eprintln!("NaR:\t{}", dest);
                exit(NA_R);
            }
        }
    }

    fn jump(&mut self, v: usize) {
        #[cfg(feature = "verbose")]
        eprintln!("Processor::jump({})", v);
        self.counter = v;
    }

    fn jump_str(&mut self, v: &str) {
        #[cfg(feature = "verbose")]
        eprintln!("Processor::jump_str({})", v);
        let n = match v.parse::<usize>() {
            Ok(a) => a,
            Err(_) => {
                eprintln!("NaN:\t{}", v);
                exit(NA_N);
            }
        };

        self.jump(n);
    }

    fn goto(&mut self, dest: &str) {
        #[cfg(feature = "verbose")]
        eprintln!("Processor::goto({})\n{:?}", dest, self.thread.functions);

        for i in self.thread.functions.clone() {
            #[cfg(feature = "verbose")]
            eprintln!("Looking at: {:?}", i);
            let (function, loc) = i;
            if dest == function || dest == loc.to_string() {
                self.past = self.counter;
                self.jump(loc);
                return;
            }
        }
        eprintln!("NaF:\t{:?}", dest);
        exit(NA_C);
    }

    fn view(&self, dest: &str) {
        #[cfg(feature = "verbose")]
        eprintln!("Processor::view({})", dest);

        match dest {
            "proc" => println!("{:?}", self.proc.process_val()),

            "uia" => println!("{:?}", self.uia.valu8()),
            "sia" => println!("{:?}", self.sia.vali8()),

            "uib" => println!("{:?}", self.uib.valu8()),
            "sib" => println!("{:?}", self.sib.vali8()),

            "uic" => println!("{:?}", self.uic.valu8()),
            "sic" => println!("{:?}", self.sic.vali8()),

            "uid" => println!("{:?}", self.uid.valu8()),
            "sid" => println!("{:?}", self.sid.vali8()),

            _ => {
                eprintln!("NaR\t{}", dest);
                exit(NA_R);
            }
        }
    }

    fn print(&self, dest: &str) {
        #[cfg(feature = "verbose")]
        eprintln!("Processor::print({})", dest);
        print!("{}", dest);
    }

    fn printl(&self, dest: &str) {
        #[cfg(feature = "verbose")]
        eprintln!("Processor::print({})", dest);
        println!("{}", dest);
    }

    fn add(&mut self, dest: &str, v: &str) {
        #[cfg(feature = "verbose")]
        eprintln!("Processor::add({}, {})", dest, v);

        if dest == "proc" {
            eprintln!("NaR:\t{}", dest);
            exit(NA_R);
        }

        if dest.contains("ui") && v.contains("ui") {
            let n = match dest {
                "uia" => self.uia.value.numu8(),
                "uib" => self.uib.value.numu8(),
                "uic" => self.uic.value.numu8(),
                "uid" => self.uid.value.numu8(),
                _ => {
                    eprintln!("NaR\t{}", dest);
                    exit(NA_R);
                }
            };

            match v {
                "uia" => self.uia.value = RKVal::U8(self.uia.value.numu8() + n),
                "uib" => self.uib.value = RKVal::U8(self.uib.value.numu8() + n),
                "uic" => self.uic.value = RKVal::U8(self.uic.value.numu8() + n),
                "uid" => self.uid.value = RKVal::U8(self.uid.value.numu8() + n),
                _ => {
                    eprintln!("NaR\t{}", dest);
                    exit(NA_R);
                }
            };
        } else if dest.contains("si") && v.contains("si") {
            let n = match dest {
                "sia" => self.sia.value.numi8(),
                "sib" => self.sib.value.numi8(),
                "sic" => self.sic.value.numi8(),
                "sid" => self.sid.value.numi8(),
                _ => {
                    eprintln!("NaR\t{}", dest);
                    exit(NA_R);
                }
            };

            match v {
                "sia" => self.sia.value = RKVal::I8(self.sia.value.numi8() + n),
                "sib" => self.sib.value = RKVal::I8(self.sib.value.numi8() + n),
                "sic" => self.sic.value = RKVal::I8(self.sic.value.numi8() + n),
                "sid" => self.sid.value = RKVal::I8(self.sid.value.numi8() + n),
                _ => {
                    eprintln!("NaR\t{}", dest);
                    exit(NA_R);
                }
            };
        }
    }

    fn sub(&mut self, dest: &str, v: &str) {
        #[cfg(feature = "verbose")]
        eprintln!("Processor::add({}, {})", dest, v);

        if dest == "proc" {
            eprintln!("NaR:\t{}", dest);
            exit(NA_R);
        }

        if dest.contains("ui") && v.contains("ui") {
            let n = match dest {
                "uia" => self.uia.value.numu8(),
                "uib" => self.uib.value.numu8(),
                "uic" => self.uic.value.numu8(),
                "uid" => self.uid.value.numu8(),
                _ => {
                    eprintln!("NaR\t{}", dest);
                    exit(NA_R);
                }
            };

            match v {
                "uia" => self.uia.value = RKVal::U8(self.uia.value.numu8() - n),
                "uib" => self.uib.value = RKVal::U8(self.uib.value.numu8() - n),
                "uic" => self.uic.value = RKVal::U8(self.uic.value.numu8() - n),
                "uid" => self.uid.value = RKVal::U8(self.uid.value.numu8() - n),
                _ => {
                    eprintln!("NaR\t{}", dest);
                    exit(NA_R);
                }
            };
        } else if dest.contains("si") && v.contains("si") {
            let n = match dest {
                "sia" => self.sia.value.numi8(),
                "sib" => self.sib.value.numi8(),
                "sic" => self.sic.value.numi8(),
                "sid" => self.sid.value.numi8(),
                _ => {
                    eprintln!("NaR\t{}", dest);
                    exit(NA_R);
                }
            };

            match v {
                "sia" => self.sia.value = RKVal::I8(self.sia.value.numi8() - n),
                "sib" => self.sib.value = RKVal::I8(self.sib.value.numi8() - n),
                "sic" => self.sic.value = RKVal::I8(self.sic.value.numi8() - n),
                "sid" => self.sid.value = RKVal::I8(self.sid.value.numi8() - n),
                _ => {
                    eprintln!("NaR\t{}", dest);
                    exit(NA_R);
                }
            };
        }
    }

    fn r#move(&mut self, dest: &str, v: &str) {
        let buf;

        buf = match dest {
            "proc" => self.proc.value,
            "uia" => self.uia.value,
            "sia" => self.sia.value,

            "uib" => self.uib.value,
            "sib" => self.sib.value,

            "uic" => self.uic.value,
            "sic" => self.sic.value,

            "uid" => self.uid.value,
            "sid" => self.sid.value,

            _ => {
                eprintln!("NaR\t{}", dest);
                exit(NA_R);
            }
        };

        match v {
            "proc" => self.proc.value = buf,

            "uia" => self.uia.value = buf,
            "sia" => self.sia.value = buf,

            "uib" => self.uib.value = buf,
            "sib" => self.sib.value = buf,

            "uic" => self.uic.value = buf,
            "sic" => self.sic.value = buf,
            "uid" => self.uid.value = buf,

            "sid" => self.sid.value = buf,

            _ => {
                eprintln!("NaR\t{}", dest);
                exit(NA_R);
            }
        }
    }

    fn del(&mut self, dest: &str) {
        match dest {
            "proc" => self.proc.value = RKVal::Null,

            "uia" => self.uia.value = RKVal::Null,
            "sia" => self.sia.value = RKVal::Null,

            "uib" => self.uib.value = RKVal::Null,
            "sib" => self.sib.value = RKVal::Null,

            "uic" => self.uic.value = RKVal::Null,
            "sic" => self.sic.value = RKVal::Null,

            "uid" => self.uid.value = RKVal::Null,
            "sid" => self.sid.value = RKVal::Null,

            _ => {
                eprintln!("NaR\t{}", dest);
                exit(NA_R);
            }
        }
    }
}

#[derive(Debug)]
pub struct Register {
    kind: RegisterKind,
    value: RKVal,
}

impl Register {
    pub fn new_unsigned() -> Self {
        #[cfg(feature = "verbose")]
        eprintln!("Register::new_unsigned()");

        Self {
            kind: RegisterKind::U8,
            value: RKVal::Null,
        }
    }

    pub fn new_signed() -> Self {
        #[cfg(feature = "verbose")]
        eprintln!("Register::new_signed()");

        Self {
            kind: RegisterKind::I8,
            value: RKVal::Null,
        }
    }

    pub fn new_process() -> Self {
        #[cfg(feature = "verbose")]
        eprintln!("Register::new_process()");

        Self {
            kind: RegisterKind::Process,
            value: RKVal::Null,
        }
    }

    pub fn process_val(&self) -> i32 {
        #[cfg(feature = "verbose")]
        eprintln!("Register::process_val()");

        match self.value {
            RKVal::I32(v) => v,
            _ => {
                eprintln!("NaN:\t{:?}", self.value);
                exit(NA_N);
            }
        }
    }

    pub fn valu8(&self) -> u8 {
        match self.value {
            RKVal::U8(v) => v,
            _ => {
                eprintln!("NaN:\t{:?}", self.value);
                exit(NA_N);
            }
        }
    }

    pub fn vali8(&self) -> i8 {
        match self.value {
            RKVal::I8(v) => v,
            _ => {
                eprintln!("NaN:\t{:?}", self.value);
                exit(NA_N);
            }
        }
    }

    pub fn set_process(&mut self, v: &str) {
        #[cfg(feature = "verbose")]
        eprintln!("Register::set_process({})", v);

        let num = match v.parse::<i32>() {
            Ok(i) => i,
            Err(_) => {
                eprintln!("NaN:\t{}", v);
                exit(NA_N);
            }
        };

        match self.kind {
            RegisterKind::Process => self.value = RKVal::I32(num),
            _ => {
                eprintln!("set_process: invalid register:\t{:?}", self);
                exit(NA_R);
            }
        }
    }

    pub fn set_u8_str(&mut self, v: &str) {
        #[cfg(feature = "verbose")]
        eprintln!("Register::set_u8_str({})", v);

        let num = match v.parse::<u8>() {
            Ok(i) => i,
            Err(_) => {
                eprintln!("NaN:\t{}", v);
                exit(NA_N);
            }
        };

        match self.kind {
            RegisterKind::U8 => self.value = RKVal::U8(num),
            _ => {
                eprintln!("set_u8_str: invalid register:\t{:?}", self);
                exit(NA_R);
            }
        }
    }

    #[cfg(feature = "raw")]
    pub fn set_u8(&mut self, v: u8) {
        #[cfg(feature = "verbose")]
        eprintln!("Register::set_u8({})", v);

        match self.kind {
            RegisterKind::U8 => self.value = RKVal::U8(v),
            _ => {
                eprintln!("set_u8: invalid register:\t{:?}", self);
                exit(NA_R);
            }
        }
    }

    #[cfg(feature = "raw")]
    pub fn set_i8(&mut self, v: i8) {
        #[cfg(feature = "verbose")]
        eprintln!("Register::set_i8({})", v);

        match self.kind {
            RegisterKind::I8 => self.value = RKVal::I8(v),
            _ => {
                eprintln!("set_i8: invalid register:\t{:?}", self);
                exit(NA_R);
            }
        }
    }

    pub fn set_i8_str(&mut self, v: &str) {
        #[cfg(feature = "verbose")]
        eprintln!("Register::set_i8_str({})", v);

        let num = match v.parse::<i8>() {
            Ok(i) => i,
            Err(_) => {
                eprintln!("NaN:\t{}", v);
                exit(NA_N);
            }
        };

        match self.kind {
            RegisterKind::I8 => self.value = RKVal::I8(num),
            _ => {
                eprintln!("set_i8_str: invalid register:\t{:?}", self);
                exit(NA_R);
            }
        }
    }
}

#[derive(Debug)]
pub enum RegisterKind {
    U8,
    I8,
    Process,
}

#[derive(Debug, Clone, Copy)]
pub enum RKVal {
    U8(u8),
    I8(i8),
    I32(i32),
    Null,
}

impl RKVal {
    pub fn numu8(&self) -> u8 {
        #[cfg(feature = "verbose")]
        eprintln!("RKVal::numu8()");

        match self {
            RKVal::U8(n) => *n,
            _ => {
                eprintln!("numu8: invalid register:\t{:?}", self);
                exit(NA_R)
            }
        }
    }

    pub fn numi8(&self) -> i8 {
        #[cfg(feature = "verbose")]
        eprintln!("RKVal::numi8");

        match self {
            RKVal::I8(n) => *n,
            _ => {
                eprintln!("numi8: invalid register:\t{:?}", self);
                exit(NA_R)
            }
        }
    }
}
