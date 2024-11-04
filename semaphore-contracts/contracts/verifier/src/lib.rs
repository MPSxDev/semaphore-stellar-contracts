#![no_std]
extern crate alloc;

use soroban_sdk::{contract, contractimpl, Env, IntoVal, TryFromVal, Vec};
use num_bigint::BigUint;
use num_traits::Zero; 
use alloc::vec::Vec as StdVec; 