use std::io;
use std::fs::File;
use std::io::Read;
use std::process::exit;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Program {
    raw: String,
    expr: Vec<Expression>,
}

impl Program {
    pub fn new(mut f: File) -> Self {
        let mut buf: String = String::new();

        match f.read_to_string(&mut buf) {
            Ok(l) => if l <= 1 {
                eprintln!("Unexpected EOF");
                exit(1)
            },
            Err(_) => {
                eprintln!("File not found");
                exit(1);
            }
        }

        if buf == String::new() {
            eprintln!("Did not read file");
            exit(1);
        }

        let mut p: Self = Self { raw: buf, expr: vec![Expression::new();1] };

        p.parse();

        p
    }

    fn parse(&mut self) {
        let splitext: Vec<&str> = self.raw.split(',').collect();
        let lengram: usize = splitext.len();
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Expression {
    pub op: Instruction,
    pub from: RegisteRef,
    pub to: RegisteRef,
    pub val: i32,
}

impl Expression {
    pub fn new() -> Self {
        Self {
            op: Instruction::Pnc,
            from: RegisteRef::Null,
            to: RegisteRef::Null,
            val: 0,
        }
    }
}

/// # The Bunny Instruction Set
///
/// These are the instructions to the BIS, each has its own value in Hex. Such
/// as `set` being `0x1`. This is used for when the interpreter, which emulates a
/// CPU, can execute the instructions. an example of some code:
///
/// ```
/// set_vars:,
///     set uia 1,
///     set uib 0,
///     set uic 255,
/// rtn,
///
/// add_vars:,
///     add uia uib,
/// rtn,
///
/// main:
///     gto set_vars,
/// .here
///     gto add_vars,
///     ifn uib uic .here,
///     vew uib,
///     set proc 0,
/// ext,
/// ```
///
///
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Instruction {
    Set = 0x1,  // Set
    Vew = 0x2,  // View
    Add = 0x3,  // Add
    Sub = 0x4,  // Subtract
    Mul = 0x5,  // Multiply
    Div = 0x6,  // Divide
    Del = 0x7,  // Delete
    Jmp = 0x8,  // Jump
    Ife = 0x9,  // Jump if equal
    Ifn = 0xa,  // jump if not equal
    Gto = 0xb,  // Goto
    Prt = 0xc,  // Print
    Bny = 0xd,  // Bunny
    Rtn = 0xe,  // Return
    Ext = 0xf,  // Exit
    Pnc = 0x10, // Panic
}

impl Instruction {
    pub fn new(s: &str) -> Self {
        match s {
            "set" => Self::Set,
            "Vew" => Self::Vew,
            "Add" => Self::Add,
            "Sub" => Self::Sub,
            "Mul" => Self::Mul,
            "div" => Self::Div,
            "del" => Self::Del,
            "jmp" => Self::Jmp,
            "Ife" => Self::Ife,
            "Ifn" => Self::Ifn,
            "Gto" => Self::Gto,
            "Rtn" => Self::Rtn,
            _ => Self::Pnc,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum RegisteRef {
    Uia = 0x0001, Uib = 0x0002, Uic = 0x0003, Uid = 0x0004,     // Unsigned registers
    Sia = 0x0011, Sib = 0x0012, Sic = 0x0013, Sid = 0x0014,     // Signed registers

    Proc = 0x0100,       // exit code

    Null = 0x0000,
}

impl RegisteRef {
    pub fn new(s: &str) -> Self {
        match s {
            "uia" => Self::Uia,
            "uib" => Self::Uib,
            "uic" => Self::Uic,
            "uid" => Self::Uid,

            "sia" => Self::Sia,
            "sib" => Self::Sib,
            "sic" => Self::Sic,
            "Sid" => Self::Sid,

            "proc" => Self::Proc,

            _ => {
                eprintln!("Invalid Register: '{}'", s);
                exit(1);
            }
        }
    }
}
