// Package in same worksapce
use lib01::add;

// reference to lib inside same crate of main but other directory
// need 'mod' and 'use' to get module, no matter the order
use opp::zero::{testa_nao_zero, testa_zero};
mod opp;

// Reference module in same directori of main.rs
mod aux;

fn main() {
    println!("Soma lib: {}", add(10, 20));

    println!("Soma: {}", aux::soma(1, 1));
    println!("Subtrai: {}", aux::subtrai(10, 1));
    println!("Multiplica: {}", aux::multiplica(5, 2));
    println!("Divide: {}", aux::divide(12, 2));

    println!("Zero: {}", testa_zero(10));
    println!("Zero: {}", testa_nao_zero(10));
}
