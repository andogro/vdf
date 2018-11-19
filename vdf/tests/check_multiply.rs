use std::env;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use std::fs::File;

extern crate vdf;
extern crate gmp;
extern crate num_traits;
use gmp::mpz::Mpz;
use vdf::ClassGroup;
use num_traits::Zero;


fn split_into_three_pieces(line: &str, c: char) -> [&str; 3] {
    let mut iter = line.split(c);
    let fst = iter.next().expect("bad test file");
    let snd = iter.next().expect("bad test file");
    let thd = iter.next().expect("bad test file");
    assert!(iter.next().is_none(), "bad test file");
    [fst, snd, thd]
}

#[test]
fn multiplication_is_correct() {
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").expect("cargo should have set this");
    let mut path = PathBuf::from(&manifest_path);
    path.push("tests/multiply.txt");
    let mut f = BufReader::new(File::open(path).expect("test malfunction"));
    let mut buffer = String::new();
    loop {
        let bytes_read = f.read_line(&mut buffer).expect("test malfunction");
        assert!(bytes_read == buffer.len());
        if bytes_read == 0 {
            break
        }
        if buffer.ends_with('\n') {
            buffer.pop();
        }
        if buffer.ends_with('\r') {
            buffer.pop();
        }
        let mut current_discriminant: Option<Mpz> = None;
        let q: Vec<_> = split_into_three_pieces(&buffer, '|').iter().map(|i| {
            let k = split_into_three_pieces(i, ',');

            let a = Mpz::from_str_radix(k[0], 10).expect("bad test file");
            let b = Mpz::from_str_radix(k[1], 10).expect("bad test file");
            let c = Mpz::from_str_radix(k[2], 10).expect("bad test file");
            let mut discriminant: Mpz = &b * &b;
            let mut minuand: Mpz = (4u64).into();
            minuand *= &a * &c;
            discriminant -= &minuand;
            assert!(discriminant < Mpz::zero());
            // takes waaaay too long
            //assert!(discriminant.probab_prime(20) != gmp::mpz::ProbabPrimeResult::NotPrime);
            if let Some(ref q) = current_discriminant {
                assert_eq!(q, &discriminant, "mismatching discriminant in test files");
            } else {
                current_discriminant = Some(discriminant.clone());
            }
            vdf::GmpClassGroup::from_ab_discriminant(a, b, discriminant)
        }).collect();
        assert_eq!(q.len(), 3);
        if &q[0] == &q[1] {
            let mut i = q[0].clone();
            i.square();
            assert_eq!(i, q[2]);
        }
        assert_eq!(&q[1] * &q[0], q[2], "multiplication not valid");
        assert_eq!(&q[0] * &q[1], q[2], "multiplication not valid");
        buffer.clear();
    }
}