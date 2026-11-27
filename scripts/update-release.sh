#!/usr/bin/env bash
# Rebuild TweaksPlus and update the matching GitHub release.
#
# This script:
#   1. rebuilds a portable tar.xz archive (and a .deb when dpkg is available),
#   2. replaces those assets on the release,
#   3. uploads scripts/install-local.sh as a release asset,
#   4. refreshes the release notes with Ubuntu and CachyOS instructions.
#
# Environment:
#   GITHUB_TOKEN  GitHub token with permission to manage release assets.
#                 Without it, a token is read from git's credential store.

set -euo pipefail

REPO="mkhrussell/TweaksPlus"
APP_ID="com.mkhrussell.TweaksPlus"

die() {
    printf 'error: %s\n' "$*" >&2
    exit 1
}

need() {
    command -v "$1" >/dev/null 2>&1 || die "required command not found: $1"
}

need curl
need python3

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd -- "$script_dir/.." && pwd)"
cd "$root"

VERSION="$(awk -F'"' '/^version = /{print $2; exit}' Cargo.toml)"
[[ -n "$VERSION" ]] || die "could not read the package version from Cargo.toml"

if command -v dpkg-parsechangelog >/dev/null 2>&1; then
    deb_version="$(dpkg-parsechangelog -SVersion)"
    [[ "$deb_version" == "$VERSION" ]] || die "Cargo.toml ($VERSION) and debian/changelog ($deb_version) disagree"
fi

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

TAG="v$VERSION"
TARBALL_NAME="tweaksplus_${VERSION}_${ARCH}.tar.xz"
DEB_NAME="tweaksplus_${VERSION}_${ARCH}.deb"
PARENT="$(cd -- "$root/.." && pwd)"
TARBALL="$PARENT/$TARBALL_NAME"
DEB="$PARENT/$DEB_NAME"
INSTALL_SCRIPT="$script_dir/install-local.sh"
[[ -x "$INSTALL_SCRIPT" ]] || die "install-local.sh must be executable"

token="${GITHUB_TOKEN:-}"
if [[ -z "$token" ]] && command -v git >/dev/null 2>&1; then
    token="$(git credential fill 2>/dev/null <<'EOF' | awk -F= '/^password=/{print substr($0, 10); exit}' || true
protocol=https
host=github.com

EOF
)"
fi
[[ -n "$token" ]] || die "a GitHub token is required to update the release"

auth_args=(-H "Authorization: Bearer $token")
api_base="https://api.github.com/repos/$REPO"

printf 'Fetching release %s\n' "$TAG"
release_json="$(curl -fsS "${auth_args[@]}" "$api_base/releases/tags/$TAG")" \
    || die "release $TAG does not exist on GitHub"

