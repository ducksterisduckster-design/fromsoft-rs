// tools/binary-mapper/examples/dump_version.rs
//
// Prints the PE VERSIONINFO resource (product name + dwProductVersion) for a
// given executable. Use this to find the exact strings to match on in a
// GameVersion::from_metadata() implementation before hardcoding a new patch.
//
// Usage:
//   cargo run --bin binary-mapper --example dump_version -- "<path to exe>"

use pelite::pe64::{Pe, PeFile};
use std::{env, fs};

fn main() {
    let path = env::args().nth(1).expect("usage: dump_version <exe path>");
    let bytes = fs::read(&path).expect("could not read exe");
    let file = PeFile::from_bytes(&bytes).expect("not a valid PE file");

    let resources = file.resources().expect("no resources directory in this PE");
    let info = resources.version_info().expect("no VERSIONINFO resource");
    let fixed = info.fixed().expect("no fixed file info");
    let v = fixed.dwProductVersion;

    println!(
        "dwProductVersion: {}.{}.{}.{}",
        v.Major, v.Minor, v.Patch, v.Build
    );

    for language in info.translation() {
        info.strings(*language, |k, val| {
            if k == "ProductName" {
                println!("ProductName (lang_id {:#06x}): {:?}", language.lang_id, val);
            }
        });
    }
}
