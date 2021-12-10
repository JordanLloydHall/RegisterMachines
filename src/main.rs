#![allow(dead_code)]
use std::collections::HashMap;

use num_bigint::BigUint;
use num_traits::{Pow, ToPrimitive, Zero};

pub type Label = usize;
pub type Register = u64;
pub type State = (Label, HashMap<Register, BigUint>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
    Add(Register, Label),
    Sub(Register, Label, Label),
    Halt,
}

use Instruction::*;
pub fn eval_program(program: &[Instruction], state: &State) -> State {
    let mut new_state = state.clone();

    while new_state.0 < program.len() {
        let curr_instr = program[new_state.0];
        match curr_instr {
            Add(r, l) => {
                *(new_state.1.entry(r).or_insert_with(BigUint::zero)) += 1u32;
                new_state.0 = l;
            }
            Sub(r, l1, l2) => {
                if new_state.1.entry(r).or_insert_with(BigUint::zero).is_zero() {
                    new_state.0 = l2;
                } else {
                    *(new_state.1.entry(r).or_insert_with(BigUint::zero)) -= 1u32;
                    new_state.0 = l1;
                }
            }
            Halt => return new_state,
        }
    }

    new_state
}

fn encode_pair1(x: &BigUint, y: &BigUint) -> BigUint {
    (BigUint::from(2u32)).pow(x) * (2u32 * y + 1u32)
}

fn encode_pair2(x: &BigUint, y: &BigUint) -> BigUint {
    encode_pair1(x, y) - 1u32
}

fn encode_program_to_list(program: &[Instruction]) -> Vec<BigUint> {
    program
        .iter()
        .map(|p| match p {
            Add(r, l) => encode_pair1(&BigUint::from(2 * *r), &BigUint::from(*l)),
            Sub(r, l1, l2) => encode_pair1(
                &BigUint::from(2 * *r + 1),
                &encode_pair2(&BigUint::from(*l1), &BigUint::from(*l2)),
            ),
            Halt => BigUint::zero(),
        })
        .collect()
}

fn decode_pair1(a: &BigUint) -> (BigUint, BigUint) {
    let mut tmp = a.clone();
    let mut x = BigUint::zero();

    while (&tmp % 2u32).is_zero() {
        x += 1u32;
        tmp /= 2u32;
    }

    let y = (tmp - 1u32) / 2u32;

    (x, y)
}

fn decode_pair2(a: &BigUint) -> (BigUint, BigUint) {
    decode_pair1(&(a + 1u32))
}

pub fn decode_list_to_program(program: &[BigUint]) -> Vec<Instruction> {
    program
        .iter()
        .map(|p| {
            if p.is_zero() {
                Halt
            } else {
                let (y, z) = decode_pair1(p);
                if (&y % 2u32).is_zero() {
                    Add(
                        (y / 2u32)
                            .to_u64()
                            .expect("Number is too big to be converted into u64"),
                        z.to_usize()
                            .expect("Number is too big to be converted into usize"),
                    )
                } else {
                    let (j, k) = decode_pair2(&z);
                    Sub(
                        ((y - 1u32) / 2u32)
                            .to_u64()
                            .expect("Number is too big to be converted into u64"),
                        j.to_usize()
                            .expect("Number is too big to be converted into usize"),
                        k.to_usize()
                            .expect("Number is too big to be converted into usize"),
                    )
                }
            }
        })
        .collect()
}

pub fn decode_godel_to_list(a: BigUint) -> Vec<BigUint> {
    let mut ret_vec = Vec::new();
    let mut tmp = a;

    while !tmp.is_zero() {
        let mut x = BigUint::zero();
        while (&tmp % 2u32).is_zero() {
            x += 1u32;
            tmp /= 2u32;
        }
        ret_vec.push(x);

        tmp = (tmp - 1u32) / 2u32;
    }

    ret_vec
}

pub fn encode_list_to_godel(l: &[BigUint]) -> BigUint {
    l.iter()
        .rev()
        .fold(BigUint::zero(), |acc, v| encode_pair1(v, &acc))
}

fn main() {
    // let n = u64::pow(2, 46) * 20483;
    // let godel_list = decode_godel_to_list(n);
    // println!("{:?} as a list is {:?} as a program is {:?}", n, &godel_list, decode_list_to_program(&godel_list))
}

#[test]
fn godel_num_to_godel_list() {
    let n = BigUint::from(2u32).pow(46u32) * 20483u32;
    let godel_list = decode_godel_to_list(n);
    let true_godel_list = vec![
        BigUint::from(46u32),
        BigUint::zero(),
        BigUint::from(10u32),
        BigUint::from(1u32),
    ];
    assert_eq!(godel_list, true_godel_list)
}

#[test]
fn godel_list_to_godel_num() {
    let godel_num = encode_list_to_godel(&[
        BigUint::from(46u32),
        BigUint::zero(),
        BigUint::from(10u32),
        BigUint::from(1u32),
    ]);
    assert_eq!(godel_num, BigUint::from(2u32).pow(46u32) * 20483u32)
}

#[test]
fn godel_list_to_program() {
    let program = decode_list_to_program(&[
        BigUint::from(46u32),
        BigUint::zero(),
        BigUint::from(10u32),
        BigUint::from(1u32),
    ]);
    assert_eq!(program, vec![Sub(0, 2, 1), Halt, Sub(0, 0, 1), Add(0, 0)])
}

#[test]
fn program_to_godel_list() {
    let program = encode_program_to_list(&[Sub(0, 2, 1), Halt, Sub(0, 0, 1), Add(0, 0)]);
    assert_eq!(
        program,
        vec![
            BigUint::from(46u32),
            BigUint::zero(),
            BigUint::from(10u32),
            BigUint::from(1u32)
        ]
    )
}

#[test]
fn program_produces_correct_state() {
    use std::array::IntoIter;
    let program = vec![
        Sub(1, 2, 1),
        Halt,
        Sub(1, 3, 4),
        Sub(1, 5, 4),
        Halt,
        Add(0, 0),
    ];
    let final_state = eval_program(
        &program,
        &(
            0,
            HashMap::<_, _>::from_iter(IntoIter::new([
                (0, BigUint::zero()),
                (1, BigUint::from(7u32)),
            ])),
        ),
    );
    assert_eq!(
        final_state,
        (
            4,
            HashMap::<_, _>::from_iter(IntoIter::new([
                (0, BigUint::from(2u32)),
                (1, BigUint::zero())
            ]))
        )
    )
}
