# Rupa
**Ru**st Color**pa**nes
Show off your terminal colors!  
![](https://modeus.is-inside.me/uoICFLYH.png)

# Usage 
A precompiled binary is provided [here](https://github.com/Luvella/Rupa/releases), or you can [build yourself](#build).

# Build
This requires [Rust](https://rust-lang.org) and Cargo.  
```sh
cargo build # --release (For a smaller binary)
strip target/release/rupa # Remove debug symbols, making the binary even smaller 
```  
`strip` isn't provided on Windows by default, you will need MinGW or TDM-GCC.