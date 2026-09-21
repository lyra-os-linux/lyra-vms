#!/usr/bin/env bash
# Build reproducible OBS inputs from a committed revision; never the worktree.
set -euo pipefail
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"
rev="$(git rev-parse --verify "${1:-HEAD}^{commit}")"
out_dir="${OUT_DIR:-$repo_root/packaging/obs/out/$rev}"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
git archive "$rev" | tar -xf - -C "$work"
version="$(sed -n 's/^version = "\(.*\)"/\1/p' "$work/Cargo.toml" | head -1)"
name="lyra-vms-$version"
export SOURCE_DATE_EPOCH="$(git log -1 --format=%ct "$rev")"
printf '%s\n' "$rev" > "$work/.source-revision"
mkdir -p "$out_dir"
# Prefix the source archive without depending on the temporary directory name.
tar --format=posix --pax-option=exthdr.name=%d/PaxHeaders/%f,delete=atime,delete=ctime --sort=name --mtime="@$SOURCE_DATE_EPOCH" --owner=0 --group=0 --numeric-owner \
    --transform="s,^\.,$name," -C "$work" -cJf "$out_dir/$name.tar.xz" .
(
    cd "$work"
    mkdir -p .cargo
    cargo vendor --offline --locked --versioned-dirs vendor > .cargo/config.toml
)
tar --format=posix --pax-option=exthdr.name=%d/PaxHeaders/%f,delete=atime,delete=ctime --sort=name --mtime="@$SOURCE_DATE_EPOCH" --owner=0 --group=0 --numeric-owner \
    -C "$work" -cJf "$out_dir/vendor.tar.xz" vendor .cargo/config.toml
cp "$work/packaging/obs/lyra-vms.spec" "$work/packaging/obs/lyra-vms.changes" "$out_dir/"
(cd "$out_dir" && sha256sum "$name.tar.xz" vendor.tar.xz > SHA256SUMS)
printf 'OBS sources: %s\n' "$out_dir"
