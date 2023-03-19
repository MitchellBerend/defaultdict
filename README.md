[![Crates.io](https://img.shields.io/crates/v/defaultdict.svg)](https://crates.io/crates/defaultdict)
[![codecov](https://codecov.io/gh/MitchellBerend/defaultdict/branch/master/graph/badge.svg?token=S1VNH7GNGP)](https://codecov.io/gh/MitchellBerend/defaultdict)


# Motivation
This serves as an utility library and an example project. It has no dependencies
so it can be used in a different project without pulling in other dependencies transitively.

# Description

This library exposes structs that mimicks the behaviour of the python
[defaultdict](https://docs.python.org/3/library/collections.html#collections.defaultdict).

This behaviour does require that the type of the value does have the [Default](https://doc.rust-lang.org/std/default/trait.Default.html) implemented.
