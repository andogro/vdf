// Copyright 2018 Chia Network Inc and POA Networks Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#![forbid(unsafe_code)]
use self::ffi::Mpz;
use super::ffi;

/// Stores temporary values for congruence computations, to avoid
/// repeated allocations.
///
/// It is allowed (but inefficient) to generate a fresh `CongruenceContest`
/// for each call to `solve_linear_congruence`.
///
/// `self.solve_linear_congruence` can be called no matter what values
/// this struct’s public members hold, so long as they are valid `Mpz` values.
/// However, the values of these members after such a call must not be relied
/// on.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct CongruenceContext {
    pub g: Mpz,
    pub d: Mpz,
    pub q: Mpz,
    pub r: Mpz,
}

impl Default for CongruenceContext {
    fn default() -> Self {
        Self {
            g: Mpz::new(),
            d: Mpz::new(),
            q: Mpz::new(),
            r: Mpz::new(),
        }
    }
}

impl CongruenceContext {
    /// Solves `a*x = b (mod m)`, storing `x` in `mu`
    ///
    /// This function may clobber any or all of `self`’s member variables.
    ///
    /// # Panics
    ///
    /// Panics if the congruence could not be solved.
    pub fn solve_linear_congruence(
        &mut self,
        mu: &mut Mpz,
        v: Option<&mut Mpz>,
        a: &Mpz,
        b: &Mpz,
        m: &Mpz,
    ) {
        ffi::mpz_gcdext(&mut self.g, &mut self.d, mu, &a, &m);
        if cfg!(test) {
            println!(
                "g = {}, d = {}, e = {}, a = {}, m = {}",
                self.g, self.d, mu, a, m
            );
        }
        ffi::mpz_fdiv_qr(&mut self.q, &mut self.r, &b, &self.g);
        assert!(self.r.is_zero());
        ffi::mpz_mul(mu, &self.q, &self.d);
        *mu = mu.modulus(m);
        if let Some(v) = v {
            ffi::mpz_fdiv_q(v, &m, &self.g)
        }
    }
}
