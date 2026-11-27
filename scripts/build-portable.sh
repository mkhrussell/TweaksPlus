#!/usr/bin/env bash
# Build a distro-independent tar.xz release archive.
#
# The archive mirrors the .deb layout (usr/bin, usr/share/applications, icons)
# so install-local.sh can install it on Arch/CachyOS without dpkg.

set -euo pipefail

die() {
    printf 'error: %s\n' "$*" >&2
    exit 1
}

need() {
    command -v "$1" >/dev/null 2>&1 || die "required command not found: $1"
}

need awk
need cargo
need tar

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd -- "$script_dir/.." && pwd)"
cd "$root"

version="$(awk -F'"' '/^version = /{print $2; exit}' Cargo.toml)"
[[ -n "$version" ]] || die "could not read the version from Cargo.toml"

if command -v dpkg >/dev/null 2>&1; then
    ARCH="$(dpkg --print-architecture)"
else
    case "$(uname -m)" in
        x86_64) ARCH=amd64 ;;
        aarch64|arm64) ARCH=arm64 ;;
        armv7*) ARCH=armhf ;;
        *) die "unsupported architecture: $(uname -m)" ;;
    esac
fi
[[ -n "$ARCH" ]] || die "could not determine the package architecture"

name="tweaksplus_${version}_${ARCH}.tar.xz"
output="$(cd -- "$root/.." && pwd)/$name"

printf 'Building tweaksplus %s\n' "$version"
cargo build --release --locked

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
stage="$tmp/usr"
install -d "$stage/bin" \
    "$stage/share/applications" \
    "$stage/share/icons/hicolor/scalable/apps" \
    "$stage/share/icons/hicolor/512x512/apps"
install -m 0755 target/release/tweaksplus "$stage/bin/tweaksplus"
install -m 0644 data/com.mkhrussell.TweaksPlus.desktop \
    "$stage/share/applications/com.mkhrussell.TweaksPlus.desktop"
install -m 0644 data/icons/hicolor/scalable/apps/com.mkhrussell.TweaksPlus.svg \
    "$stage/share/icons/hicolor/scalable/apps/com.mkhrussell.TweaksPlus.svg"
install -m 0644 data/icons/hicolor/512x512/apps/com.mkhrussell.TweaksPlus.png \
    "$stage/share/icons/hicolor/512x512/apps/com.mkhrussell.TweaksPlus.png"

tar -C "$tmp" -cJf "$output" usr
printf 'Created %s\n' "$output"
