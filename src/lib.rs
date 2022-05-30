//! # Sharing
//! Use for example the Shamir implementation

pub mod ids;

mod share;

#[doc(inline)]
pub use crate::{
    ids::RabinInformationDispersal,
};


#[cfg(test)]
mod tests {}
