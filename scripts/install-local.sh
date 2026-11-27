#!/usr/bin/env bash
# Install TweaksPlus from a GitHub release into ~/.local without building it.
# Works on Debian/Ubuntu (.deb asset) and Arch/CachyOS (portable tar.xz asset,
# with a .deb fallback that does not require dpkg-deb).
#
# Environment:
#   RELEASE_TAG   Install a specific release tag, for example v1.1.0.
#                 By default the latest release is used.
#   GITHUB_TOKEN  Optional GitHub token for API rate limits.

set -euo pipefail

APP_ID="com.mkhrussell.TweaksPlus"
REPO="mkhrussell/TweaksPlus"

die() {
    printf 'error: %s\n' "$*" >&2
    exit 1
}

need() {
    command -v "$1" >/dev/null 2>&1 || die "required command not found: $1"
}

need curl
need python3
need tar

if command -v dpkg >/dev/null 2>&1; then
    ARCH="$(dpkg --print-architecture 2>/dev/null || true)"
else
    # Arch/CachyOS does not ship dpkg. Map the kernel architecture to the
    # Debian-style names used in release asset filenames.
    case "$(uname -m)" in
        x86_64) ARCH=amd64 ;;
        aarch64|arm64) ARCH=arm64 ;;
        armv7*) ARCH=armhf ;;
    esac
fi
[[ -n "${ARCH:-}" ]] || die "could not determine the package architecture"

token="${GITHUB_TOKEN:-}"
if [[ -z "$token" ]] && command -v git >/dev/null 2>&1; then
    token="$(git credential fill 2>/dev/null <<'EOF' | awk -F= '/^password=/{print substr($0, 10); exit}' || true
protocol=https
host=github.com

EOF
)"
fi

auth_args=()
if [[ -n "$token" ]]; then
    auth_args=(-H "Authorization: Bearer $token")
fi

if [[ -n "${RELEASE_TAG:-}" ]]; then
    api="https://api.github.com/repos/$REPO/releases/tags/$RELEASE_TAG"
else
    api="https://api.github.com/repos/$REPO/releases/latest"
fi

release_json="$(curl -fsS "${auth_args[@]}" "$api")" || die "could not fetch release information from GitHub"

if [[ -n "${RELEASE_TAG:-}" ]]; then
    tag="$RELEASE_TAG"
else
    tag="$(python3 -c 'import json,sys; print(json.load(sys.stdin)["tag_name"])' <<<"$release_json")"
fi

asset_fields="$(python3 - "$release_json" "$ARCH" <<'PY' 2>/dev/null
import json
import sys

data = json.loads(sys.argv[1])
arch = sys.argv[2]
suffixes = (".tar.xz", ".tar.gz", ".deb")
assets = [
    asset for asset in data.get("assets", [])
    if asset["name"].startswith("tweaksplus_")
    and any(asset["name"].endswith("_" + arch + suffix) for suffix in suffixes)
]
if not assets:
    print("no tweaksplus package found for architecture " + arch, file=sys.stderr)
    sys.exit(1)
# Prefer the portable archive; fall back to the Debian package.
assets.sort(key=lambda asset: suffixes.index(
    next(suffix for suffix in suffixes if asset["name"].endswith("_" + arch + suffix))
))
asset = assets[0]
print(asset["name"])
print(asset["browser_download_url"])
PY
)" || die "no compatible TweaksPlus package was found in $tag"

mapfile -t asset_info <<<"$asset_fields"
asset_name="${asset_info[0]}"
asset_url="${asset_info[1]}"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

printf 'Downloading %s from %s\n' "$asset_name" "$tag"
curl -fsSL "$asset_url" -o "$tmp/$asset_name"

mkdir -p "$tmp/root"
case "$asset_name" in
    *.tar.xz|*.tar.gz)
        tar -xf "$tmp/$asset_name" -C "$tmp/root"
        ;;
    *.deb)
        if command -v dpkg-deb >/dev/null 2>&1; then
            dpkg-deb -x "$tmp/$asset_name" "$tmp/root"
        else
            # Arch/CachyOS: extract the data member with binutils + tar.
            need ar
            member="$(ar t "$tmp/$asset_name" | grep '^data\.tar' | head -1 || true)"
            [[ -n "$member" ]] || die "$asset_name has no data.tar member"
            case "$member" in
                *.gz)  ar p "$tmp/$asset_name" "$member" | tar -xz -C "$tmp/root" ;;
                *.xz)  ar p "$tmp/$asset_name" "$member" | tar -xJ -C "$tmp/root" ;;
                *.zst) ar p "$tmp/$asset_name" "$member" | tar --zstd -x -C "$tmp/root" ;;
                *)     ar p "$tmp/$asset_name" "$member" | tar -x -C "$tmp/root" ;;
            esac
        fi
        ;;
    *) die "unsupported release asset: $asset_name" ;;
esac

src="$tmp/root/usr"
if [[ ! -f "$src/bin/tweaksplus" ]]; then
    # Some archives contain a top-level version directory.
    found="$(find "$tmp/root" -path '*/usr/bin/tweaksplus' -print -quit 2>/dev/null || true)"
    [[ -n "$found" ]] || die "package is missing usr/bin/tweaksplus"
    src="${found%/usr/bin/tweaksplus}"
fi
[[ -f "$src/bin/tweaksplus" ]] || die "package is missing usr/bin/tweaksplus"
[[ -f "$src/share/applications/$APP_ID.desktop" ]] || die "package is missing the desktop file"

local_bin="$HOME/.local/bin"
local_apps="$HOME/.local/share/applications"
local_icons="$HOME/.local/share/icons/hicolor"

install -d "$local_bin" "$local_apps" \
    "$local_icons/scalable/apps" "$local_icons/512x512/apps"

install -m 0755 "$src/bin/tweaksplus" "$local_bin/tweaksplus"

awk -v cmd="\"$local_bin/tweaksplus\"" -v raw="$local_bin/tweaksplus" '
    /^Exec=/ { print "Exec=" cmd; next }
    /^TryExec=/ { next }
    { print }
    END { print "TryExec=" raw }
' "$src/share/applications/$APP_ID.desktop" >"$tmp/$APP_ID.desktop"
install -m 0644 "$tmp/$APP_ID.desktop" "$local_apps/$APP_ID.desktop"

if [[ -f "$src/share/icons/hicolor/scalable/apps/$APP_ID.svg" ]]; then
    install -m 0644 \
        "$src/share/icons/hicolor/scalable/apps/$APP_ID.svg" \
        "$local_icons/scalable/apps/$APP_ID.svg"
fi
if [[ -f "$src/share/icons/hicolor/512x512/apps/$APP_ID.png" ]]; then
    install -m 0644 \
        "$src/share/icons/hicolor/512x512/apps/$APP_ID.png" \
        "$local_icons/512x512/apps/$APP_ID.png"
fi

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$local_apps" >/dev/null 2>&1 || true
fi
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -f -t "$local_icons" >/dev/null 2>&1 || true
fi

printf 'Installed TweaksPlus %s into %s/.local\n' "$tag" "$HOME"
printf 'Launch it with: %s\n' "$local_bin/tweaksplus"
