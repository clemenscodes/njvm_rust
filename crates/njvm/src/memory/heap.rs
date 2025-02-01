use std::{
    cell::RefCell,
    fmt::Debug,
    io::{BufRead, StderrLock, StdinLock, StdoutLock, Write},
    rc::Rc,
};

use crate::io::InputOutput;

pub const DEFAULT_HEAP_MEMORY: usize = 8192;
pub const KIBI: usize = 1024;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Heap<R: BufRead + Debug, W: Write + Debug, E: Write + Debug> {
    pub io: Rc<RefCell<InputOutput<R, W, E>>>,
    pub size: usize,      // Number of allocations
    pub memory: usize,    // Total heap memory size
    pub bytes: usize,     // Bytes used
    pub available: usize, // Free bytes remaining
    pub used: usize,      // Bytes in use
    pub data: Box<[u8]>,  // The heap memory
    pub begin: usize,     // Start of memory
    pub active: usize,    // Active half
    pub passive: usize,   // Passive half
    pub next: usize,      // Next free byte
}

impl Default for Heap<StdinLock<'_>, StdoutLock<'_>, StderrLock<'_>> {
    fn default() -> Self {
        let io = InputOutput::default();
        Self::new(Rc::new(RefCell::new(io)), DEFAULT_HEAP_MEMORY)
    }
}

impl<R: BufRead + Debug, W: Write + Debug, E: Write + Debug> Heap<R, W, E> {
    pub fn new(io: Rc<RefCell<InputOutput<R, W, E>>>, memory: usize) -> Self {
        let bytes = memory * KIBI;
        let available = bytes / 2;
        let data = vec![0u8; bytes].into_boxed_slice();

        Self {
            io,
            size: 0,
            used: 0,
            memory,
            bytes,
            available,
            begin: 0,
            active: 0,
            passive: bytes / 2,
            next: 0,
            data,
        }
    }

    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        if self.next + size > self.memory {
            None
        } else {
            let start = self.next;
            self.next += size;
            self.available -= size;
            self.used += size;
            Some(start)
        }
    }

    pub fn get_mut_slice(
        &mut self,
        offset: usize,
        len: usize,
    ) -> Option<&mut [u8]> {
        if offset + len > self.memory {
            None
        } else {
            Some(&mut self.data[offset..offset + len])
        }
    }

    pub fn reset(&mut self) {
        self.next = 0;
        self.available = self.memory;
        self.used = 0;
    }
}
