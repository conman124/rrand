mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/* wasm_bindgen doesn't support exporting traits, so this seems 
like it might be the best way to do what I want */
trait RRandT {
	fn new(seed: f64) -> Self;

	fn next(&mut self) -> u32;

	fn next_range(&mut self, upper: u32) -> u32 {
		if upper < 2 {
			0
		} else {
			let min = upper.wrapping_neg() % upper;
			loop {
				let next = self.next();
				if next >= min {
					break next % upper;
				}
			}
		}
	}
}

/* Based on http://xoshiro.di.unimi.it/xoroshiro128starstar.c */
struct Xoroshiro128StarStar {
	a: u64,
	b: u64,
}

impl RRandT for Xoroshiro128StarStar {
	fn new(seed: f64) -> Xoroshiro128StarStar {
		// TODO do this better
		Xoroshiro128StarStar {
			a: seed.to_bits(),
			b: seed.to_bits()
		}
	}

	fn next(&mut self) -> u32 {
		let a = self.a;
		let mut b = self.b;

		let res = a.wrapping_mul(5).rotate_left(7).wrapping_mul(9);
		b = b ^ a;

		self.a = a.rotate_left(24) ^ b ^ (b << 16);
		self.b = b.rotate_left(37);

		(res >> 32) as u32
	}
}

#[wasm_bindgen]
pub struct RRand {
	rand_impl: Xoroshiro128StarStar
}

#[wasm_bindgen]
impl RRand {
	#[wasm_bindgen(constructor)]
	pub fn new(seed: f64) -> RRand {
		RRand {
			rand_impl: Xoroshiro128StarStar::new(seed)
		}
	}

	pub fn next(&mut self) -> u32 {
		self.rand_impl.next()
	}

	pub fn next_range(&mut self, upper: u32) -> u32 {
		self.rand_impl.next_range(upper)
	}
}