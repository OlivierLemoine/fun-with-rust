use std::marker::PhantomData;

#[derive(Debug)]
struct True;
#[derive(Debug)]
struct False;
#[derive(Debug)]
struct Zero;
#[derive(Debug)]
struct Suc<N: Nat> {
    _pred: PhantomData<N>,
}

trait Nat {}
impl Nat for Zero {}
impl<N: Nat> Nat for Suc<N> {}

trait IsEqualTo<N, Result> {}
impl IsEqualTo<Zero, True> for Zero {}
impl<N: Nat> IsEqualTo<Suc<N>, False> for Zero {}
impl<N: Nat> IsEqualTo<Zero, False> for Suc<N> {}

#[derive(Debug)]
struct SizeProofVec<T, N: Nat> {
    v: Vec<T>,
    _size: PhantomData<N>,
}
impl<T, N: Nat> SizeProofVec<T, N> {
    pub fn new() -> SizeProofVec<T, N> {
        SizeProofVec {
            v: vec![],
            _size: PhantomData,
        }
    }
    pub fn push(mut self, x: T) -> SizeProofVec<T, Suc<N>> {
        self.v.push(x);
        SizeProofVec {
            v: self.v,
            _size: PhantomData,
        }
    }
    pub fn suck_up_one<M: Nat>(
        mut self,
        other: SizeProofVec<T, Suc<M>>,
    ) -> (SizeProofVec<T, Suc<N>>, SizeProofVec<T, M>) {
        let (v, x) = other.pop();
        let res = self.push(x);
        (res, v)
    }
    pub fn merge<M: Nat>(mut self, other: SizeProofVec<T, Suc<M>>) -> SizeProofVec<T, impl Nat> {
        if other.is_empty_bool() {
            self
        } else {
            let (s, o) = self.suck_up_one(other);
            s
        }
    }
}
impl<T, N: Nat> SizeProofVec<T, Suc<N>> {
    pub fn pop(mut self) -> (SizeProofVec<T, N>, T) {
        let res = self.v.pop().unwrap();
        (
            SizeProofVec {
                v: self.v,
                _size: PhantomData,
            },
            res,
        )
    }
}
impl<T> SizeProofVec<T, Zero> {
    fn is_empty(&self) -> True {
        True
    }
    fn is_empty_bool(&self) -> bool {
        true
    }
}
impl<T, N: Nat> SizeProofVec<T, Suc<N>> {
    fn is_empty(&self) -> False {
        False
    }
    fn is_empty_bool(&self) -> bool {
        false
    }
}

fn test_eq<T, N: Nat, M: Nat>(_a: &SizeProofVec<T, N>, _b: &SizeProofVec<T, M>)
where
    N: IsEqualTo<M, True>,
{
}

fn main() {
    let a: SizeProofVec<u32, Zero> = SizeProofVec::new();
    let b: SizeProofVec<u32, Zero> = SizeProofVec::new();
    let _is_a_empty = a.is_empty();

    let a = a.push(1);
    let b = b.push(1);
    let _is_a_empty = a.is_empty();

    let a = a.pop().0;
    let _is_a_empty = a.is_empty();

    let a = a.push(1);

    let (a, b) = a.suck_up_one(b);

    let b = b.push(1);
    let b = b.push(1);
    let b = b.push(1);

    let a = a.merge(b);

    //println!("{:?} {:?}", a, b);

    // test_eq(&a, &b);

    // a.merge(b);
}