release_id="$(python3 -c 'import json,sys; print(json.load(sys.stdin)["id"])' <<<"$release_json")"
upload_url="$(python3 -c 'import json,sys; print(json.load(sys.stdin)["upload_url"].split("{", 1)[0])' <<<"$release_json")"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

printf 'Building %s\n' "$TARBALL_NAME"
"$script_dir/build-portable.sh"
[[ -f "$TARBALL" ]] || die "expected archive was not created: $TARBALL"

if command -v dpkg-deb >/dev/null 2>&1 && command -v dpkg-gencontrol >/dev/null 2>&1; then
    printf 'Building %s\n' "$DEB_NAME"
    rm -rf debian/tweaksplus
    if command -v fakeroot >/dev/null 2>&1; then
        fakeroot debian/rules binary
    else
        debian/rules binary
    fi
    [[ -f "$DEB" ]] || die "expected package was not created: $DEB"
    sha256_deb="$(sha256sum "$DEB" | awk '{print $1}')"
else
    printf 'dpkg tooling not found; skipping the Debian package\n'
fi

sha256_tarball="$(sha256sum "$TARBALL" | awk '{print $1}')"
sha256_script="$(sha256sum "$INSTALL_SCRIPT" | awk '{print $1}')"

printf 'Removing old release assets\n'
python3 - "$release_json" "$DEB_NAME" "$TARBALL_NAME" "install-local.sh" <<'PY' >"$tmp/old-assets"
import json
import sys

data = json.loads(sys.argv[1])
names = set(sys.argv[2:])
for asset in data.get("assets", []):
    if asset["name"] in names:
        print(asset["id"], asset["name"])
PY

while read -r asset_id asset_name; do
    [[ -z "$asset_id" ]] && continue
    printf '  deleting %s\n' "$asset_name"
    curl -fsS -X DELETE "${auth_args[@]}" \
        "$api_base/releases/assets/$asset_id" >/dev/null
done <"$tmp/old-assets"

upload_asset() {
    local file="$1"
    local name="$2"
    local content_type="$3"

    printf '  uploading %s\n' "$name"
    curl -fsS -X POST \
        "${auth_args[@]}" \
        -H "Content-Type: $content_type" \
        --data-binary "@$file" \
        "$upload_url?name=$name" >/dev/null
}

printf 'Uploading new release assets\n'
upload_asset "$TARBALL" "$TARBALL_NAME" "application/x-xz"
if [[ -n "${sha256_deb:-}" ]]; then
    upload_asset "$DEB" "$DEB_NAME" "application/vnd.debian.binary-package"
fi
upload_asset "$INSTALL_SCRIPT" "install-local.sh" "text/x-shellscript"

deb_section=""
checksum_deb=""
if [[ -n "${sha256_deb:-}" ]]; then
    deb_section="### Debian / Ubuntu package

Install \`$DEB_NAME\` with your package manager:

\`\`\`sh
sudo apt install ./$DEB_NAME
\`\`\`
"
    checksum_deb="- \`$DEB_NAME\`: \`$sha256_deb\`
"
fi

arch_section="### Arch / CachyOS package

Build from source with the AUR-style PKGBUILD in \`packaging/arch\`, or track the
latest source with \`packaging/arch-git\`:

\`\`\`sh
makepkg -si
\`\`\`
"

body="$(cat <<EOF
## TweaksPlus $VERSION

TweaksPlus is a native GTK4/Rust alternative to GNOME Tweaks, with live, bidirectional GSettings bindings and 12 configuration panels. The same binary supports Ubuntu 26.04 and CachyOS GNOME.

### User-local install (no build required)

\`\`\`sh
curl -fsSL https://github.com/$REPO/releases/download/$TAG/install-local.sh | bash
\`\`\`

The installer downloads the portable archive (or the \`.deb\` as a fallback) and extracts it into \`~/.local\`: the binary goes to \`~/.local/bin\`, the launcher to \`~/.local/share/applications\`, and the icons to \`~/.local/share/icons/hicolor\`. No Cargo build or system-wide install is needed.

$deb_section
$arch_section
### Asset checksums

- \`$TARBALL_NAME\`: \`$sha256_tarball\`
$checksum_deb- \`install-local.sh\`: \`$sha256_script\`
EOF
)"

python3 - "TweaksPlus $VERSION" "$body" >"$tmp/release.json" <<'PY'
import json
import sys

print(json.dumps({"name": sys.argv[1], "body": sys.argv[2]}))
PY

printf 'Updating release notes\n'
curl -fsS -X PATCH \
    "${auth_args[@]}" \
    -H "Content-Type: application/json" \
    --data-binary "@$tmp/release.json" \
    "$api_base/releases/$release_id" >/dev/null

printf 'Release updated: https://github.com/%s/releases/tag/%s\n' "$REPO" "$TAG"
printf '  %s\n' "https://github.com/$REPO/releases/download/$TAG/$DEB_NAME"
printf '  %s\n' "https://github.com/$REPO/releases/download/$TAG/install-local.sh"
