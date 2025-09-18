# DirectXTex Rust Bindings

These are unofficial Rust bindings for the DirectXTex texture processing library. A C++17 compliant compiler is required to build this library, and it is verified to work on Windows, Linux, and MacOS. The docs contain a migration table that describes the equivalency between the original C++ API and its Rust bindings. Please see [DirectXTex's README](https://github.com/microsoft/DirectXTex/blob/main/README.md) for more info.

The stable release docs are available at: <https://docs.rs/directxtex/latest/directxtex/>

Changelogs are available at: <https://github.com/Ryan-rsm-McKenzie/directxtex-rs/releases>


## OpenMP Support

DirectXTex uses [OpenMP](https://www.openmp.org/) for parallel processing. To enable OpenMP in this crate, build with the `openmp` feature:

### Platform-specific Instructions

- **Windows:**
  - Visual Studio supports OpenMP via the vcomp runtime DLL. Ensure `vcomp.dll` is available at runtime (it is included with the MSVC redistributable).

- **Linux:**
  - Most distributions provide OpenMP support out-of-the-box with GCC or Clang.
  - If you encounter issues, install the OpenMP development package for your compiler

- **macOS:**
  - Install OpenMP runtime via Homebrew:
    ```sh
    brew install libomp
    ```
  - The build script will automatically detect and link Homebrew's libomp. If you see errors about `omp.h` not found or linker errors, ensure your Homebrew is up to date and try:
    ```sh
    export LIBRARY_PATH="$(brew --prefix libomp)/lib${LIBRARY_PATH:+:$LIBRARY_PATH}"
    export CXXPATH="$(brew --prefix libomp)/include${CXXPATH:+:$CXXPATH}"
    ```
> All platforms will statically link to OpenMP except for Windows. On Windows, make sure that `vcomp.dll` is available at runtime