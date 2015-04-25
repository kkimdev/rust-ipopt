/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#[link(name = "ipopt")]
extern crate ipopt;

fn main() {
    let problem = ipopt::Ipopt::new(
        1,
        &[-1.0],
        &[1.0],
        0,
        None,
        None,
        0,
        0,
        ipopt::MatrixLayout::RowMajor,
        None,
        None,
        None,
        None,
        None
        ).unwrap();
}
