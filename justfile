# SPDX-License-Identifier: MPL-2.0
#
# Copyright © 2025 RemasteredArch
#
# This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
# copy of the Mozilla Public License was not distributed with this file, You can obtain one at
# <https://mozilla.org/MPL/2.0/>.

default: check test rust-doc

build:
    cargo build

test:
    cargo test

bench once='false':
    #! /bin/sh
    set -eu

    if [ "{{once}}" = 'true' ] || [ "{{once}}" = 'once' ]; then
        cargo bench -- --ignored -- 'bench_'
    else
        executable="$(
            cargo bench --no-run -- --ignored -- 'bench_' 2>&1 \
                | grep '^ \+Executable' \
                | sed 's/.* (\(target\/.*\))$/\1/'
        )"
        hyperfine --shell 'none' --warmup 500 -- "./$executable"
    fi

check:
    cargo clippy

rust-doc:
    cargo doc

typst-doc:
    [ -d './out/' ] || mkdir './out/';

    for file in ./docs/*.typ; do \
        typst compile "$file" "./out/$(basename "$file" '.typ').pdf"; \
    done

watch:
    watchexec --quiet --clear --watch './src/' -- \
        'cargo doc && cargo t --quiet'

ci: ci-rust ci-typst

ci-rust:
    cargo test --verbose
    cargo clippy --verbose
    cargo clippy --features 'serde' --verbose
    cargo build --release --verbose
    cargo fmt --all -- --check
    cargo doc --verbose
    cargo doc --verbose --document-private-items

# This just uses the default Typst build step for now. I'm making a `ci-typst` recipe now because
# I'm likely to add linting for Typst documents in the future.
ci-typst: typst-doc
