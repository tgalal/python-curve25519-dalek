from curve25519_dalek.ristretto import RistrettoPoint, CompressedRistretto
from curve25519_dalek.scalar import Scalar
from curve25519_dalek.constants import RISTRETTO_BASEPOINT_POINT

def test_scalarmult_ristrettopoint_works_both_ways():
    P = RISTRETTO_BASEPOINT_POINT
    s = Scalar.from64(999)
    P1 = P * s
    P2 = s * P

    assert P1.compress().as_bytes() == P2.compress().as_bytes()

def test_impl_sum():
    BASE = RISTRETTO_BASEPOINT_POINT
    s1 = Scalar.from64(999)
    P1 = BASE * s1

    s2 = Scalar.from64(333);
    P2 = BASE * s2;

    vec = [P1, P2]

def test_elligator_vs_ristretto_sage():
    data =  bytes([184, 249, 135, 49, 253, 123, 89, 113, 67, 160, 6, 239,
            7, 105, 211, 41, 192, 249, 185, 57, 9, 102, 70, 198, 15, 127, 7,
            26, 160, 102, 134, 71])
    encoded_images = CompressedRistretto(
            bytes([176, 157, 237, 97, 66, 29, 140, 166, 168, 94, 26, 157,
                212, 216, 229, 160, 195, 246, 232, 239, 169, 112, 63, 193, 64,
                32, 152, 69, 11, 190, 246, 86])
    )


def test_multiscalar_mul():
    s1 = Scalar.from64(999)
    P1 = RISTRETTO_BASEPOINT_POINT * s1
    scalars = [s1, s1, s1]
    points = [P1, P1, P1]
    res = RistrettoPoint.multiscalar_mul(scalars, points)
    print(res)
