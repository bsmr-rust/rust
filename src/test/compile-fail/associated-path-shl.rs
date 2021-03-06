// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that associated paths starting with `<<` are successfully parsed.

fn main() {
    let _: <<A>::B>::C; //~ ERROR unresolved type `A`
    let _ = <<A>::B>::C; //~ ERROR unresolved type `A`
    let <<A>::B>::C; //~ ERROR unresolved type `A`
    let 0 ... <<A>::B>::C; //~ ERROR unresolved type `A`
                           //~^ ERROR only char and numeric types are allowed in range patterns
    <<A>::B>::C; //~ ERROR unresolved type `A`
}
