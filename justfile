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
    #!/bin/sh
    # Print lines as they're executed, error when accessing unset variables, and exit on any error.
    set -exu

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
    cargo fmt -- --check
    just ci-toml
    actionlint

fmt:
    cargo fmt
    taplo fmt
    mdformat .
    yamlfmt .

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

ci: ci-rust ci-typst ci-yaml ci-toml ci-markdown

# This is also replicated in `/.github/workflows/rust.yml`. Don't forget to update that (or ping
# somebody else to do so)!
ci-rust:
    #!/bin/sh
    # Print lines as they're executed, error when accessing unset variables, and exit on any error.
    set -exu
    # Elevate all warnings to errors.
    export RUSTFLAGS='-D warnings'

    # Build code normally.
    cargo build --verbose \
        --release

    # Test code normally.
    #
    # As of right now, regular tests do include Serde tests, so no need for a `--all-features` run.
    cargo test --verbose

    # Lint code.
    cargo clippy --verbose
    # Lint code with all features enabled.
    cargo clippy --verbose \
        --all-features

    # Check that code is properly formatted.
    cargo fmt --verbose -- --verbose \
        --check

    # Check that docs build cleanly.
    cargo doc --verbose
    # Check that private docs build cleanly too.
    cargo doc --verbose \
        --document-private-items
    # Check docs build cleanly with all features enabled.
    cargo doc --verbose \
        --all-features

    # Check dependencies for duplicated/banned crates, incompatible licenses, and untrusted
    # sources.
    cargo deny --all-features check \
        bans licenses sources
    # Check for security advisories from dependencies (or unmaintained/yanked dependencies).
    #
    # Seperated from other `cargo deny` checks to distinguish these warnings from the more
    # predictable checks.
    cargo deny --all-features check \
        advisories

# This just uses the default Typst build step for now. I'm making a `ci-typst` recipe now because
# I'm likely to add linting for Typst documents in the future.
ci-typst: typst-doc

ci-yaml:
    yamlfmt -lint .
    # actionlint cannot recurse on its own without `.git/`, which `actions/checkout` does not
    # provide. Accordingly, we do that ourselves.
    #
    # It is necessary to use `find` instead of just `actionlint .github/workflows/*` because GitHub
    # is considering allowing subdirectories.
    find '.github/workflows/' \
        \( -name '*.yml' -o -name '*.yaml' \) \
        -print0 \
        | xargs -0 actionlint

ci-toml:
    taplo fmt --check --diff
    taplo lint

ci-markdown:
    # Mdformat has a built-in `exclude` option, but this requires Python 3.13+. Ubuntu 24.04
    # doesn't even ship that, and I'd rather do this than require a version of Python so new for a
    # Markdown formatter.
    find '.' \
        -name '*.md' \
        \! \( -path './target/*' -o -path './out/*' -o -path './git/*' \) \
        -print0 \
        | xargs -0 mdformat --check

act *FLAGS:
    # `FLAGS` will split on spaces.
    sudo "$(which act)" -P 'ubuntu-24.04=catthehacker/ubuntu:act-22.04' {{ FLAGS }}

act-general:
    just act --workflows './.github/workflows/general.yml'

act-rust:
    just act --workflows './.github/workflows/rust.yml'
