use std::sync::LazyLock;

pub use pulp::{Arch as SimdArch, Simd, WithSimd};

static SIMD_ARCH: LazyLock<SimdArch> = LazyLock::new(SimdArch::new); // Detect CPU only once.

pub fn get_simd_arch() -> SimdArch {
    *SIMD_ARCH
}
