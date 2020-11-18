use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use rand::Rng;
use rand::rngs::{OsRng};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "generate", about = "generate a grid")]
struct Args {

    #[structopt(short = "s", long = "size", default_value = "1000", parse(try_from_str))]
    size: u32,

    #[structopt(parse(from_os_str))]
    output: PathBuf
}

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

fn main() -> std::io::Result<()>{
    let args = Args::from_args();
    let mut file_handle = BufWriter::new(File::create(&args.output)?);
    let mut rng = OsRng::default();
    for _y in 0..args.size {
        let row: String = (0..args.size).map(|_| {
            let i = rng.gen_range(0, CHARSET.len());
            CHARSET[i] as char
        }).collect();
        file_handle.write(row.as_bytes())?;
    }
    Ok(())
}