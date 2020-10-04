use crate::RNG;

/// A trait used for generating a random object with an RNG,
pub trait RandomGen<R: RNG> {
	/// Return a random instance of the implementing type, from the specified RNG instance.
	fn random(r: &mut R) -> Self;
}

/// A trait used for generating a random number within a range, with an RNG,
pub trait RandomRange<R: RNG>: RandomGen<R> {
	/// Return a ranged number of the implementing type, from the specified RNG instance.
	fn random_range(r: &mut R, lower: Self, upper: Self) -> Self;
}

impl<R: RNG> RandomGen<R> for char {
	fn random(r: &mut R) -> Self {
		loop {
			let generated = r.rand();
			let mut bytes = [0u8; core::mem::size_of::<u32>()];
			bytes
				.iter_mut()
				.zip(generated.as_ref())
				.for_each(|(a, b)| *a = *b);
			if let Some(c) = core::char::from_u32(u32::from_ne_bytes(bytes)) {
				break c;
			}
		}
	}
}

/*
static inline uint64_t random_bounded_nearlydivisionless64(uint64_t range) {
  __uint128_t random64bit, multiresult;
  uint64_t leftover;
  uint64_t threshold;
  random64bit = RandomBitGenerator();
  multiresult = random64bit * range;
  leftover = (uint64_t)multiresult;
  if (leftover < range) {
	threshold = -range % range;
	while (leftover < threshold) {
	  random64bit = RandomBitGenerator();
	  multiresult = random64bit * range;
	  leftover = (uint64_t)multiresult;
	}
  }
  return multiresult >> 64; // [0, range)
}
 */

/// Boilerplate code for creating a RandomGen implementation for number types.  
/// Uses Lemire's debiased integer multiplication method.
macro_rules! randomgen_number {
    ($(($unsigned:ty, $signed:ty, $bigger_unsigned:ty, $bigger_signed:ty)),*) => {
        $(
            impl<R: RNG> RandomGen<R> for $unsigned {
                fn random(r: &mut R) -> Self {
                    let generated = r.rand();
                    let mut bytes = [0u8; core::mem::size_of::<$unsigned>()];
                    bytes.iter_mut().zip(generated.as_ref()).for_each(|(a, b)| *a = *b);
                    Self::from_ne_bytes(bytes)
                }
            }

            impl<R: RNG> RandomRange<R> for $unsigned {
                fn random_range(r: &mut R, lower: $unsigned, upper: $unsigned) -> Self {
                    const BIT_SIZE: usize = core::mem::size_of::<$unsigned>() * 8;
                    let mut random_bigger: $bigger_unsigned = r.generate();
                    let mut multiresult: $bigger_unsigned = random_bigger * (upper as $bigger_unsigned);
                    let threshold: $unsigned;
                    let mut leftover = multiresult as $unsigned;
                    if leftover < upper {
                        threshold = (-(upper as $signed) % (upper as $signed)) as $unsigned;
                        while leftover < threshold {
                            random_bigger = r.generate();
                            multiresult = random_bigger * (upper as $bigger_unsigned);
                            leftover = multiresult as $unsigned;
                        }
                    }
                    if BIT_SIZE == core::mem::size_of::<$bigger_unsigned>() {
                        (multiresult as $unsigned).max(lower)
                    } else {
                        ((multiresult >> BIT_SIZE) as $unsigned).max(lower)
                    }
                }
            }

            impl<R: RNG> RandomGen<R> for $signed {
                fn random(r: &mut R) -> Self {
                    let generated = r.rand();
                    let mut bytes = [0u8; core::mem::size_of::<$signed>()];
                    bytes.iter_mut().zip(generated.as_ref()).for_each(|(a, b)| *a = *b);
                    Self::from_ne_bytes(bytes)
                }
            }

            impl<R: RNG> RandomRange<R> for $signed {
                fn random_range(r: &mut R, lower: $signed, upper: $signed) -> Self {
                    const BIT_SIZE: usize = core::mem::size_of::<$signed>() * 8;
                    let mut random_bigger: $bigger_signed = r.generate();
                    let mut multiresult: $bigger_signed = random_bigger * (upper as $bigger_signed);
                    let threshold: $signed;
                    let mut leftover = multiresult as $signed;
                    if leftover < upper {
                        threshold = -(upper as $signed) % (upper as $signed);
                        while leftover < threshold {
                            random_bigger = r.generate();
                            multiresult = random_bigger * (upper as $bigger_signed);
                            leftover = multiresult as $signed;
                        }
                    }
                    if BIT_SIZE == core::mem::size_of::<$bigger_signed>() {
                        (multiresult as $signed).max(lower)
                    } else {
                        ((multiresult >> BIT_SIZE) as $signed).max(lower)
                    }
                }
            }
        )*
    }
}

randomgen_number!(
	(u8, i8, u16, i16),
	(u16, i16, u32, i32),
	(u32, i32, u64, i16),
	(u64, i64, u128, i128),
	(u128, i128, u128, i128),
	(usize, isize, u128, i128)
);
