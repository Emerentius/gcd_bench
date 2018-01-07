#![allow(dead_code)]
#![feature(test)]
#![feature(slice_patterns)]
extern crate test;

extern crate rand;
extern crate time;

use rand::Rng;
use time::PreciseTime;

trait HasGCD {
    fn gcd(&self, other: &Self) -> Self;
    fn binary_gcd(&self, other: &Self) -> Self;
}

macro_rules! implement_has_gcd_for_uints {
    ( $t:ty ) => {
        impl HasGCD for $t {
            #[inline]
            fn gcd(&self, other: &Self) -> Self {
                let mut m = *self;
                let mut n = *other;

                // Use Euclid's algorithm
                while m != 0 {
                    let temp = m;
                    m = n % temp;
                    n = temp;
                }
                n
            }

            #[inline]
            fn binary_gcd(&self, other: &Self) -> Self {
                let mut m = *self;
                let mut n = *other;
                if m == 0 || n == 0 { return m | n }

                // find common factors of 2
                let shift = (m | n).trailing_zeros();

                // divide a and b by 2 until odd
                // m inside loop
                n >>= n.trailing_zeros();

                while m != 0 {
                    m >>= m.trailing_zeros();
                    if n > m { std::mem::swap(&mut n, &mut m) }
                    m -= n;
                }

                n << shift
            }
        }
    };
}

macro_rules! implement_has_gcd_for_ints {
    ( $t:ty, $min: expr) => {
        impl HasGCD for $t {
            #[inline]
            fn gcd(&self, other: &Self) -> Self {
                // Use Euclid's algorithm
                let mut m = *self;
                let mut n = *other;
                while m != 0 {
                    let temp = m;
                    m = n % temp;
                    n = temp;
                }
                n.abs()
            }

            #[inline]
            fn binary_gcd(&self, other: &Self) -> Self {
                let mut m = *self;
                let mut n = *other;
                if m == 0 || n == 0 { return (m | n).abs() }

                // find common factors of 2
                let shift = (m | n).trailing_zeros();

                // If one number is the minimum value, it cannot be represented as a
                // positive number. It's also a power of two, so the gcd can
                // trivially be calculated in that case by bitshifting

                // The result is always positive in two's complement, unless
                // a and b are the minimum value, then it's negative
                // no other way to represent that number
                if m == $min || n == $min { return 1 << shift }

                // guaranteed to be positive now, rest like unsigned algorithm
                m = m.abs();
                n = n.abs();

                // divide a and b by 2 until odd
                // m inside loop
                n >>= n.trailing_zeros();

                while m != 0 {
                    m >>= m.trailing_zeros();
                    if n > m { std::mem::swap(&mut n, &mut m) }
                    m -= n;
                }

                n << shift
            }
        }
    };
}

implement_has_gcd_for_uints!(u8);
implement_has_gcd_for_uints!(u16);
implement_has_gcd_for_uints!(u32);
implement_has_gcd_for_uints!(u64);
implement_has_gcd_for_uints!(usize);

implement_has_gcd_for_ints!(i8, i8::min_value());
implement_has_gcd_for_ints!(i16, i16::min_value());
implement_has_gcd_for_ints!(i32, i32::min_value());
implement_has_gcd_for_ints!(i64, i64::min_value());
implement_has_gcd_for_ints!(isize, isize::min_value());

