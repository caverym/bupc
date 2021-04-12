/// Copyright 2021 Avery Murray
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///     http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

use std::process::exit;

use processor::Processor;
use read::Program;

mod processor;
mod read;

pub const LOGOD: &str = "\
(\\ /)
( . .) Bunny Unit Processing Central
C(\")(\")\tPress Ctrl+D to execute!\n";

pub const LOGO: &str = "\
(\\ /)
( . .) Bunny Unit Processing Central
C(\")(\")\n";

pub const BUNNY: &str = "\
(\\ /)
( . .)
C(\")(\")";

fn main() {
    #[cfg(feature = "verbose")]
    eprintln!("main");

    let args: Vec<String> = std::env::args().collect();
    let argc: usize = args.len();
    let mut processor: Processor = Processor::new();
    let mut program = Program::new();

    if argc < 2 {
        if let Err(e) = program.geti() {
            eprintln!("{}", e);
            exit(processor.proc.process_val());
        }
    } else {
        match args[1].as_str() {
            "about" => {
                about();
                return;
            }

            "help" => {
                help();
                return;
            }
            _ => {
                if let Err(e) = program.get(&args) {
                    eprintln!("{}", e);
                    exit(processor.proc.process_val());
                }
            }
        }
    }

    processor.run(program);

    exit(processor.proc.process_val());
}

const ABOUT: &str = "\
An assembly-like interpreted language written using Rust,
by Avery Murray

Copyright 2021 Avery Murray

Licensed under the Apache License, Version 2.0 (the \"License\");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an \"AS IS\" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.";

const HELP: &str = "\
Usage: [OPTION]

about:\tview about information
help:\tview help information
[FILE]: exectes given script";


fn about() {
    println!("{}", LOGO);
    println!("{}", ABOUT);
}

fn help() {
    println!("{}", HELP);
}
