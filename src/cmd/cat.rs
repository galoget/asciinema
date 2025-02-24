use super::Command;
use crate::asciicast;
use crate::cli;
use crate::config::Config;
use anyhow::Result;
use std::io;
use std::io::Write;

impl Command for cli::Cat {
    fn run(self, _config: &Config) -> Result<()> {
        let mut encoder = asciicast::Encoder::new(0);
        let mut stdout = io::stdout();
        let mut time_offset: u64 = 0;
        let mut first = true;

        for path in self.filename.iter() {
            let recording = asciicast::open_from_path(path)?;
            let mut time = time_offset;

            if first {
                stdout.write_all(&encoder.header(&recording.header))?;
                first = false;
            }

            for event in recording.events {
                let mut event = event?;
                time = time_offset + event.time;
                event.time = time;
                stdout.write_all(&encoder.event(&event))?;
            }

            time_offset = time;
        }

        Ok(())
    }
}
