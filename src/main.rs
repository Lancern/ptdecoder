extern crate clap;
extern crate libipt;

use std::path::PathBuf;
use std::str::FromStr;

use clap::{App, Arg};

use libipt::packet::{Packet, PacketDecoder};
use libipt::ConfigBuilder as IptConfigBuilder;

macro_rules! report_error {
    ($($arg:tt)*) => ({
        eprintln!($($arg)*);
        std::process::exit(1);
    })
}

fn main() {
    let matches = App::new("ptdecoder")
        .version("0.1.0")
        .author("Sirui Mu <msrlancern@126.com>")
        .about("Command line utility to decode Intel PT packets from binary data stream")
        .arg(
            Arg::with_name("INPUT")
                .help("Path to the file holding encoded Intel PT packets data")
                .required(true),
        )
        .get_matches();

    let input_file_path = matches.value_of("INPUT").unwrap();
    let input_file_path = PathBuf::from_str(input_file_path).expect("invalid path");

    println!("Loading packet data");
    let mut file_data = match std::fs::read(&input_file_path) {
        Ok(data) => data,
        Err(err) => report_error!("Cannot read input file: {}", err),
    };
    let file_data_len = file_data.len();

    println!("Initializing packet decoder");

    let ipt_config_builder = match IptConfigBuilder::new(&mut file_data) {
        Ok(builder) => builder,
        Err(err) => report_error!("Cannot create Intel PT config: {}", err),
    };
    let ipt_config = ipt_config_builder.finish();

    let mut ipt_decoder = match PacketDecoder::new(&ipt_config) {
        Ok(decoder) => decoder,
        Err(err) => report_error!("Cannot create packet decoder: {}", err),
    };

    println!("Synchronizing packet decoder");
    match ipt_decoder.sync_forward() {
        Ok(()) => (),
        Err(err) => report_error!("Cannot synchronize packet decoder: {}", err),
    };

    println!("Start decoding packets");
    loop {
        let offset = match ipt_decoder.offset() {
            Ok(offset) => offset,
            Err(err) => report_error!("cannot tell offset: {}", err),
        };
        if offset as usize >= file_data_len {
            break;
        }

        let packet = match ipt_decoder.next() {
            Ok(packet) => packet,
            Err(err) => report_error!("cannot decode packet at offset {}: {}", offset, err),
        };

        print!("Offset {}: ", offset);
        let packet_msg = match packet {
            Packet::Invalid(_) => "Invalid",
            Packet::Psbend(_) => "Psbend",
            Packet::Stop(_) => "Stop",
            Packet::Pad(_) => "Pad",
            Packet::Psb(_) => "Psb",
            Packet::Ovf(_) => "Ovf",
            Packet::Unknown(_) => "Unknown",
            Packet::Fup(_) => "Fup",
            Packet::Tip(_) => "Tip",
            Packet::TipPge(_) => "TipPge",
            Packet::TipPgd(_) => "TipPgd",
            Packet::Tnt8(_) => "Tnt8",
            Packet::Tnt64(_) => "Tnt64",
            Packet::Mode(_) => "Mode",
            Packet::Pip(_) => "Pip",
            Packet::Vmcs(_) => "Vmcs",
            Packet::Cbr(_) => "Cbr",
            Packet::Tsc(_) => "Tsc",
            Packet::Tma(_) => "Tma",
            Packet::Mtc(_) => "Mtc",
            Packet::Cyc(_) => "Cyc",
            Packet::Mnt(_) => "Mnt",
            Packet::Exstop(_) => "Exstop",
            Packet::Mwait(_) => "Mwait",
            Packet::Pwre(_) => "Pwre",
            Packet::Pwrx(_) => "Pwrx",
            Packet::Ptw(_) => "Ptw",
        };
        println!("{}", packet_msg);
    }
}
