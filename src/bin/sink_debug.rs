use std::io;
use std::io::{Write, Sink, sink};
use std::fmt;
use std::fmt::Debug;

pub trait DebugWrite: Debug + Write {}

pub struct SinkDebug(Sink);

impl<SinkDebug: Debug + Write> DebugWrite for SinkDebug {}

pub fn sink_debug() -> SinkDebug {
    SinkDebug(sink())
}

impl Write for SinkDebug {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl Debug for SinkDebug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("Empty")
    }
}
