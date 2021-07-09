use std::ops;

use crate::elliptic::curves::traits::*;

use super::*;

macro_rules! matrix {
    (
        trait = $trait:ident,
        trait_fn = $trait_fn:ident,
        output = $output:ty,
        output_new = $output_new:expr,
        point_fn = $point_fn:ident,
        point_assign_fn = $point_assign_fn:ident,
        pairs = {(r_<$($l:lifetime),*> $lhs_ref:ty, $rhs:ty), $($rest:tt)*}
    ) => {
        impl<$($l,)* E: Curve> ops::$trait<$rhs> for $lhs_ref {
            type Output = $output;
            fn $trait_fn(self, rhs: $rhs) -> Self::Output {
                let p = self.as_raw().$point_fn(rhs.as_raw());
                $output_new(p)
            }
        }
        matrix!{
            trait = $trait,
            trait_fn = $trait_fn,
            output = $output,
            output_new = $output_new,
            point_fn = $point_fn,
            point_assign_fn = $point_assign_fn,
            pairs = {$($rest)*}
        }
    };

    (
        trait = $trait:ident,
        trait_fn = $trait_fn:ident,
        output = $output:ty,
        output_new = $output_new:expr,
        point_fn = $point_fn:ident,
        point_assign_fn = $point_assign_fn:ident,
        pairs = {(_r<$($l:lifetime),*> $lhs:ty, $rhs_ref:ty), $($rest:tt)*}
    ) => {
        impl<$($l,)* E: Curve> ops::$trait<$rhs_ref> for $lhs {
            type Output = $output;
            fn $trait_fn(self, rhs: $rhs_ref) -> Self::Output {
                let p = rhs.as_raw().$point_fn(self.as_raw());
                $output_new(p)
            }
        }
        matrix!{
            trait = $trait,
            trait_fn = $trait_fn,
            output = $output,
            output_new = $output_new,
            point_fn = $point_fn,
            point_assign_fn = $point_assign_fn,
            pairs = {$($rest)*}
        }
    };

    (
        trait = $trait:ident,
        trait_fn = $trait_fn:ident,
        output = $output:ty,
        output_new = $output_new:expr,
        point_fn = $point_fn:ident,
        point_assign_fn = $point_assign_fn:ident,
        pairs = {(o_<$($l:lifetime),*> $lhs_owned:ty, $rhs:ty), $($rest:tt)*}
    ) => {
        impl<$($l,)* E: Curve> ops::$trait<$rhs> for $lhs_owned {
            type Output = $output;
            fn $trait_fn(self, rhs: $rhs) -> Self::Output {
                let mut raw = self.into_raw();
                raw.$point_assign_fn(rhs.as_raw());
                $output_new(raw)
            }
        }
        matrix!{
            trait = $trait,
            trait_fn = $trait_fn,
            output = $output,
            output_new = $output_new,
            point_fn = $point_fn,
            point_assign_fn = $point_assign_fn,
            pairs = {$($rest)*}
        }
    };

    (
        trait = $trait:ident,
        trait_fn = $trait_fn:ident,
        output = $output:ty,
        output_new = $output_new:expr,
        point_fn = $point_fn:ident,
        point_assign_fn = $point_assign_fn:ident,
        pairs = {(_o<$($l:lifetime),*> $lhs:ty, $rhs_owned:ty), $($rest:tt)*}
    ) => {
        impl<$($l,)* E: Curve> ops::$trait<$rhs_owned> for $lhs {
            type Output = $output;
            fn $trait_fn(self, rhs: $rhs_owned) -> Self::Output {
                let mut raw = rhs.into_raw();
                raw.$point_assign_fn(self.as_raw());
                $output_new(raw)
            }
        }
        matrix!{
            trait = $trait,
            trait_fn = $trait_fn,
            output = $output,
            output_new = $output_new,
            point_fn = $point_fn,
            point_assign_fn = $point_assign_fn,
            pairs = {$($rest)*}
        }
    };

    (
        trait = $trait:ident,
        trait_fn = $trait_fn:ident,
        output = $output:ty,
        output_new = $output_new:expr,
        point_fn = $point_fn:ident,
        point_assign_fn = $point_assign_fn:ident,
        pairs = {}
    ) => {
        // happy termination
    };
}

