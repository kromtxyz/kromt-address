# KROMT Address Utilities

**Offline Ethereum address tools for the KROMT ecosystem.**

Part of the public tooling suite for [KROMT](https://kromt.xyz).

- Website: https://kromt.xyz  
- GitHub: https://github.com/kromtxyz  

---

## What it does

Purely local utilities for working with Ethereum / EVM addresses:

- Format validation
- EIP-55 checksum generation
- Case conversion
- Detailed address inspection

**No network access.** Completely offline. No private keys involved.

## Requirements

- Rust 1.70+ (edition 2021)

```bash
cargo build --release
# binary at target/release/kromt-address
```

## Usage

```bash
# Validate
./kromt-address validate 0xAbC123...

# Generate EIP-55 checksum
./kromt-address checksum 0xabc123...

# Lowercase / Uppercase
./kromt-address lower 0xAbC...
./kromt-address upper 0xAbC...

# Full info
./kromt-address info 0xYourAddress
```

## Security Notes

- 100% offline after compilation.
- Does not handle private keys or signing.
- Safe to run in restricted environments.

## License

MIT — see [LICENSE](LICENSE)

---

**KROMT** · Non-custodial AI trading agent  
https://kromt.xyz · https://github.com/kromtxyz
