//!
//! A non-thread-safe ringbuffer, with handy functions for audio
//! The interface is loosely inspired by the threadsafe ringbuffer of crate rb
//! https://github.com/klingtnet/rb
//!
//! This ringbuffer is resizeable.

use std::cmp;
use std::fmt;

// TODO: maybe use Cell or RefCell for read and write pos in order not to haveto use &mut self?

pub struct RingBuffer<T> {
    buf: Vec<T>,
    size: usize,
    write_pos: usize,
    read_pos: usize,
}

#[derive(Debug)]
pub enum RingBufferError {
    Full,
    Empty,
}

impl fmt::Display for RingBufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &RingBufferError::Full => write!(f, "No free slots in the buffer"),
            &RingBufferError::Empty => write!(f, "Buffer is empty"),
        }
    }
}

pub type Result<T> = ::std::result::Result<T, RingBufferError>;

impl<T: Clone + Default> RingBuffer<T> {
    pub fn new(size: usize) -> RingBuffer<T> {
        let buffer = vec![T::default(); size + 1];
        RingBuffer {
            buf: buffer,
            size: size + 1,
            write_pos: 0,
            read_pos: 0,
        }
    }

    pub fn clear(&mut self) {
        self.buf.iter_mut().map(|_| T::default()).count();
        self.read_pos = 0;
        self.write_pos = 0;
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.size - 1
    }

    #[inline(always)]
    pub fn slots_free(&self) -> usize {
        if self.write_pos < self.read_pos {
            self.read_pos - self.write_pos - 1
        } else {
            self.capacity() - self.write_pos + self.read_pos
        }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.slots_free() == self.capacity()
    }

    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.slots_free() == 0
    }

    #[inline(always)]
    pub fn count(&self) -> usize {
        self.capacity()
            .checked_sub(self.slots_free())
            .expect(&format!(
                "Underflow: capacity {}; slots free {}!",
                self.capacity(),
                self.slots_free()
            ))
    }

    pub fn write(&mut self, data: &[T]) -> Result<usize> {
        if data.len() == 0 {
            return Ok(0);
        }
        if self.is_full() {
            return Err(RingBufferError::Full);
        }
        let free_slots1 = self.slots_free();
        debug_assert!(self.slots_free() <= self.capacity());
        let cnt = cmp::min(data.len(), self.slots_free());
        {
            let buf = self.buf.as_mut_slice();
            let buf_len = buf.len();
            for idx in 0..cnt {
                buf[self.write_pos] = data[idx].clone();
                self.write_pos = (self.write_pos + 1) % buf_len;
            }
        }
        debug_assert!(free_slots1 >= self.slots_free());
        debug_assert!(self.slots_free() <= self.capacity());
        Ok(cnt)
    }

    pub fn fill(&mut self, count: usize, value: T) -> Result<usize> {
        if count == 0 {
            return Ok(0);
        }
        if self.is_full() {
            return Err(RingBufferError::Full);
        }
        debug_assert!(self.slots_free() <= self.capacity());
        let cnt = cmp::min(count, self.slots_free());
        {
            let buf = self.buf.as_mut_slice();
            let buf_len = buf.len();

            for _ in 0..cnt {
                buf[self.write_pos] = value.clone();
                self.write_pos = (self.write_pos + 1) % buf_len;
            }
        }
        debug_assert!(self.slots_free() <= self.capacity());
        Ok(cnt)
    }

    pub fn skip_pending(&mut self) -> Result<usize> {
        if self.is_empty() {
            Err(RingBufferError::Empty)
        } else {
            let count = self.count();
            self.read_pos = self.write_pos;
            debug_assert!(self.slots_free() <= self.capacity());;
            Ok(count)
        }
    }

    pub fn skip(&mut self, cnt: usize) -> Result<usize> {
        if self.is_empty() {
            Err(RingBufferError::Empty)
        } else {
            let count = cmp::min(cnt, self.count());
            let buf_len = self.buf.len();
            self.read_pos = (self.read_pos + count) % buf_len;
            Ok(count)
        }
    }

    pub fn get(&mut self, data: &mut [T]) -> Result<usize> {
        if data.len() == 0 {
            return Ok(0);
        }
        if self.is_empty() {
            return Err(RingBufferError::Empty);
        }
        debug_assert!(self.slots_free() <= self.capacity());;
        let cnt = cmp::min(data.len(), self.count());
        let buf_len = self.buf.len();

        for idx in 0..cnt {
            let buf_idx = (idx + self.read_pos) % buf_len;
            data[idx] = self.buf[buf_idx].clone();
        }
        debug_assert!(self.slots_free() <= self.capacity());
        Ok(cnt)
    }

    pub fn read(&mut self, data: &mut [T]) -> Result<usize> {
        if data.len() == 0 {
            return Ok(0);
        }
        if self.is_empty() {
            return Err(RingBufferError::Empty);
        }
        debug_assert!(self.slots_free() <= self.capacity());
        let cnt = cmp::min(data.len(), self.count());
        let buf_len = self.buf.len();

        for idx in 0..cnt {
            self.read_pos = (idx + self.read_pos) % buf_len;
            data[idx] = self.buf[self.read_pos].clone();
        }
        debug_assert!(self.slots_free() <= self.capacity());
        Ok(cnt)
    }
}

impl<T> fmt::Debug for RingBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Size: {}; write_pos: {}; read_pos: {}",
            self.size, self.write_pos, self.read_pos
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BUFFER_SIZE: usize = 10000;

    #[test]
    #[ignore]
    fn ringbuffer_test() {
        let mut ring_buffer = RingBuffer::new(BUFFER_SIZE);
        let mut buffer = Vec::with_capacity(BUFFER_SIZE);

        let mut i = 2;
        loop {
            if let Err(e) = ring_buffer.fill(i, 0) {
                println!("Stopping: {}", e); //Won't be displayed except if --nocapture
                break;
            }
            buffer.resize(i / 2, 0);

            if let Err(e) = ring_buffer.read(buffer.as_mut_slice()) {
                println!("Stopping: {}", e);
                break;
            }
            i += 1
        }
    }
}
