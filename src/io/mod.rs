use std::fmt::Debug;
use std::io::{StderrLock, StdinLock, StdoutLock, Write};
use std::{cell::RefCell, io::BufRead, rc::Rc};

use crate::VERSION;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct InputOutput<R: BufRead + Debug, W: Write + Debug, E: Write + Debug> {
    stdin: Rc<RefCell<R>>,
    stdout: Rc<RefCell<W>>,
    stderr: Rc<RefCell<E>>,
}

impl Default for InputOutput<StdinLock<'_>, StdoutLock<'_>, StderrLock<'_>> {
    fn default() -> Self {
        let stdin = Box::leak(Box::new(std::io::stdin()));
        let stdout = Box::leak(Box::new(std::io::stdout()));
        let stderr = Box::leak(Box::new(std::io::stderr()));
        Self::new(stdin.lock(), stdout.lock(), stderr.lock())
    }
}

impl<R: BufRead + Debug, W: Write + Debug, E: Write + Debug>
    InputOutput<R, W, E>
{
    pub fn new(stdin: R, stdout: W, stderr: E) -> Self {
        Self {
            stdin: Rc::new(RefCell::new(stdin)),
            stdout: Rc::new(RefCell::new(stdout)),
            stderr: Rc::new(RefCell::new(stderr)),
        }
    }

    pub fn stdin_borrow(&self) -> std::cell::Ref<'_, R> {
        self.stdin.borrow()
    }

    pub fn stdout_borrow(&self) -> std::cell::Ref<'_, W> {
        self.stdout.borrow()
    }

    pub fn stderr_borrow(&self) -> std::cell::Ref<'_, E> {
        self.stderr.borrow()
    }

    pub fn stdin_borrow_mut(&self) -> std::cell::RefMut<'_, R> {
        self.stdin.borrow_mut()
    }

    pub fn stdout_borrow_mut(&self) -> std::cell::RefMut<'_, W> {
        self.stdout.borrow_mut()
    }

    pub fn stderr_borrow_mut(&self) -> std::cell::RefMut<'_, E> {
        self.stderr.borrow_mut()
    }

    pub fn read_line(&self) -> Option<String> {
        let mut buffer = String::new();
        let bytes_read = self.stdin.borrow_mut().read_line(&mut buffer);
        match bytes_read {
            Ok(0) => None, // EOF
            Ok(_) => Some(buffer),
            Err(err) => {
                let message = format!("Error reading from stdin: {err:?}\n");
                self.write_stderr(&message);
                None
            }
        }
    }

    pub fn write_stdout(&self, message: &str) {
        if let Err(err) = self.stdout.borrow_mut().write_all(message.as_bytes())
        {
            let message = format!("Error reading from stdout: {err:?}\n");
            self.write_stderr(&message);
        }
    }

    pub fn write_stderr(&self, message: &str) {
        if let Err(err) = self.stderr.borrow_mut().write_all(message.as_bytes())
        {
            eprintln!("Critical error: Unable to write to stderr: {err:?}",);
        }
    }

    pub fn flush_stdout(&self) {
        if let Err(err) = self.stdout.borrow_mut().flush() {
            self.write_stderr(&format!("Failed to flush stdout: {err:?}\n"));
        }
    }

    pub fn flush_stderr(&self) {
        if let Err(err) = self.stderr.borrow_mut().flush() {
            eprintln!("Critical error: Unable to flush stderr: {err:?}");
        }
    }

    pub fn fatal_error(&self, error: &str) -> ! {
        self.write_stderr(error);
        self.flush_stderr();
        #[cfg(not(test))]
        std::process::exit(1);
        #[cfg(test)]
        panic!("{error}\n");
    }

    pub fn check_ninja_version(&self, file: &[u8]) {
        let version = match file
            .chunks(4)
            .nth(1)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        {
            Some(version) => version,
            None => self.fatal_error("Failed to read version\n"),
        };
        if VERSION != version as u8 {
            self.fatal_error(
                "Error: code file does not have correct Ninja version\n",
            )
        }
    }

    pub fn check_instructions(&self, file: &[u8]) -> usize {
        match file
            .chunks(4)
            .nth(2)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        {
            Some(count) => match count.try_into() {
                Ok(count) => count,
                Err(err) => {
                    let message = format!(
                        "Error: failed to parse to usize from u32: {err}\n"
                    );
                    self.fatal_error(&message)
                }
            },
            None => {
                self.fatal_error("Error: failed to read instruction count\n")
            }
        }
    }

    pub fn check_ninja_format(&self, file: &[u8], arg: &str) {
        let ninja_binary_format = &[78, 74, 66, 70];
        if !file.starts_with(ninja_binary_format) {
            let message =
                format!("Error: file '{arg}' is not a Ninja binary\n");
            self.fatal_error(&message);
        }
    }

    pub fn check_variables(&self, file: &[u8]) -> usize {
        match file
            .chunks(4)
            .nth(3)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        {
            Some(count) => match count.try_into() {
                Ok(count) => count,
                Err(err) => {
                    let message = format!(
                        "Error: failed to parse to usize from u32: {err}\n"
                    );
                    self.fatal_error(&message)
                }
            },
            None => self
                .fatal_error("Error: failed to read global variable count\n"),
        }
    }

    pub fn read_file(&self, arg: &str) -> Vec<u8> {
        if arg.trim().is_empty() {
            self.fatal_error("Error: no code file specified\n");
        }

        std::fs::read(arg).unwrap_or_else(|err| {
            let error =
                format!("Error: cannot open code file '{arg}': {err}\n");
            self.fatal_error(&error);
        })
    }

    pub fn set_ninja_version(&self, file: &mut [u8]) {
        let version = match file.chunks_mut(4).nth(1).map(|c| {
            c[0] = VERSION;
            u32::from_le_bytes([c[0], c[1], c[2], c[3]])
        }) {
            Some(version) => version,
            None => self.fatal_error("Failed to read version\n"),
        };
        if VERSION != version as u8 {
            self.fatal_error(
                "Error: code file does not have correct Ninja version\n",
            )
        }
    }

    pub fn split_file_metadata(&self, file: &mut Vec<u8>) -> Vec<u8> {
        if file.len() < 16 {
            self.fatal_error("Error: code file is corrupted\n")
        }
        file.split_off(16)
    }

    pub fn unknown_arg(&self, arg: &str) {
        let message = format!(
            "unknown command line argument '{arg}', try './njvm --help'\n"
        );
        self.fatal_error(&message);
    }

    pub fn verify_arg(&self, arg: &str) {
        if arg.starts_with('-') {
            self.unknown_arg(arg)
        }
    }
}
