/*
 * Copyright (c) 2023 David Irvine david.irvine@maidsafe.net
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * 
 * Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met (BSD 3-Clause License):
 * 
 * 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
 * 
 * 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
 * 
 * 3. Neither the name of David Irvine nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 * 
 * You may use this software under the terms of the MIT License or the BSD 3-Clause License. Choice of license is left to the discretion of the user of this software.
 */


pub mod ida;

mod gf;
mod rabin_share;

#[doc(inline)]
pub use crate::ida::RabinIDA;

#[cfg(test)]
mod tests {

    use crate::ida::RabinIDA;
    use proptest::collection::size_range;
    use rand::{prelude::SliceRandom, thread_rng};
    use test_strategy::proptest;

    #[proptest]
    fn test_up_to_1_mb(
        #[any(size_range(0..1024).lift())] data: Vec<u8>,
        #[strategy(2..255u8)] n: u8,
        #[strategy(1..#n)] k: u8,
    ) {
        let sharer = RabinIDA::new(n, k);
        let mut shares = sharer.share(data.clone());
        let mut rng = thread_rng();
        shares.shuffle(&mut rng); // test any k shares will recreate data
        let rec = sharer.reconstruct(shares[1..=k as usize].to_vec()).unwrap();
        assert_eq!(data, rec);
    }
}
