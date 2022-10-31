# curve25519-dalek

## Usage

```python
from curve25519_dalek.ristretto import RistrettoPoint, CompressedRistretto
from curve25519_dalek.scalar import Scalar
from curve25519_dalek.constants import RISTRETTO_BASEPOINT_POINT

point: RistrettoPoint = RISTRETTO_BASEPOINT_POINT
scalar: Scalar = Scalar.from64(999)
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