#[cfg(not(release))]
fn addition_of_two_points<E: Curve>(result: E::Point) -> PointZ<E> {
    // In non-release environment we check that every addition results into correct point (either
    // zero or of the expected order)
    PointZ::from_raw(result)
        .expect("addition of two points must be either a zero or of the same order")
}
#[cfg(release)]
fn addition_of_two_points<E: Curve>(result: E::Point) -> PointZ<E> {
    // In release we skip checks
    PointZ::from_raw_unchecked(result)
}

matrix! {
    trait = Add,
    trait_fn = add,
    output = PointZ<E>,
    output_new = addition_of_two_points,
    point_fn = add_point,
    point_assign_fn = add_point_assign,
    pairs = {
        (o_<> Point<E>, Point<E>), (o_<> Point<E>, PointZ<E>),
        (o_<> Point<E>, &Point<E>), (o_<> Point<E>, &PointZ<E>),
        (o_<'p> Point<E>, PointRef<'p, E>), (o_<> Point<E>, Generator<E>),

        (o_<> PointZ<E>, Point<E>), (o_<> PointZ<E>, PointZ<E>),
        (o_<> PointZ<E>, &Point<E>), (o_<> PointZ<E>, &PointZ<E>),
        (o_<'p> PointZ<E>, PointRef<'p, E>), (o_<> PointZ<E>, Generator<E>),

        (_o<> &Point<E>, Point<E>), (_o<> &Point<E>, PointZ<E>),
        (r_<> &Point<E>, &Point<E>), (r_<> &Point<E>, &PointZ<E>),
        (r_<'p> &Point<E>, PointRef<'p, E>), (r_<> &Point<E>, Generator<E>),

        (_o<> &PointZ<E>, Point<E>), (_o<> &PointZ<E>, PointZ<E>),
        (r_<> &PointZ<E>, &Point<E>), (r_<> &PointZ<E>, &PointZ<E>),
        (r_<'p> &PointZ<E>, PointRef<'p, E>), (r_<> &PointZ<E>, Generator<E>),

        (_o<'p> PointRef<'p, E>, Point<E>), (_o<'p> PointRef<'p, E>, PointZ<E>),
        (r_<'p> PointRef<'p, E>, &Point<E>), (r_<'p> PointRef<'p, E>, &PointZ<E>),
        (r_<'a, 'b> PointRef<'a, E>, PointRef<'b, E>), (r_<'p> PointRef<'p, E>, Generator<E>),

        (_o<> Generator<E>, Point<E>), (_o<> Generator<E>, PointZ<E>),
        (r_<> Generator<E>, &Point<E>), (r_<> Generator<E>, &PointZ<E>),
        (r_<'p> Generator<E>, PointRef<'p, E>), (r_<> Generator<E>, Generator<E>),
    }
}

#[cfg(not(release))]
fn subtraction_of_two_point<E: Curve>(result: E::Point) -> PointZ<E> {
    // In non-release environment we check that every subtraction results into correct point (either
    // zero or of the expected order)
    PointZ::from_raw(result)
        .expect("subtraction of two points must be either a zero or of the same order")
}
#[cfg(release)]
fn subtraction_of_two_point<E: Curve>(result: E::Point) -> PointZ<E> {
    // In release we skip checks
    PointZ::from_raw_unchecked(result)
}

