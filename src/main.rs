use anyhow::{anyhow, Context};
use byteorder::ByteOrder;
use std::env;
use std::io::{Read, Write};
use std::mem::MaybeUninit;

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

fn main() {
    for arg in env::args().skip(1) {
        match read_file(&arg) {
            Ok(plot) => {
                println!(
                    "{} {} {} {} {}",
                    &arg,
                    plot.format_description,
                    plot.k,
                    base64::encode(&plot.plot_id),
                    base64::encode(&plot.memo)
                );
            }
            Err(e) => {
                eprintln!("error processing {}: {}", &arg, e);
            }
        }
    }
}
