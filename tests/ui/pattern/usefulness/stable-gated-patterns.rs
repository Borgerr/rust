//@ aux-build:unstable.rs

extern crate unstable;

use unstable::UnstableEnum;

fn main() {
    match UnstableEnum::Stable {
        UnstableEnum::Stable => {}
    }
    //~^^^ ERROR non-exhaustive patterns: `UnstableEnum::Stable2` and `_` not covered

    match UnstableEnum::Stable {
        UnstableEnum::Stable => {}
        UnstableEnum::Stable2 => {}
    }
    //~^^^^ ERROR non-exhaustive patterns: `_` not covered
}