matrix! {
    trait = Sub,
    trait_fn = sub,
    output = PointZ<E>,
    output_new = subtraction_of_two_point,
    point_fn = sub_point,
    point_assign_fn = sub_point_assign,
    pairs = {
        (o_<> Point<E>, Point<E>), (o_<> Point<E>, PointZ<E>),
        (o_<> Point<E>, &Point<E>), (o_<> Point<E>, &PointZ<E>),
        (o_<'p> Point<E>, PointRef<'p, E>), (o_<> Point<E>, Generator<E>),

        (o_<> PointZ<E>, Point<E>), (o_<> PointZ<E>, PointZ<E>),
        (o_<> PointZ<E>, &Point<E>), (o_<> PointZ<E>, &PointZ<E>),
        (o_<'p> PointZ<E>, PointRef<'p, E>), (o_<> PointZ<E>, Generator<E>),

        (_o<> &Point<E>, Point<E>), (_o<> &Point<E>, PointZ<E>),
        (r_<> &Point<E>, &Point<E>), (r_<> &Point<E>, &PointZ<E>),
        (r_<'p> &Point<E>, PointRef<'p, E>), (r_<> &Point<E>, Generator<E>),

        (_o<> &PointZ<E>, Point<E>), (_o<> &PointZ<E>, PointZ<E>),
        (r_<> &PointZ<E>, &Point<E>), (r_<> &PointZ<E>, &PointZ<E>),
        (r_<'p> &PointZ<E>, PointRef<'p, E>), (r_<> &PointZ<E>, Generator<E>),

        (_o<'p> PointRef<'p, E>, Point<E>), (_o<'p> PointRef<'p, E>, PointZ<E>),
        (r_<'p> PointRef<'p, E>, &Point<E>), (r_<'p> PointRef<'p, E>, &PointZ<E>),
        (r_<'a, 'b> PointRef<'a, E>, PointRef<'b, E>), (r_<'p> PointRef<'p, E>, Generator<E>),

        (_o<> Generator<E>, Point<E>), (_o<> Generator<E>, PointZ<E>),
        (r_<> Generator<E>, &Point<E>), (r_<> Generator<E>, &PointZ<E>),
        (r_<'p> Generator<E>, PointRef<'p, E>), (r_<> Generator<E>, Generator<E>),
    }
}

#[cfg(not(release))]
fn multiplication_of_nonzero_point_at_nonzero_scalar<E: Curve>(result: E::Point) -> Point<E> {
    Point::from_raw(result)
        .expect("multiplication of point at non-zero scalar must always produce a non-zero point of the same order")
}
#[cfg(release)]
fn multiplication_of_point_at_nonzero_scalar<E: Curve>(result: E::Point) -> Point<E> {
    Point::from_raw_unchecked(result)
}

matrix! {
    trait = Mul,
    trait_fn = mul,
    output = Point<E>,
    output_new = multiplication_of_nonzero_point_at_nonzero_scalar,
    point_fn = scalar_mul,
    point_assign_fn = scalar_mul_assign,
    pairs = {
        (_o<> Scalar<E>, Point<E>),
        (_r<> Scalar<E>, &Point<E>),
        (_r<'p> Scalar<E>, PointRef<'p, E>),

        (_o<> &Scalar<E>, Point<E>),
        (_r<> &Scalar<E>, &Point<E>),
        (_r<'p> &Scalar<E>, PointRef<'p, E>),

        // --- and vice-versa ---

        (o_<> Point<E>, Scalar<E>),
        (o_<> Point<E>, &Scalar<E>),

        (r_<> &Point<E>, Scalar<E>),
        (r_<> &Point<E>, &Scalar<E>),

        (r_<'p> PointRef<'p, E>, Scalar<E>),
        (r_<'p> PointRef<'p, E>, &Scalar<E>),
    }
}

#[cfg(not(release))]
fn multiplication_of_point_at_scalar<E: Curve>(result: E::Point) -> PointZ<E> {
    PointZ::from_raw(result)
        .expect("multiplication of point at scalar must always produce either a point of the same order or a zero point")
}
#[cfg(release)]
fn multiplication_of_point_at_scalar<E: Curve>(result: E::Point) -> PointZ<E> {
    PointZ::from_raw_unchecked(result)
}

