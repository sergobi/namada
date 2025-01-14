//! VM types can be passed between the host and guest via wasm linear
//! memory.
//!
//! These are either:
//! 1. Module call types
//!    The module call inputs are passed host-to-guest.
//!
//! 2. Execution environment types
//!    The environment inputs are passed guest-to-host and outputs back from
//!    host-to-guest.

use std::collections::BTreeSet;

use crate::types::address::Address;
use crate::types::storage;

/// Input for validity predicate wasm module call
pub struct VpInput<'a> {
    /// The address of the validity predicate's owning account
    pub addr: &'a Address,
    /// The input data as arbitrary bytes
    pub data: &'a [u8],
    /// The storage changed keys from the write log of storage updates
    /// performed by the transaction for the account associated with the VP
    pub keys_changed: &'a BTreeSet<storage::Key>,
    /// The verifiers to trigger VPs
    pub verifiers: &'a BTreeSet<Address>,
}
