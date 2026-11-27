#!/usr/bin/env bash
# Build an Arch/CachyOS package from the current checkout.
#
# Usage:
#   scripts/build-arch-package.sh          # build tweaksplus-*.pkg.tar.zst
#   scripts/build-arch-package.sh -si      # build and install with pacman
#
# Any extra arguments are passed through to makepkg. The source archive is
# created from the working tree, so local (unreleased) fixes are included.

set -euo pipefail

die() {
    printf 'error: %s\n' "$*" >&2
    exit 1
}

need() {
    command -v "$1" >/dev/null 2>&1 || die "required command not found: $1"
}

need awk
need tar
need makepkg

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd -- "$script_dir/.." && pwd)"
arch_dir="$root/packaging/arch"

version="$(awk -F'"' '/^version = /{print $2; exit}' "$root/Cargo.toml")"
[[ -n "$version" ]] || die "could not read the version from Cargo.toml"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

stage="$tmp/tweaksplus-$version"
mkdir -p "$stage"
for path in Cargo.toml Cargo.lock LICENSE README.md ROADMAP.md src data; do
    [[ -e "$root/$path" ]] || die "source tree is missing $path"
    cp -a "$root/$path" "$stage/"
done

archive="$arch_dir/tweaksplus-$version-src.tar.xz"
tar -C "$tmp" -cJf "$archive" "tweaksplus-$version"
printf 'Created %s\n' "$archive"

cd "$arch_dir"
export TWEAKSPLUS_LOCAL_SOURCE=1
if (($# > 0)); then
    makepkg "$@"
else
    makepkg -f
fi
