// Based off lemire's wyrand C++ code at https://github.com/lemire/testingRNG/blob/master/source/wyrand.h

use super::RNG;

/// An instance of the wyrand random number generator.  
/// Seeded from the system entropy generator when available.  
/// **This generator is _NOT_ cryptographically secure.**
pub struct WyRand {
	seed: u64,
}

#[cfg(feature = "std")]
impl WyRand {
	/// Create a new [`WyRand`] instance, seeding from the system's default source of entropy.
	pub fn new() -> Self {
		Self {
			seed: crate::entropy::entropy_from_system(),
		}
	}
}

#[cfg(feature = "std")]
impl Default for WyRand {
	/// Create a new [`WyRand`] instance, seeding from the system's default source of entropy.
	fn default() -> Self {
		Self {
			seed: crate::entropy::entropy_from_system(),
		}
	}
}

impl RNG for WyRand {
	type Output = [u8; 8];

	fn rand(&mut self) -> Self::Output {
		self.seed = self.seed.wrapping_add(0xa0761d6478bd642f);
		let t: u128 = (self.seed as u128).wrapping_mul((self.seed ^ 0xe7037ed1a0b428db) as u128);
		let ret = ((t >> 64) ^ t) as u64;
		ret.to_ne_bytes()
	}

	fn rand_with_seed(seed: &[u8]) -> Self::Output {
		let mut seed_bytes = [0u8; 8];
		seed_bytes.iter_mut().zip(seed).for_each(|(a, b)| *a = *b);
		let seed = u64::from_ne_bytes(seed_bytes);
		let seed = seed.wrapping_add(0xa0761d6478bd642f);
		let t: u128 = (seed as u128).wrapping_mul((seed ^ 0xe7037ed1a0b428db) as u128);
		let ret = ((t >> 64) ^ t) as u64;
		ret.to_ne_bytes()
	}

	fn reseed(&mut self, new_seed: &[u8]) {
		let mut seed = [0u8; 8];
		seed.iter_mut().zip(new_seed).for_each(|(a, b)| *a = *b);
		self.seed = u64::from_ne_bytes(seed)
	}
}

impl Clone for WyRand {
	fn clone(&self) -> Self {
		Self { seed: self.seed }
	}
}
