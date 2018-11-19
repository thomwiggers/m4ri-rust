# Rust bindings for M4RI

[M4RI][m4ri] is a C library that provides efficient implementations of binary matrix operations.
This crate aims to expose those functions to Rust and provides a nice wrapper around matrices, vectors and operations on both.

# References

Martin Albrecht and Gregory Bard. The M4RI Library. [https://malb.bitbucket.io/m4ri][m4ri]

See also the references section of the M4RI docs [here][m4ri references].

# This was written in the context of:

Thom Wiggers. Solving LPN using Large Covering Codes. *Master's Thesis* Radboud University, 2018.

See also https://thomwiggers.nl/research/msc-thesis/

# Optional features

* `serde`: Enable serialization
* M4RI options:
    * `m4rm_mul`: Use `m4rm` as multiplication algorithm
    * `naive_mul`: Use the `naive` strategy
    * `strassen_mul`: Use the Strassen algorithm

# Releases

Releases are available with DOI:

[![DOI](https://zenodo.org/badge/126357607.svg)](https://zenodo.org/badge/latestdoi/126357607)

[m4ri]: https://malb.bitbucket.io/m4ri/
[m4ri references]: https://bitbucket.org/malb/m4ri/wiki/Further%20Reading
