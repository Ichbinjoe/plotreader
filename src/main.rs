use anyhow::{anyhow, Context};
use byteorder::ByteOrder;
use std::io::Read;
use structopt::StructOpt;

struct PlotInfo {
    plot_id: [u8; 32],
    k: u8,
    format_description: String,
    memo: Vec<u8>,
}

const HEADER: [u8; 19] = [
    0x50, 0x72, 0x6f, 0x6f, 0x66, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x70, 0x61, 0x63, 0x65, 0x20, 0x50,
    0x6c, 0x6f, 0x74,
];

fn read_file(name: &str) -> Result<PlotInfo, anyhow::Error> {
    let f = std::fs::File::open(name).context("couldn't open")?;
    let mut reader = std::io::BufReader::new(f);

    let mut buf = [0; 19];
    reader.read_exact(&mut buf).context("header read error")?;

    if buf != HEADER {
        return Err(anyhow!("header mismatch"));
    }

    let mut plot_id = [0; 32];
    reader
        .read_exact(&mut plot_id)
        .context("plotid read error")?;

    reader.read_exact(&mut buf[0..1]).context("k read error")?;
    let k = buf[0];

    reader
        .read_exact(&mut buf[0..2])
        .context("fdl read error")?;
    let fdl = byteorder::BigEndian::read_u16(&buf[0..2]);

    let mut fd = vec![0; fdl as usize];
    reader
        .read_exact(fd.as_mut_slice())
        .context("fd read error")?;

    let fds = String::from_utf8(fd).context("fd decode error")?;

    reader.read_exact(&mut buf[0..2]).context("ml read error")?;
    let ml = byteorder::BigEndian::read_u16(&buf[0..2]);

    let mut m = vec![0; ml as usize];
    reader
        .read_exact(m.as_mut_slice())
        .context("m read error")?;

    Ok(PlotInfo {
        plot_id,
        k,
        format_description: fds,
        memo: m,
    })
}

enum Pool<'a> {
    PublicKey(&'a [u8]),
    PoolContractHash(&'a [u8])
}

const PK_MEMO_LEN: usize = 48 + 48 + 32;
const PH_MEMO_LEN: usize = 32 + 48 + 32;

fn decode_memo<'a>(memo: &'a [u8]) -> Option<(Pool<'a>, &'a [u8], &'a [u8])> {
    match memo.len() {
        PK_MEMO_LEN => {
            Some((Pool::PublicKey(&memo[0..48]), &memo[48..96], &memo[96..128]))
        },
        PH_MEMO_LEN => {
            Some((Pool::PoolContractHash(&memo[0..32]), &memo[32..80], &memo[80..102]))
        },
        _ => None
    }
}

fn display_plot_big(name: &str, p: &PlotInfo) {
    println!("=============================");
    println!("Filename: {}:", name);
    println!("Format: {}", p.format_description);
    println!("K: {}", p.k);
    println!("Plot ID: {}", hex::encode(p.plot_id));
    println!("Memo:");
    match decode_memo(&p.memo) {
        Some((pool, farmer, local)) => {
            match pool {
                Pool::PublicKey(k) => {
                    println!("  Pool pk: {}", hex::encode(k));
                },
                Pool::PoolContractHash(ph) => {
                    println!("  Pool contract hash: {}", hex::encode(ph));
                }
            }
            println!("  Farmer pk: {}", hex::encode(farmer));
            println!("  Local sk: {}", hex::encode(local));
        },
        None => {
            println!("  Raw: {}", hex::encode(&p.memo));
        }
    }
    println!("=============================");
}

fn display_plot_row(name: &str, p: &PlotInfo) {
    println!("{} {} {} {} {}", name, p.format_description, p.k, hex::encode(p.plot_id), hex::encode(&p.memo));
}

#[derive(StructOpt)]
#[structopt(name="plotreader", about="Reads chia plot headers")]
struct Opts {
    #[structopt(short, long)]
    short: bool,

    files: Vec<String>
}

fn main() {
    let opt = Opts::from_args();

    for arg in opt.files {
        match read_file(&arg) {
            Ok(plot) => {
                if opt.short {
                    display_plot_row(&arg, &plot);
                } else {
                    display_plot_big(&arg, &plot);
                }
            }
            Err(e) => {
                eprintln!("error processing {}: {}", &arg, e);
            }
        }
    }
}
