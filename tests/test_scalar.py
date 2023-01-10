from curve25519_dalek.ristretto import RistrettoPoint, CompressedRistretto
from curve25519_dalek.scalar import Scalar
    
zero = Scalar.from_u64(0)
zero2 = Scalar.from_u64(0)
one = Scalar.from_u64(1)
two = Scalar.from_u64(2)
five = Scalar.from_u64(5)
ten = Scalar.from_u64(10)

X = Scalar.from_bytes_mod_order(bytes([
    0x4e, 0x5a, 0xb4, 0x34, 0x5d, 0x47, 0x08, 0x84,
    0x59, 0x13, 0xb4, 0x64, 0x1b, 0xc2, 0x7d, 0x52,
    0x52, 0xa5, 0x85, 0x10, 0x1b, 0xcc, 0x42, 0x44,
    0xd4, 0x49, 0xf4, 0xa8, 0x79, 0xd9, 0xf2, 0x04
]))

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


def test_from_bytes_mod_order():
    l_plus_two_bytes = bytes([
        0xef, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58,
        0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde, 0x14,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10,
    ])
    a = Scalar.from_bytes_mod_order(l_plus_two_bytes)
    assert a == two

def test_from_bytes_mod_order_wide():
    bignum = bytearray(64)
    for i in range(0, 32):
        bignum[i] = X[i]
        bignum[32 + i] = X[i]

    reduced = Scalar.from_bytes_mod_order(bytes([
        216, 154, 179, 139, 210, 121,   2,  71,
        69,  99, 158, 216,  23, 173,  63, 100,
        204,   0,  91,  50, 219, 153,  57, 249,
        28,  82,  31, 197, 100, 165,
        192,   8,
    ]))

    test_red = Scalar.from_bytes_mod_order_wide(bytes(bignum))

    for i in range(0, 32):
        assert test_red[i] == reduced[i]