matrix! {
    trait = Mul,
    trait_fn = mul,
    output = PointZ<E>,
    output_new = multiplication_of_point_at_scalar,
    point_fn = scalar_mul,
    point_assign_fn = scalar_mul_assign,
    pairs = {
        (_o<> Scalar<E>, PointZ<E>),
        (_r<> Scalar<E>, &PointZ<E>),

        (_o<> ScalarZ<E>, Point<E>), (_o<> ScalarZ<E>, PointZ<E>),
        (_r<> ScalarZ<E>, &Point<E>), (_r<> ScalarZ<E>, &PointZ<E>),
        (_r<'p> ScalarZ<E>, PointRef<'p, E>),

        (_o<> &Scalar<E>, PointZ<E>),
        (_r<> &Scalar<E>, &PointZ<E>),

        (_o<> &ScalarZ<E>, Point<E>), (_o<> &ScalarZ<E>, PointZ<E>),
        (_r<> &ScalarZ<E>, &Point<E>), (_r<> &ScalarZ<E>, &PointZ<E>),
        (_r<'p> &ScalarZ<E>, PointRef<'p, E>),

        // --- and vice-versa ---

        (o_<> Point<E>, ScalarZ<E>),
        (o_<> Point<E>, &ScalarZ<E>),

        (o_<> PointZ<E>, Scalar<E>), (o_<> PointZ<E>, ScalarZ<E>),
        (o_<> PointZ<E>, &Scalar<E>), (o_<> PointZ<E>, &ScalarZ<E>),

        (r_<> &Point<E>, ScalarZ<E>),
        (r_<> &Point<E>, &ScalarZ<E>),

        (r_<> &PointZ<E>, Scalar<E>), (r_<> &PointZ<E>, ScalarZ<E>),
        (r_<> &PointZ<E>, &Scalar<E>), (r_<> &PointZ<E>, &ScalarZ<E>),

        (r_<'p> PointRef<'p, E>, ScalarZ<E>),
        (r_<'p> PointRef<'p, E>, &ScalarZ<E>),
    }
}

matrix! {
    trait = Add,
    trait_fn = add,
    output = ScalarZ<E>,
    output_new = ScalarZ::from_raw,
    point_fn = add,
    point_assign_fn = add_assign,
    pairs = {
        (o_<> Scalar<E>, Scalar<E>), (o_<> Scalar<E>, ScalarZ<E>),
        (o_<> Scalar<E>, &Scalar<E>), (o_<> Scalar<E>, &ScalarZ<E>),
        (o_<> ScalarZ<E>, Scalar<E>), (o_<> ScalarZ<E>, ScalarZ<E>),
        (o_<> ScalarZ<E>, &Scalar<E>), (o_<> ScalarZ<E>, &ScalarZ<E>),
        (_o<> &Scalar<E>, Scalar<E>), (_o<> &Scalar<E>, ScalarZ<E>),
        (r_<> &Scalar<E>, &Scalar<E>), (r_<> &Scalar<E>, &ScalarZ<E>),
        (_o<> &ScalarZ<E>, Scalar<E>), (_o<> &ScalarZ<E>, ScalarZ<E>),
        (r_<> &ScalarZ<E>, &Scalar<E>), (r_<> &ScalarZ<E>, &ScalarZ<E>),
    }
}

