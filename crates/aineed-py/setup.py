from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension

setup(
    name="aineed",
    version="0.1.1",
    packages=find_packages(where="python"),
    package_dir={"": "python"},
    rust_extensions=[RustExtension("aineed.aineed", binding=Binding.PyO3)],
    include_package_data=True,
    zip_safe=False,
    python_requires=">=3.8",
    author="Nbiish",
    author_email="nbiish@umich.edu",
    description="AI assistant CLI tool for multiple providers",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/nbiish/aineed",
    classifiers=[
        "Development Status :: 4 - Beta",
        "Environment :: Console",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Rust",
        "Topic :: Software Development :: Libraries :: Python Modules",
    ],
    entry_points={
        "console_scripts": [
            "aineed=aineed.__main__:main",
        ],
    },
) 