use std::io;
use std::process;

struct Interpreter {
    register: Vec<String>,
    src: Vec<char>,
}

impl Interpreter {
    fn new(size: usize, input: &str) -> Self {
        Self {
            register: vec!["".to_string(); 32],
            src: input.chars().collect(),
        }
    }

    fn run_code(&mut self) {
        while !self.src.is_empty() {
            match self.src[0] {
                '=' => self.parse_assignment(),
                '#' => self.parse_stdout(),
                '<' => self.parse_loop(),
                _ => unimplemented!(),
            }
        }
    }

    fn parse_loop(&mut self) {
        let mut output = String::new();
        self.consume('[');
        while self.src[0] != ']' {
            match self.src[0] {
                '#' => output = self.parse_reg_access(),
                _ => unimplemented!(),
            }
        }

        self.expect(']');
        self.expect('<');

        let mut times = match self.src[0] {
            '[' => {
                let reg_idx = self.get_reg();

                self.expect('>');
                reg_idx
            }
            _ => {
                let mut reg_idx = String::new();

                while self.src[0] != '>' {
                    if !self.src[0].is_digit(10) {
                        process::exit(1);
                    }
                    reg_idx.push(self.src.remove(0));
                }

                self.expect('>');
                reg_idx.parse::<usize>().unwrap()
            }
        };

        while times > 0 {
            print!("{}", output);
            times -= 1
        }
    }

    fn get_reg(&mut self) -> usize {
        self.expect('[');

        let mut reg_val = String::new();

        while self.src[0] != ']' {
            if !self.src[0].is_digit(10) {
                std::process::exit(1)
            }

            reg_val.push(self.src.remove(0));
        }

        self.expect(']');

        reg_val.parse::<usize>().unwrap()
    }

    fn parse_reg_access(&mut self) -> String {
        self.consume('[');
        let mut idx = String::new();

        while self.src[0] != ']' {
            if !self.src[0].is_digit(10) {
                std::process::exit(1);
            }

            idx.push(self.src.remove(0));
        }

        self.expect('>');
        let idx = idx.parse::<usize>().unwrap();

        format!("{}", self.register[idx - 1])
    }

    fn parse_stdout(&mut self) {
        self.consume('<');
        let mut idx = String::new();

        while self.src[0] != '>' {
            if !self.src[0].is_digit(10) {
                std::process::exit(1);
            }

            idx.push(self.src.remove(0));
        }

        self.expect('>');
        let idx = idx.parse::<usize>().unwrap();

        print!("{}", self.register[idx - 1])
    }

    fn parse_assignment(&mut self) {
        self.consume('[');
        let mut idx = String::new();
        let mut val = String::new();

        while self.src[0] != ']' {
            if !self.src[0].is_digit(10) {
                std::process::exit(1);
            }
            idx.push(self.src.remove(0));
        }

        let idx = idx.parse::<usize>().unwrap();

        self.expect(']');
        self.expect('\"');

        while self.src[0] != '\"' {
            val.push(self.src.remove(0))
        }

        self.expect('\"');

        self.register[idx - 1] = val;
    }

    fn expect(&mut self, c: char) {
        if self.src[0] != c {
            std::process::exit(1)
        } else {
            self.src.remove(0);
        }
    }

    fn consume(&mut self, c: char) -> char {
        self.src.remove(0);
        if self.src[0] != c {
            std::process::exit(1)
        } else {
            self.src.remove(0)
        }
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read input");
    let mut qivol = Interpreter::new(32, &buffer.trim());

    qivol.run_code();
}
