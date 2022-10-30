import pycurve25519_dalek

def test_lib():
    basepoint = pycurve25519_dalek.RISTRETTO_BASEPOINT_POINT
    print(basepoint)
