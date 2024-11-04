#![no_std]
extern crate alloc;

use soroban_sdk::{contract, contractimpl, Env, IntoVal, TryFromVal, Vec};
use num_bigint::BigUint;
use num_traits::Zero; 
use alloc::vec::Vec as StdVec; 

// Constants for scalar and base field sizes using BigUint
fn get_constants() -> (BigUint, BigUint) {
    let r = BigUint::parse_bytes(b"21888242871839275222246405745257275088548364400416034343698204186575808495617", 10).unwrap();
    let q = BigUint::parse_bytes(b"21888242871839275222246405745257275088696311157297823662689037894645226208583", 10).unwrap();
    (r, q)
}