use std::marker::PhantomData;

struct True;
struct False;
struct Zero;
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

trait Merge<Other> {
    type Res;
    fn merge(self, other: Other) -> Self::Res;
}
impl<T, M: Nat> Merge<SizeProofVec<T, Zero>> for SizeProofVec<T, M> {
    type Res = SizeProofVec<T, M>;
    fn merge(self, _: SizeProofVec<T, Zero>) -> Self::Res {
        self
    }
}
impl<T, N: Nat, M: Nat> Merge<SizeProofVec<T, Suc<N>>> for SizeProofVec<T, M> {
    type Res = SizeProofVec<T, Suc<M>>;
    fn merge(self, other: SizeProofVec<T, Suc<N>>) -> Self::Res {
        let (new_other, x) = other.pop();
        self.push(x)
    }
}

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
    let is_a_empty = a.is_empty();

    let a = a.push(1);
    let b = b.push(1);
    let is_a_empty = a.is_empty();

    let a = a.pop().0;
    let is_a_empty = a.is_empty();

    let a = a.push(1);

    let a = a.merge(b);

    // test_eq(&a, &b);

    // a.merge(b);
}