macro_rules! define_bench {
    ( $name: ident, $t:ty, $print_message: expr) => {
        fn $name() {
            println!("\n{}", $print_message);

            let mut rng = rand::StdRng::new().unwrap();
            let total_repetitions = (N/2 * REPS) as f64;
            let random_nums: Vec<$t> = rng.gen_iter().take(N).collect();
            let total_time = |start: PreciseTime, end| start.to(end).num_nanoseconds().unwrap() as f64 / total_repetitions;

            // num crate gcd
            let start1 = PreciseTime::now();
            for nums in random_nums.chunks(2) {
                if let &[a,b] = nums {
                    for _ in 0..REPS {
                        test::black_box( gcd(a,b) );
                    }
                }
            }
            let end1 = PreciseTime::now();
            let time1 = total_time(start1,end1);
            println!("{:15}{:6.2} ns / call", "gcd: ", time1);

            // binary gcd
            let start2 = PreciseTime::now();
            for nums in random_nums.chunks(2) {
                if let &[a,b] = nums {
                    for _ in 0..REPS {
                        test::black_box( binary_gcd(a,b) );
                    }
                }
            }
            let end2 = PreciseTime::now();
            let time2 = total_time(start2,end2);
            let improvement = (time1/time2 - 1.) * 100.;

            println!("{:15}{:6.2} ns / call ( {:5.1}% faster )", "binary_gcd: ", time2, improvement);

            for nums in random_nums.chunks(2) {
                if let &[a,b] = nums {
                    let gcd_1 = gcd(a,b);
                    if gcd_1 != binary_gcd(a,b) {
                        panic!("Assertion failed for x,y: {}, {}, type {}", a,b,$print_message)
                    }
                    assert!( gcd_1 == binary_gcd(a,b) );
                }
            }
        }
    }
}

define_bench!(bench_u8, u8, "u8");
define_bench!(bench_u16, u16, "u16");
define_bench!(bench_u32, u32, "u32");
define_bench!(bench_u64, u64, "u64");

define_bench!(bench_i8, i8, "i8");
define_bench!(bench_i16, i16, "i16");
define_bench!(bench_i32, i32, "i32");
define_bench!(bench_i64, i64, "i64");

fn gcd<T: HasGCD>(a: T, b: T) -> T { a.gcd(&b) }
fn binary_gcd<T: HasGCD>(a: T, b: T) -> T { a.binary_gcd(&b) }

const N: usize = 100;
const REPS: usize = 10;

fn main() {
    //println!("{}", gcd(-64, -128i8));
    bench_u8();
    bench_u16();
    bench_u32();
    bench_u64();

    bench_i8();
    bench_i16();
    bench_i32();
    bench_i64();
}

#[test]
fn equality() {
    for num1 in -2000..2000 {
        for num2 in -2000..2000 {
            let gcd_1 = gcd(num1, num2);
            let gcd_2 = binary_gcd(num1, num2);
            if gcd_1 != gcd_2 { panic!("num1: {}, num2: {}, gcd: {}, binary_gcd: {}", num1, num2, gcd_1, gcd_2) }
            assert!( gcd_1 == binary_gcd(num1, num2) );
        }
    }
}

/* panics for another reason than when the code is inside main()
   reason: overflow when attempting negation
           nothing ever attempts to negate except the -128_i8 literal
#[test]
fn almost_every_combination_i8() {
    // except for max_value obviously
    for num1 in -128_i8..127 {
        for num2 in -128_i8..127 {
            let gcd_1 = gcd(num1, num2);
            let gcd_2 = binary_gcd(num1, num2);
            if gcd_1 != gcd_2 { panic!("num1: {}, num2: {}, gcd: {}, binary_gcd: {}", num1, num2, gcd_1, gcd_2) }
            assert!( gcd_1 == binary_gcd(num1, num2) );
        }
    }
}
*/

#[test]
fn almost_every_combination_u8() {
    // except for max_value obviously
    for num1 in 0_u8..255 {
        for num2 in 0_u8..255 {
            let gcd_1 = gcd(num1, num2);
            let gcd_2 = binary_gcd(num1, num2);
            if gcd_1 != gcd_2 { panic!("num1: {}, num2: {}, gcd: {}, binary_gcd: {}", num1, num2, gcd_1, gcd_2) }
            assert!( gcd_1 == binary_gcd(num1, num2) );
        }
    }
}

#[test]
fn border_cases() {
    assert!( binary_gcd(i8::min_value(), i8::min_value()) == i8::min_value() );
    assert!( binary_gcd(i8::min_value(), i8::max_value()) == 1 );
    assert!( binary_gcd(i8::max_value(), i8::min_value()) == 1 );
    assert!( binary_gcd(i8::max_value(), i8::max_value()) == i8::max_value() );
}
