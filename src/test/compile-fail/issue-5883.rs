// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait A {}

struct Struct {
    r: A+'static
}

fn new_struct(r: A+'static) -> Struct {
    //~^ ERROR variable `r` has dynamically sized type
    Struct { r: r } //~ ERROR trying to initialise a dynamically sized struct
    //~^ ERROR E0161
    //~^^ ERROR E0161
}

trait Curve {}
enum E {X(Curve+'static)}
fn main() {}
