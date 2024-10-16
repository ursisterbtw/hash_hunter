import os
from setuptools import setup, Extension
from Cython.Build import cythonize
import numpy as np

current_dir = os.path.dirname(os.path.abspath(__file__))

main_pyx_path = os.path.join(current_dir, "main.pyx")

extensions = [
    Extension(
        "main",
        [main_pyx_path],
        include_dirs=[np.get_include()],
        extra_compile_args=["-O3", "-march=native", "-fopenmp"],
        extra_link_args=["-fopenmp"],
    )
]

setup(
    name="hash_hunter",
    ext_modules=cythonize(extensions, language_level=3),
    zip_safe=False,
)
