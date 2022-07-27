# Raybow
A simple, small personal ray-tracing renderer, based on the *Ray Tracing in One Weekend* by Peter Shirley, written in Rust.

## About

This is a simple command-line program - no time for GUIs (yet). It uses the hardcoded scene (currently, importing objects/scenes from other files is not implemented) and produces a ``.ppm`` image file with the result.

## How to run it

When I get around to it, I'll add some binary releases.

Currently, one can run the application by first installing ``Rust`` and ``Cargo``. Then, one can clone this repository and run ``cargo build`` at the directory that houses ``Cargo.toml``. Built binaries should be in the newly created ``/target`` directory.

### Parameters
Currently, the program requires following parameters:

```
./<program-name> out_filename
```
* ``out_filename`` - the name of the output image (without the extension)