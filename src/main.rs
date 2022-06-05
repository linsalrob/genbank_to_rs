
use clap::Parser;
use std::fs::File;
use std::io;

use gb_io::reader::SeqReader;

/// Parse a GenBank file and output in different formats.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // GenBank file to parse
    #[clap(short = 'g', long = "genbank")]
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,

}

fn main() {
    let file = File::open("mg1655.gb").unwrap();
    let stdout = io::stdout();
    for seq in SeqReader::new(file) {
        let seq = seq.unwrap();
        let rc = seq.revcomp();
        rc.write(stdout.lock()).unwrap();
    }
}