matrix! {
    trait = Sub,
    trait_fn = sub,
    output = ScalarZ<E>,
    output_new = ScalarZ::from_raw,
    point_fn = sub,
    point_assign_fn = sub_assign,
    pairs = {
        (o_<> Scalar<E>, Scalar<E>), (o_<> Scalar<E>, ScalarZ<E>),
        (o_<> Scalar<E>, &Scalar<E>), (o_<> Scalar<E>, &ScalarZ<E>),
        (o_<> ScalarZ<E>, Scalar<E>), (o_<> ScalarZ<E>, ScalarZ<E>),
        (o_<> ScalarZ<E>, &Scalar<E>), (o_<> ScalarZ<E>, &ScalarZ<E>),
        (_o<> &Scalar<E>, Scalar<E>), (_o<> &Scalar<E>, ScalarZ<E>),
        (r_<> &Scalar<E>, &Scalar<E>), (r_<> &Scalar<E>, &ScalarZ<E>),
        (_o<> &ScalarZ<E>, Scalar<E>), (_o<> &ScalarZ<E>, ScalarZ<E>),
        (r_<> &ScalarZ<E>, &Scalar<E>), (r_<> &ScalarZ<E>, &ScalarZ<E>),
    }
}

matrix! {
    trait = Mul,
    trait_fn = mul,
    output = ScalarZ<E>,
    output_new = ScalarZ::from_raw,
    point_fn = mul,
    point_assign_fn = mul_assign,
    pairs = {
        (o_<> Scalar<E>, ScalarZ<E>),
        (o_<> Scalar<E>, &ScalarZ<E>),
        (o_<> ScalarZ<E>, Scalar<E>), (o_<> ScalarZ<E>, ScalarZ<E>),
        (o_<> ScalarZ<E>, &Scalar<E>), (o_<> ScalarZ<E>, &ScalarZ<E>),
        (_o<> &Scalar<E>, ScalarZ<E>),
        (r_<> &Scalar<E>, &ScalarZ<E>),
        (_o<> &ScalarZ<E>, Scalar<E>), (_o<> &ScalarZ<E>, ScalarZ<E>),
        (r_<> &ScalarZ<E>, &Scalar<E>), (r_<> &ScalarZ<E>, &ScalarZ<E>),
    }
}

fn multiplication_of_two_nonzero_scalars<E: Curve>(result: E::Scalar) -> Scalar<E> {
    Scalar::from_raw(result)
        .expect("multiplication of two nonzero scalar by prime modulo must be nonzero")
}

matrix! {
    trait = Mul,
    trait_fn = mul,
    output = Scalar<E>,
    output_new = multiplication_of_two_nonzero_scalars,
    point_fn = mul,
    point_assign_fn = mul_assign,
    pairs = {
        (o_<> Scalar<E>, Scalar<E>),
        (o_<> Scalar<E>, &Scalar<E>),
        (_o<> &Scalar<E>, Scalar<E>),
        (r_<> &Scalar<E>, &Scalar<E>),
    }
}

impl<E: Curve> ops::Mul<&Scalar<E>> for Generator<E> {
    type Output = Point<E>;
    fn mul(self, rhs: &Scalar<E>) -> Self::Output {
        Point::from_raw(E::Point::generator_mul(rhs.as_raw()))
            .expect("generator multiplied by non-zero scalar is always a point of group order")
    }
}

impl<E: Curve> ops::Mul<Scalar<E>> for Generator<E> {
    type Output = Point<E>;
    fn mul(self, rhs: Scalar<E>) -> Self::Output {
        self.mul(&rhs)
    }
}

impl<E: Curve> ops::Mul<Generator<E>> for &Scalar<E> {
    type Output = Point<E>;
    fn mul(self, rhs: Generator<E>) -> Self::Output {
        rhs.mul(self)
    }
}

impl<E: Curve> ops::Mul<Generator<E>> for Scalar<E> {
    type Output = Point<E>;
    fn mul(self, rhs: Generator<E>) -> Self::Output {
        rhs.mul(self)
    }
}

impl<E: Curve> ops::Mul<&ScalarZ<E>> for Generator<E> {
    type Output = PointZ<E>;
    fn mul(self, rhs: &ScalarZ<E>) -> Self::Output {
        PointZ::from_raw(E::Point::generator_mul(rhs.as_raw()))
            .expect("sG must be either a point of group order or a zero point")
    }
}

