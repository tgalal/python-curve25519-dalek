__version__ = "0.0.1"
# import the contents of the Rust library into the Python extension
# optional: include the documentation from the Rust module
from ._curve25519_dalek import *
from ._curve25519_dalek import __all__, __doc__
