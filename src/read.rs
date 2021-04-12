use std::io::{Error, Read};

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<(String, usize)>,
    raw_thread: String,
    pub thread: Vec<Vec<String>>,
    pub thread_len: usize,
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn geti(&mut self) -> Result<(), Error> {
        println!("{}", crate::LOGOD);
        let s = stdin()?.trim().to_string();
        self.raw_thread = s;

        self.split();
        self.thread_len = self.thread.len();

        for i in 0..self.thread_len {
            let s = self.thread[i][0].clone();

            if s.contains(':') {
                #[cfg(feature = "verbose")]
                eprintln!("Assigning '{}' at {}", s, i);

                let mut vec = vec![(s, i + 1); 1];
                self.functions.append(&mut vec);
            }
        }

        Ok(())
    }

    pub fn get(&mut self, args: &[String]) -> Result<(), Error> {
        let f = std::fs::read_to_string(&args[1])?;
        self.raw_thread = f.trim().to_string();

        self.split();
        self.thread_len = self.thread.len();

        for i in 0..self.thread_len {
            let s = self.thread[i][0].clone();

            if s.contains(':') {
                #[cfg(feature = "verbose")]
                eprintln!("Assigning '{}' at {}", s, i);

                let mut vec = vec![(s, i + 1); 1];
                self.functions.append(&mut vec);
            }
        }

        Ok(())
    }

    pub fn lines(&self) -> Vec<String> {
        let mut l: Vec<&str> = self.raw_thread.split(',').collect();
        l.retain(|x| !x.is_empty());
        let l: Vec<&str> = l.iter().map(|x| x.trim()).collect();
        let a: Vec<String> = l.iter().map(|x| x.to_string()).collect();
        a
    }

    pub fn split(&mut self) {
        let v = self.lines();
        let vn: Vec<Vec<&str>> = v.iter().map(|x| x.split(' ').collect::<Vec<_>>()).collect();
        let out: Vec<Vec<String>> = vn
            .iter()
            .map(|x| x.iter().map(|s| s.to_string()).collect())
            .collect();

        self.thread_len = out.len();
        self.thread = out;
    }
}

impl Default for Program {
    fn default() -> Self {
        Self {
            raw_thread: "exit".into(),
            functions: Vec::new(),
            thread: Vec::new(),
            thread_len: 0,
        }
    }
}

pub fn stdin() -> Result<String, Error> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}
