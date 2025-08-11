use anyhow::{Context, Result};
use pcap_file::pcap::PcapReader;
use std::fs::File;

fn main() -> Result<()> {
    let file_in = File::open("mdf-kospi200.20110216-0.pcap").context("pcap file not found")?;
    let mut pcap_reader = PcapReader::new(file_in)?;

    // Read the pcap file
    while let Some(pkt) = pcap_reader.next_packet() {
        let pkt = pkt.unwrap();
        println!("{:?}", pkt);
    }

    Ok(())
}
