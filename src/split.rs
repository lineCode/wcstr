
use ::std;
use ::WCStr;

/// Created with method ```.split(delim)```
#[derive(Debug)]
pub struct Split {
    buffer: Vec<u16>,
    offset: usize,
}

pub fn new(buffer: Vec<u16>, delim: u16) -> Split {
    let mut buffer = buffer;
    *buffer.last_mut().unwrap() = delim;
    Split {
        buffer: buffer,
        offset: 0,
    }
}

impl Split {
    /// Get iterator.
    pub fn iter(&mut self) -> &mut Split {
        self
    }
}

impl AsMut<Split> for Split {
    fn as_mut(&mut self) -> &mut Split {
        self
    }
}

impl<'a> Iterator for &'a mut Split {
    type Item = &'a WCStr;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < self.buffer.len() {
            let &delim = self.buffer.last().unwrap();
            let pos = self.buffer.iter().position(|&w| w == delim).unwrap();
            self.buffer[pos] = 0u16;
            let offset = pos + 1;
            let result = &self.buffer[self.offset .. offset];
            self.offset = offset;
            Some(unsafe { std::mem::transmute(result) })
        }
        else {
            None
        }
    }
}

