#![cfg(feature = "nightly")]
#![feature(specialization)]
#![recursion_limit="1024"]
extern crate generic_array;

use generic_array::typenum::{Unsigned, U9, U3};
use generic_array::*;
use std::ops::*;
use std::iter::*;
use generic_array::sequence;
use generic_array::functional::*;

/// Complex example using fully generic `GenericArray`s with the same length.
///
/// It's mostly just the repeated `Add` traits, which would be present in other systems anyway.
pub fn generic_array_zip_sum<A, B, N: Unsigned>(a: GenericArray<A, N>, b: GenericArray<B, N>) -> i32
where
    A: Into<i32>,
    B: Into<i32>
{
    a.zip(b, |l, r| l.into() + r.into()).map(|x| x + 1).fold(0, |a, x| x + a)
}

pub trait TypeConf {
    type V: From<u8>;
    type X: Unsigned;
    type N: Unsigned;
}
struct Foo {}

impl TypeConf for Foo{
    type V = u64;
    type N = U3;
    type X = U9;
}

pub fn generic_repeat<V: Clone, N:Unsigned>(x: V) -> GenericArray<V, N>{
    let a: GenericArray<V, N> = repeat(x).take(N::USIZE).collect();
    a
}

pub fn genericer_repeat<X: Unsigned, N:Unsigned, V: From<u64>>() -> GenericArray<V, N>{
    let a: GenericArray<V, N> = repeat(X::U64).take(N::USIZE).map(|x| V::from(x)).collect();
    a
}


pub fn genericest_repeat<T: TypeConf>() -> GenericArray<T::V, T::N>{
    let a: GenericArray<T::V, T::N> = repeat(T::X::U8).take(T::N::USIZE).map(|x| x.into()).collect();
    a
}

static nines: &[u64; 3] = &[9u64,9,9];
#[test]
fn foo(){

    let a: GenericArray<u64,U3> = generic_repeat(9);
    assert_eq!(a.as_slice(), nines);
    
    let a: GenericArray<_,_> = genericer_repeat::<U9, U3, u64>();
    assert_eq!(a.as_slice(), nines);
}

#[test]
fn test_genericest(){

    let a: GenericArray<_,_> = genericest_repeat::<Foo>();
    assert_eq!(a.as_slice(), nines);
}
