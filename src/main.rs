use clap::{builder::styling::AnsiColor, Parser};
use std::fs::File;
use std::io::{BufReader, Read};

const STYLES: clap::builder::Styles = clap::builder::Styles::styled()
    .header(AnsiColor::Yellow.on_default().underline())
    .usage(AnsiColor::Yellow.on_default().underline())
    .literal(AnsiColor::Green.on_default().bold())
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Parser)]
#[clap(version)]
#[command(disable_version_flag = true, styles = STYLES)]
struct Cli {
    /// Print version
    #[arg(short = 'v', short_alias = 'V', long, action = clap::builder::ArgAction::Version)]
    version: (),

    /// Input number to search for
    number: u16,
}

fn convert(buf: &[u8]) -> anyhow::Result<Vec<u16>> {
    let mut json = Vec::with_capacity(buf.len() + 2);
    json.push(b'[');
    json.extend_from_slice(&buf);
    json.push(b']');

    let mut parsed: Vec<u16> = serde_json::from_slice(&json)?;
    parsed.sort();

    Ok(parsed)
}

fn search(numbers: &[u16], target: u16) -> Option<usize> {
    let mut low = 0;
    let mut high = numbers.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let value = numbers[mid];

        println!("🔄 midpoint: {}, value: {}", mid, value);

        match value.cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Greater => high = mid - 1,
            std::cmp::Ordering::Less => low = mid + 1,
        }
    }
    None
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let file = File::open("numberlist.txt")?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    let _ = reader.read_to_end(&mut buffer);

    let json = convert(&buffer)?;

    println!("🔎 searching for {}", cli.number);

    let result = search(&json, cli.number);

    match result {
        Some(number) => println!("☑️ found number at index: {}", number),
        None => println!("❌ unable to find number"),
    }

    Ok(())
}