impl<E: Curve> ops::Mul<ScalarZ<E>> for Generator<E> {
    type Output = PointZ<E>;
    fn mul(self, rhs: ScalarZ<E>) -> Self::Output {
        self.mul(&rhs)
    }
}

impl<E: Curve> ops::Mul<Generator<E>> for &ScalarZ<E> {
    type Output = PointZ<E>;
    fn mul(self, rhs: Generator<E>) -> Self::Output {
        rhs.mul(self)
    }
}

impl<E: Curve> ops::Mul<Generator<E>> for ScalarZ<E> {
    type Output = PointZ<E>;
    fn mul(self, rhs: Generator<E>) -> Self::Output {
        rhs.mul(self)
    }
}

impl<E: Curve> ops::Neg for Scalar<E> {
    type Output = Scalar<E>;

    fn neg(self) -> Self::Output {
        Scalar::from_raw(self.as_raw().neg()).expect("neg must not produce zero point")
    }
}

impl<E: Curve> ops::Neg for &Scalar<E> {
    type Output = Scalar<E>;

    fn neg(self) -> Self::Output {
        Scalar::from_raw(self.as_raw().neg()).expect("neg must not produce zero point")
    }
}

impl<E: Curve> ops::Neg for ScalarZ<E> {
    type Output = ScalarZ<E>;

    fn neg(self) -> Self::Output {
        ScalarZ::from_raw(self.as_raw().neg())
    }
}

impl<E: Curve> ops::Neg for &ScalarZ<E> {
    type Output = ScalarZ<E>;

    fn neg(self) -> Self::Output {
        ScalarZ::from_raw(self.as_raw().neg())
    }
}

impl<E: Curve> ops::Neg for Point<E> {
    type Output = Point<E>;

    fn neg(self) -> Self::Output {
        Point::from_raw(self.as_raw().neg_point()).expect("neg must not produce zero point")
    }
}

impl<E: Curve> ops::Neg for &Point<E> {
    type Output = Point<E>;

    fn neg(self) -> Self::Output {
        Point::from_raw(self.as_raw().neg_point()).expect("neg must not produce zero point")
    }
}

impl<'p, E: Curve> ops::Neg for PointRef<'p, E> {
    type Output = Point<E>;

    fn neg(self) -> Self::Output {
        Point::from_raw(self.as_raw().neg_point()).expect("neg must not produce zero point")
    }
}

impl<E: Curve> ops::Neg for Generator<E> {
    type Output = Point<E>;

    fn neg(self) -> Self::Output {
        Point::from_raw(self.as_raw().neg_point()).expect("neg must not produce zero point")
    }
}

impl<E: Curve> ops::Neg for PointZ<E> {
    type Output = PointZ<E>;

    fn neg(self) -> Self::Output {
        PointZ::from_raw(self.as_raw().neg_point()).expect("negated point must have the same order")
    }
}

impl<E: Curve> ops::Neg for &PointZ<E> {
    type Output = PointZ<E>;

