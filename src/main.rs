use anyhow::{Context, Result};
use pcap_file::pcap::PcapReader;
use std::fs::File;

static ASCII_PATTERN: &str = "B6034";

// pub fn find_ascii_pattern() {
//     for i in 0..=data.len().saturating_sub(rhs)
// }

fn main() -> Result<()> {
    let file_in = File::open("mdf-kospi200.20110216-0.pcap").context("pcap file not found")?;
    let mut pcap_reader = PcapReader::new(file_in)?;

    let mut counter = 0;

    // Read the pcap file
    // while

    while (counter < 5) {
        if let Some(pkt) = pcap_reader.next_packet() {
            let pkt = pkt.unwrap();
            println!("{:?}", pkt);
        }
        counter += 1;
    }
    Ok(())
}
