// https://drken1215.hatenablog.com/entry/2018/06/08/210000
// https://github.com/drken1215/algorithm/blob/master/LICENSE (CC 1.0)

use ac_library::modint::ModIntBase;

pub struct Combination<T: ModIntBase> {
    fac: Vec<T>,
    finv: Vec<T>,
}

impl<T: ModIntBase> Combination<T> {
    pub fn new(n: usize) -> Self {
        let mut fac = vec![T::new(1); n + 1];
        let mut finv = vec![T::new(1); n + 1];
        let mut inv = vec![T::new(1); n + 1];

        for i in 2..n {
            fac[i] = fac[i - 1] * i.into();
            inv[i] = -inv[(T::modulus() as usize % i) as usize] * (T::modulus() as usize / i).into();
            finv[i] = finv[i - 1] * inv[i];
        }

        Self { fac, finv }
    }

    pub fn calc(&self, n: usize, k: usize) -> T {
        if n < k {
            return T::new(0);
        }

        self.fac[n] * self.finv[k] * self.finv[n - k]
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use crate::math::combination::Combination;
    use ac_library::ModInt1000000007 as mint;

    #[test]
    fn tessoku_A30() {
        let comb = Combination::<mint>::new(10_usize.pow(6));

        assert_eq!(comb.calc(4, 2), mint::new(6));
        assert_eq!(comb.calc(77777, 44444), mint::new(409085577));
    }
}