    fn neg(self) -> Self::Output {
        PointZ::from_raw(self.as_raw().neg_point()).expect("negated point must have the same order")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_operator_defined_for {
        (
            assert_fn = $assert_fn:ident,
            lhs = {},
            rhs = {$($rhs:ty),*},
        ) => {
            // Corner case
        };
        (
            assert_fn = $assert_fn:ident,
            lhs = {$lhs:ty $(, $lhs_tail:ty)*},
            rhs = {$($rhs:ty),*},
        ) => {
            assert_operator_defined_for! {
                assert_fn = $assert_fn,
                lhs = $lhs,
                rhs = {$($rhs),*},
            }
            assert_operator_defined_for! {
                assert_fn = $assert_fn,
                lhs = {$($lhs_tail),*},
                rhs = {$($rhs),*},
            }
        };
        (
            assert_fn = $assert_fn:ident,
            lhs = $lhs:ty,
            rhs = {$($rhs:ty),*},
        ) => {
            $($assert_fn::<E, $lhs, $rhs>());*
        };
    }

    /// Function asserts that P2 can be added to P1 (ie. P1 + P2) and result is PointZ.
    /// If any condition doesn't meet, function won't compile.
    #[allow(dead_code)]
    fn assert_point_addition_defined<E, P1, P2>()
    where
        P1: ops::Add<P2, Output = PointZ<E>>,
        E: Curve,
    {
        // no-op
    }

    #[test]
    fn test_point_addition_defined() {
        fn _curve<E: Curve>() {
            assert_operator_defined_for! {
                assert_fn = assert_point_addition_defined,
                lhs = {Point<E>, PointZ<E>, &Point<E>, &PointZ<E>, PointRef<E>, Generator<E>},
                rhs = {Point<E>, PointZ<E>, &Point<E>, &PointZ<E>, PointRef<E>, Generator<E>},
            }
        }
    }

    /// Function asserts that P2 can be subtracted from P1 (ie. P1 - P2) and result is PointZ.
    /// If any condition doesn't meet, function won't compile.
    #[allow(dead_code)]
    fn assert_point_subtraction_defined<E, P1, P2>()
    where
        P1: ops::Sub<P2, Output = PointZ<E>>,
        E: Curve,
    {
        // no-op
    }

    #[test]
    fn test_point_subtraction_defined() {
        fn _curve<E: Curve>() {
            assert_operator_defined_for! {
                assert_fn = assert_point_subtraction_defined,
                lhs = {Point<E>, PointZ<E>, &Point<E>, &PointZ<E>, PointRef<E>, Generator<E>},
                rhs = {Point<E>, PointZ<E>, &Point<E>, &PointZ<E>, PointRef<E>, Generator<E>},
            }
        }
    }

    /// Function asserts that M can be multiplied by N (ie. M * N) and result is PointZ.
    /// If any condition doesn't meet, function won't compile.
    #[allow(dead_code)]
    fn assert_point_multiplication_defined<E, M, N>()
    where
        M: ops::Mul<N, Output = PointZ<E>>,
        E: Curve,
    {
        // no-op
    }

    /// Function asserts that M can be multiplied by N (ie. M * N) and result is **non-zero** Point.
    /// If any condition doesn't meet, function won't compile.
    #[allow(dead_code)]
    fn assert_point_nonzero_multiplication_defined<E, M, N>()
    where
        M: ops::Mul<N, Output = Point<E>>,
        E: Curve,
    {
        // no-op
    }

    #[test]
    fn test_point_multiplication_defined() {
        fn _curve<E: Curve>() {
            assert_operator_defined_for! {
                assert_fn = assert_point_nonzero_multiplication_defined,
                lhs = {Point<E>, &Point<E>, PointRef<E>},
                rhs = {Scalar<E>, &Scalar<E>},
            }
            assert_operator_defined_for! {
                assert_fn = assert_point_multiplication_defined,
                lhs = {Point<E>, &Point<E>, PointRef<E>},
                rhs = {ScalarZ<E>, &ScalarZ<E>},
            }
            assert_operator_defined_for! {
                assert_fn = assert_point_multiplication_defined,
                lhs = {PointZ<E>, &PointZ<E>},
                rhs = {Scalar<E>, &Scalar<E>, ScalarZ<E>, &ScalarZ<E>},
            }

            // and vice-versa

            assert_operator_defined_for! {
                assert_fn = assert_point_nonzero_multiplication_defined,
                lhs = {Scalar<E>, &Scalar<E>},
                rhs = {Point<E>, &Point<E>, PointRef<E>},
            }
            assert_operator_defined_for! {
                assert_fn = assert_point_multiplication_defined,
                lhs = {ScalarZ<E>, &ScalarZ<E>},
                rhs = {Point<E>, &Point<E>, PointRef<E>},
            }
            assert_operator_defined_for! {
                assert_fn = assert_point_multiplication_defined,
                lhs = {Scalar<E>, &Scalar<E>, ScalarZ<E>, &ScalarZ<E>},
                rhs = {PointZ<E>, &PointZ<E>},
            }
        }
    }

    /// Function asserts that S2 can be added to S1 (ie. S1 + S2) and result is ScalarZ.
    /// If any condition doesn't meet, function won't compile.
    #[allow(dead_code)]
    fn assert_scalars_addition_defined<E, S1, S2>()
    where
        S1: ops::Add<S2, Output = ScalarZ<E>>,
        E: Curve,
    {
        // no-op
    }

    #[test]
    fn test_scalars_addition_defined() {
        fn _curve<E: Curve>() {
            assert_operator_defined_for! {
                assert_fn = assert_scalars_addition_defined,
                lhs = {Scalar<E>, ScalarZ<E>, &Scalar<E>, &ScalarZ<E>},
                rhs = {Scalar<E>, ScalarZ<E>, &Scalar<E>, &ScalarZ<E>},
            }
        }
    }

    /// Function asserts that S2 can be added to S1 (ie. S1 + S2) and result is ScalarZ.
    /// If any condition doesn't meet, function won't compile.
    #[allow(dead_code)]
    fn assert_scalars_subtraction_defined<E, S1, S2>()
    where
        S1: ops::Sub<S2, Output = ScalarZ<E>>,
        E: Curve,
    {
        // no-op
    }

    #[test]
    fn test_scalars_subtraction_defined() {
        fn _curve<E: Curve>() {
            assert_operator_defined_for! {
                assert_fn = assert_scalars_subtraction_defined,
                lhs = {Scalar<E>, ScalarZ<E>, &Scalar<E>, &ScalarZ<E>},
                rhs = {Scalar<E>, ScalarZ<E>, &Scalar<E>, &ScalarZ<E>},
            }
        }
    }

    /// Function asserts that S1 can be multiplied by S2 (ie. S1 * S2) and result is ScalarZ.
    /// If any condition doesn't meet, function won't compile.
    #[allow(dead_code)]
    fn assert_scalars_multiplication_defined<E, S1, S2>()
    where
        S1: ops::Mul<S2, Output = ScalarZ<E>>,
        E: Curve,
    {
        // no-op
    }

    /// Function asserts that S1 can be multiplied by S2 (ie. S1 * S2) and result is Scalar.
    /// If any condition doesn't meet, function won't compile.
    #[allow(dead_code)]
    fn assert_nonzero_scalars_multiplication_defined<E, S1, S2>()
    where
        S1: ops::Mul<S2, Output = Scalar<E>>,
        E: Curve,
    {
        // no-op
    }

    #[test]
    fn test_scalars_multiplication_defined() {
        fn _curve<E: Curve>() {
            assert_operator_defined_for! {
                assert_fn = assert_scalars_multiplication_defined,
                lhs = {ScalarZ<E>, &ScalarZ<E>},
                rhs = {Scalar<E>, ScalarZ<E>, &Scalar<E>, &ScalarZ<E>},
            }
            assert_operator_defined_for! {
                assert_fn = assert_scalars_multiplication_defined,
                lhs = {Scalar<E>, ScalarZ<E>, &Scalar<E>, &ScalarZ<E>},
                rhs = {ScalarZ<E>, &ScalarZ<E>},
            }
            assert_operator_defined_for! {
                assert_fn = assert_nonzero_scalars_multiplication_defined,
                lhs = {Scalar<E>, &Scalar<E>},
                rhs = {Scalar<E>, &Scalar<E>},
            }
        }
    }
}