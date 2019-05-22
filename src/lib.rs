// Copyright 2018 Grove Enterprises LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Additional modifications to this file may have been made by Timely
// Data, Inc. See the version control log for precise modification
// information. The derived work is copyright 2019 Timely Data and
// is not licensed under the terms of the above license.

//! SQL Parser for Rust
//!
//! Example code:
//!
//! This crate provides an ANSI:SQL 2011 lexer and parser that can parsed SQL into an Abstract
//! Syntax Tree (AST).
//!
//! ```
//! use sqlparser::dialect::GenericSqlDialect;
//! use sqlparser::sqlparser::Parser;
//!
//! let dialect = GenericSqlDialect {}; // or AnsiSqlDialect
//!
//! let sql = "SELECT a, b, 123, myfunc(b) \
//!            FROM table_1 \
//!            WHERE a > b AND b < 100 \
//!            ORDER BY a DESC, b";
//!
//! let ast = Parser::parse_sql(&dialect, sql.to_string()).unwrap();
//!
//! println!("AST: {:?}", ast);
//! ```
#![warn(clippy::all)]

pub mod dialect;
pub mod sqlast;
pub mod sqlparser;
pub mod sqltokenizer;

#[doc(hidden)]
// This is required to make utilities accessible by both the crate-internal
// unit-tests and by the integration tests <https://stackoverflow.com/a/44541071/1026>
pub mod test_utils;
