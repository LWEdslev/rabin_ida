//! # Sharing

pub mod ida;

mod rabin_share;
mod gf;

#[doc(inline)]
pub use crate::ida::RabinIDA;


#[cfg(test)]
mod tests {
use test_strategy::proptest;
use proptest::collection::size_range;
use crate::ida::RabinIDA;

#[proptest]
fn test_up_to_5_mb(#[any(size_range(0..1024*5).lift())] data: Vec<u8>, #[strategy(2..255u8)] n: u8, #[strategy(1..#n)] k: u8) {
 let sharer = RabinIDA::new(n, k);
 let shares = sharer.share(data.clone());
let rec = sharer.reconstruct(shares[1..=k as usize].to_vec()).unwrap();
assert_eq!(data, rec);
}
}
