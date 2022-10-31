# python-curve25519-dalek

A python library providing operations in [Ristretto
group](https://ristretto.group/). This is only an extension module where the
implementation itself is in rust and provided by
[dalek-cryptography/curve25519-dalek](https://github.com/dalek-cryptography/curve25519-dalek).
More specifically, support for Elligator inverse was desired which, at the time
of publishing, is only present in [Signal's
Fork](https://github.com/signalapp/curve25519-dalek) and is therefore the one
this library uses.

Currently this library only exposes a very limited set of APIs from rust to
python, more will gradually be added as demand requires. Feel free to open an
issue indicating particular interfaces you'd like to prioritize. PRs
incorporating those changes are also welcome, make sure to follow the
[contribution guidelines](CONTRIBUTING.md).

## Usage

```python
from curve25519_dalek.ristretto import RistrettoPoint, CompressedRistretto
from curve25519_dalek.scalar import Scalar
from curve25519_dalek.constants import RISTRETTO_BASEPOINT_POINT

point: RistrettoPoint = RISTRETTO_BASEPOINT_POINT
scalar: Scalar = Scalar.from_u64(999)
product: RistrettoPoint = point * scalar
compressed_product: CompressedRistretto = product.compress()

print(compressed_product.as_bytes())
```

## Dev Building and Testing

(maybe) Create a virtual env:

```
python -m venv .venv
source ./.venv/bin/activate
```

Install dev dependencies:

```
pip install -r requirements-dev.txt
```

Build:

```
python setup.py build
```

Build and install:

```
python setup.py develop
```

Run tests:

```
pytest
```

## Build Release Package

```
pip install build
python -m build
```
