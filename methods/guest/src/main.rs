#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
//#![no_std]  // std support is experimental

extern {
    fn run_uarch();
}

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
   println!("entering uarch");
   unsafe {
     run_uarch();
   }
   println!("leaving uarch");
}
