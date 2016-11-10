/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate bindgen;

use std::env;
use std::path::Path;


fn main() {
    // TODO: Hard coded path is not good. :/.
    let binding = bindgen::builder()
            .header("/usr/include/coin/IpStdCInterface.h")
            .link("ipopt")
            .generate()
            .unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    binding.write_to_file(Path::new(&out_dir).join("IpStdCInterface.rs")).unwrap();
}
