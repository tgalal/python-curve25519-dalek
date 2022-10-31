# -*- coding: utf-8 -*-
import os
from setuptools import find_packages, setup
from setuptools_rust import RustExtension

setup(
    rust_extensions=[
        RustExtension(
            "curve25519_dalek._curve25519_dalek",
            debug=os.environ.get("BUILD_DEBUG")
            == "1",
            )
        ],
)
