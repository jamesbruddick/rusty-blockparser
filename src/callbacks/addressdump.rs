use crate::callbacks::{Callback, Block};
use crate::common::Result;
use clap::{ArgMatches, Command};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::collections::HashSet;
use std::hash::BuildHasherDefault;
use fxhash::FxHasher;

pub struct AddressDump {
    file: BufWriter<File>,
    seen_hashes: HashSet<[u8; 20], BuildHasherDefault<FxHasher>>, 
}

impl AddressDump {
    pub fn new(_matches: &ArgMatches) -> Result<Self> {
        let file = File::create("addresses.txt")?;
        let hasher = BuildHasherDefault::<FxHasher>::default();
        let seen_hashes = HashSet::with_capacity_and_hasher(2_000_000_000, hasher);
        
        Ok(AddressDump {
            file: BufWriter::new(file),
            seen_hashes,
        })
    }
}

impl Callback for AddressDump {
    fn build_subcommand() -> Command {
        Command::new("addressdump").about("Address Dumper")
    }

    fn new(matches: &ArgMatches) -> Result<Self> {
        Self::new(matches)
    }

    fn on_start(&mut self, _block_height: u64) -> Result<()> {
        info!(target: "addressdump", "Starting Address Dump...");
        Ok(())
    }

    fn on_block(&mut self, block: &Block, height: u64) -> Result<()> {
        for tx in &block.txs {
            for output in &tx.value.outputs {
                if let Some(ref address_str) = output.script.address {
                    let mut raw_hash = [0u8; 20];
                    let bytes = address_str.as_bytes();
                    let len = std::cmp::min(bytes.len(), 20);
                    raw_hash[..len].copy_from_slice(&bytes[..len]);

                    if self.seen_hashes.insert(raw_hash) {
                        writeln!(self.file, "{}", address_str)?;
                    }
                }
            }
        }

        if height % 10000 == 0 {
            let usage = self.seen_hashes.len();
            if usage > 2_000_000_000 {
                error!("CRITICAL: RAM CAPACITY REACHED. SHUTTING DOWN TO PREVENT CRASH.");
                std::process::exit(1);
            }
        }
        Ok(())
    }

    fn on_complete(&mut self, _block_height: u64) -> Result<()> {
        self.file.flush()?;
        Ok(())
    }
}