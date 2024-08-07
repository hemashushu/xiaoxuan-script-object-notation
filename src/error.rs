// Copyright (c) 2024 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License version 2.0 and additional exceptions,
// more details in file LICENSE, LICENSE.additional and CONTRIBUTING.

use std::fmt::{self, Display};

use crate::location::{Location, LocationWithRange};

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    Message(String),

    // note that the (index+length) of location may exceed the last index of string,
    // such as the "char incomplete" error raised by the string `'a`, which
    // index = 2.
    MessageWithLocation(String, Location),
    MessageWithLocationRange(String, LocationWithRange),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => f.write_str(msg),
            Error::MessageWithLocation(msg, loc) => {
                write!(
                    f,
                    "{}\nLine: {}, column: {}",
                    msg,
                    loc.line + 1,
                    loc.column + 1
                )
            }
            Error::MessageWithLocationRange(msg, loc) => {
                write!(
                    f,
                    "{}\nLine: {}, column: {}, length: {}",
                    msg,
                    loc.line + 1,
                    loc.column + 1,
                    loc.length
                )
            }
        }
    }
}

impl std::error::Error for Error {}

pub fn print_error_with_source(err: &Error, source: &str) -> String {
    // print pretty error with source text
    todo!()
}
