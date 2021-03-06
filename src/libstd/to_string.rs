// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!

The `ToString` trait for converting to strings

*/

#![experimental]

use fmt;
use string::String;

/// A generic trait for converting a value to a string
pub trait ToString {
    /// Converts the value of `self` to an owned string
    fn to_string(&self) -> String;
}

/// Trait for converting a type to a string, consuming it in the process.
pub trait IntoStr {
    /// Consume and convert to a string.
    fn into_string(self) -> String;
}

impl<T: fmt::Show> ToString for T {
    fn to_string(&self) -> String {
        format!("{}", *self)
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;
    use super::*;

    #[test]
    fn test_simple_types() {
        assert_eq!(1i.to_string(), "1".to_string());
        assert_eq!((-1i).to_string(), "-1".to_string());
        assert_eq!(200u.to_string(), "200".to_string());
        assert_eq!(2u8.to_string(), "2".to_string());
        assert_eq!(true.to_string(), "true".to_string());
        assert_eq!(false.to_string(), "false".to_string());
        assert_eq!(().to_string(), "()".to_string());
        assert_eq!(("hi".to_string()).to_string(), "hi".to_string());
    }

    #[test]
    fn test_vectors() {
        let x: Vec<int> = vec![];
        assert_eq!(x.to_string(), "[]".to_string());
        assert_eq!((vec![1i]).to_string(), "[1]".to_string());
        assert_eq!((vec![1i, 2, 3]).to_string(), "[1, 2, 3]".to_string());
        assert!((vec![vec![], vec![1i], vec![1i, 1]]).to_string() ==
               "[[], [1], [1, 1]]".to_string());
    }
}
