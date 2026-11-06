//! mkinitramfs — SigmaOS meta build runner entry point.
//!
//! Packs a staged sysroot directory into a bootable cpio `newc`
//! initramfs image — the final step of the OS image pipeline
//! (mkinitcpio/archiso analogue).
//!
//! Usage: mkinitramfs <sysroot-dir> <output-image>

use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: mkinitramfs <sysroot-dir> <output-image>");
        return ExitCode::from(2);
    }
    let root = Path::new(&args[1]);
    let out = Path::new(&args[2]);
    match sigmaos_initramfs::pack::pack_dir(root, out) {
        Ok(n) => {
            println!("wrote {} ({} entries)", out.display(), n);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("mkinitramfs: {e}");
            ExitCode::FAILURE
        }
    }
}
