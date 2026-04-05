use crate::callbacks::{Callback, Block};
use crate::common::Result;
use clap::{ArgMatches, Command};
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct BlockHashDump {
    file: BufWriter<File>,
}

impl BlockHashDump {
    pub fn new(_matches: &ArgMatches, end_height: Option<u64>) -> Result<Self> {
        let filename = match end_height {
            Some(end) => format!("{}.blockhashes.txt", end),
            None => "blockhashes.txt".to_string(),
        };

        let file = File::create(filename)?;

        Ok(BlockHashDump {
            file: BufWriter::new(file),
        })
    }
}

impl Callback for BlockHashDump {
    fn build_subcommand() -> Command
    where
        Self: Sized,
    {
        Command::new("blockhashdump").about("Block hash dumper")
    }

    fn new(matches: &ArgMatches) -> Result<Self>
    where
        Self: Sized,
    {
        BlockHashDump::new(matches, None)
    }

    fn on_start(&mut self, _block_height: u64) -> Result<()> {
        info!(target: "blockhashdump", "Starting Block Hash Dump...");
        Ok(())
    }

    fn on_block(&mut self, block: &Block, _block_height: u64) -> Result<()> {
        writeln!(self.file, "{}", block.header.hash)?;
        Ok(())
    }

    fn on_complete(&mut self, _block_height: u64) -> Result<()> {
        self.file.flush()?;
        Ok(())
    }
}
