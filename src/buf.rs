#![allow(unused)]

pub struct StreamBuf {
    pub(crate) inner: Vec<u8>,
    pub(crate) mark: usize,
}

impl StreamBuf {
    pub fn new() -> Self {
        StreamBuf {
            inner: Vec::new(),
            mark: 0,
        }
    }

    pub fn set(&mut self, idx: usize, byte: u8) {
        while self.inner.len() <= idx {
            self.inner.push(0);
        }
        self.inner[idx] = byte;
    }

    pub fn get(&self, idx: usize) -> Option<u8> {
        self.inner.get(idx).copied()
    }

    pub fn flip(&mut self) {
        self.mark = 0;
    }

    pub fn peek_next(&self) -> Option<u8> {
        self.get(self.mark)
    }

    pub fn write_next(&mut self, value: u8) {
        self.set(self.mark, value);
        self.mark += 1;
    }

    pub fn read_next(&mut self) -> Option<u8> {
        self.mark += 1;
        self.get(self.mark - 1)
    }
}

impl Default for StreamBuf {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::StreamBuf;

    #[test]
    fn buf_write_read() {
        let mut buf = StreamBuf::new();
        buf.set(0, 5);
        buf.set(1, 25);
        buf.set(2, 125);
        assert_eq!(buf.get(0), Some(5));
        assert_eq!(buf.get(1), Some(25));
        assert_eq!(buf.get(2), Some(125));
    }

    #[test]
    pub fn buf_next() {
        let mut buf = StreamBuf::new();
        buf.write_next(5);
        buf.write_next(25);
        buf.write_next(125);
        buf.flip();
        assert_eq!(buf.read_next(), Some(5));
        assert_eq!(buf.read_next(), Some(25));
        assert_eq!(buf.read_next(), Some(125));
    }
}
