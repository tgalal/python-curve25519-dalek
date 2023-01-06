from curve25519_dalek.ristretto import RistrettoPoint, CompressedRistretto
from curve25519_dalek.scalar import Scalar
    
zero = Scalar.from_u64(0)
zero2 = Scalar.from_u64(0)
one = Scalar.from_u64(1)
two = Scalar.from_u64(2)
five = Scalar.from_u64(5)
ten = Scalar.from_u64(10)

def test_scalar_eq():
    assert zero == zero2
    assert zero2 == zero
    assert zero != one
    assert ten != one

def test_sub():
    assert one - one == zero
    assert one - zero == one
    assert ten - five == five

def test_add():
    assert one + one == two
    assert two + two + one == five
    assert zero + zero == zero
    assert one + zero == one

def test_scalar_mul():
    assert one * one == one
    assert one * zero == zero
    assert zero * ten == zero
    assert one * ten == ten
    assert five * two == ten
