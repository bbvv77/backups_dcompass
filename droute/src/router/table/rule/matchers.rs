// Copyright 2020 LEXUGE
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

mod any;
mod domain;
mod qtype;

pub use self::{any::Any, domain::Domain, qtype::QType};

use std::fmt::Debug;
use thiserror::Error;
use trust_dns_proto::{op::query::Query, rr::resource::Record};

type Result<T> = std::result::Result<T, MatchError>;

#[derive(Error, Debug)]
/// All possible errors that may incur when using matchers.
pub enum MatchError {
    /// Error forwarded from `std::io::Error`.
    #[error("An I/O error encountered. Check files provided for matcher(s) to ensure they exist and have the right permissions.")]
    IOError(#[from] std::io::Error),

    /// Malformatted file provided to a matcher.
    #[error("File provided for matcher(s) is malformatted.")]
    Malformatted,
}

/// A matcher determines if something matches or not given the queries and responses.
pub trait Matcher: Sync + Send {
    /// Determine if match.
    fn matches(&self, queries: &[Query], resps: &[Record]) -> bool;
}
