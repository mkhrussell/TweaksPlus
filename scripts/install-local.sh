#!/usr/bin/env bash
# Install TweaksPlus from a GitHub release into ~/.local without building it.
#
# Environment:
#   RELEASE_TAG   Install a specific release tag, for example v1.0.0.
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
need dpkg-deb
need python3

ARCH="$(dpkg --print-architecture 2>/dev/null || true)"
[[ -n "$ARCH" ]] || die "could not determine the Debian architecture"

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
assets = [
    asset for asset in data.get("assets", [])
    if asset["name"].startswith("tweaksplus_")
    and asset["name"].endswith("_" + arch + ".deb")
]
if not assets:
    print("no tweaksplus .deb asset found for architecture " + arch, file=sys.stderr)
    sys.exit(1)
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
dpkg-deb -x "$tmp/$asset_name" "$tmp/root"

src="$tmp/root/usr"
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
