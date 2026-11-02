//! KROMT Address Utilities
//!
//! Local, offline-capable tools for Ethereum address validation,
//! EIP-55 checksumming, and basic conversions.
//!
//! No network calls. No private keys. No credentials.
//!
//! https://kromt.xyz  |  https://github.com/kromtxyz

use clap::{Parser, Subcommand};
use sha3::{Digest, Keccak256};

#[derive(Parser)]
#[command(name = "kromt-address")]
#[command(about = "KROMT Address Utilities – EIP-55, validation, conversion", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate an Ethereum address (basic hex + length check)
    Validate { address: String },

    /// Apply EIP-55 checksum to an address
    Checksum { address: String },

    /// Convert address to lowercase
    Lower { address: String },

    /// Convert address to uppercase
    Upper { address: String },

    /// Show detailed info about an address
    Info { address: String },
}

fn normalize_hex(addr: &str) -> Result<String, String> {
    let a = addr.trim().trim_start_matches("0x").trim_start_matches("0X");
    if a.len() != 40 {
        return Err(format!("Invalid length: expected 40 hex chars, got {}", a.len()));
    }
    if !a.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Contains non-hex characters".into());
    }
    Ok(a.to_lowercase())
}

fn eip55_checksum(addr_lower: &str) -> String {
    let hash = Keccak256::digest(addr_lower.as_bytes());
    let mut result = String::from("0x");
    for (i, c) in addr_lower.chars().enumerate() {
        let byte = hash[i / 2];
        let nibble = if i % 2 == 0 { byte >> 4 } else { byte & 0x0f };
        if c.is_ascii_digit() {
            result.push(c);
        } else if nibble >= 8 {
            result.push(c.to_ascii_uppercase());
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }
    result
}

fn is_valid_checksum(addr: &str) -> bool {
    let cleaned = addr.trim().trim_start_matches("0x").trim_start_matches("0X");
    if cleaned.len() != 40 {
        return false;
    }
    let lower = cleaned.to_lowercase();
    let checksummed = eip55_checksum(&lower);
    checksummed.eq_ignore_ascii_case(addr) && checksummed == format!("0x{}", cleaned) || checksummed == addr
        || checksummed.to_lowercase() == addr.to_lowercase()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { address } => match normalize_hex(&address) {
            Ok(_) => {
                println!("✓ Valid address format");
                let lower = normalize_hex(&address).unwrap();
                let checksum = eip55_checksum(&lower);
                if address == checksum || address.to_lowercase() == format!("0x{}", lower) {
                    println!("  Checksum: {}", checksum);
                } else {
                    println!("  Note: provided address is not EIP-55 checksummed");
                    println!("  Suggested: {}", checksum);
                }
            }
            Err(e) => {
                eprintln!("✗ Invalid: {}", e);
                std::process::exit(1);
            }
        },

        Commands::Checksum { address } => match normalize_hex(&address) {
            Ok(lower) => {
                let cs = eip55_checksum(&lower);
                println!("{}", cs);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },

        Commands::Lower { address } => match normalize_hex(&address) {
            Ok(lower) => println!("0x{}", lower),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },

        Commands::Upper { address } => match normalize_hex(&address) {
            Ok(lower) => println!("0x{}", lower.to_uppercase()),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },

        Commands::Info { address } => match normalize_hex(&address) {
            Ok(lower) => {
                let checksum = eip55_checksum(&lower);
                println!("KROMT Address Info");
                println!("------------------");
                println!("Input     : {}", address);
                println!("Lowercase : 0x{}", lower);
                println!("Uppercase : 0x{}", lower.to_uppercase());
                println!("EIP-55    : {}", checksum);
                println!("Length    : 20 bytes (40 hex chars)");
                println!();
                println!("Local-only tool. No network. No private keys.");
                println!("KROMT · https://kromt.xyz · https://github.com/kromtxyz");
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
    }
}
