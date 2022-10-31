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

def lizard_encode_helper(data, result):
    p = RistrettoPoint.lizard_encode_sha256(data)
    p_bytes = p.compress().to_bytes();
    assert(p_bytes == result);

    p = CompressedRistretto.from_slice(p_bytes).decompress()
    data_out = p.lizard_decode_sha256()
    assert(data_out == data);

def test_lizard_encode():
    lizard_encode_helper(bytes([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]),
            bytes([0xf0, 0xb7, 0xe3, 0x44, 0x84, 0xf7, 0x4c, 0xf0, 0xf, 0x15,
                0x2, 0x4b, 0x73, 0x85, 0x39, 0x73, 0x86, 0x46, 0xbb, 0xbe,
                0x1e, 0x9b, 0xc7, 0x50, 0x9a, 0x67, 0x68, 0x15, 0x22, 0x7e,
                0x77, 0x4f]))

    lizard_encode_helper(bytes([1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]),
        bytes([0xcc, 0x92, 0xe8, 0x1f, 0x58, 0x5a, 0xfc, 0x5c, 0xaa, 0xc8, 0x86,
           0x60, 0xd8, 0xd1, 0x7e, 0x90, 0x25, 0xa4, 0x44, 0x89, 0xa3,
           0x63, 0x4, 0x21, 0x23, 0xf6, 0xaf, 0x7, 0x2, 0x15, 0x6e, 0x65]))

    lizard_encode_helper(bytes([0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]),
        bytes([0xc8, 0x30, 0x57, 0x3f, 0x8a, 0x8e, 0x77, 0x78, 0x67, 0x1f,
            0x76, 0xcd, 0xc7, 0x96, 0xdc, 0xa, 0x23, 0x5c, 0xf1, 0x77, 0xf1,
            0x97, 0xd9, 0xfc, 0xba, 0x6, 0xe8, 0x4e, 0x96, 0x24, 0x74, 0x44]))
