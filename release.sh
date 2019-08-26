#!/bin/bash

set -e

if [ "$1" = "" ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

version="$1"
version_sane="$(echo $version | sed -Ee 's/^([0-9]{1,}\.){2}[0-9]{1,}$/YES/')"
if [ "$version_sane" != "YES" ]; then
    echo "Version should be in the format '12.34.56'"
    exit 1
fi

if [[ $(git diff --stat) != '' ]]; then
    echo "Dirty working directory"
    exit 1
fi

echo "Making sure we're up-to-date"
git fetch --tags

tag="$(git tag --contains)"
branch="$(git symbolic-ref --quiet --short HEAD || git rev-parse --short HEAD)"
if [ "$tag" = "v$version" ]; then
    echo "Already tagged!"
    exit 1
fi

echo "Updating Cargo.toml version"
sed -i "s/version = \"[0-9]\{1,\}\.[0-9]\{1,\}\.[0-9]\{1,\}\"  # Package version/version = \"$version\"  # Package version/" */Cargo.toml
if grep -q "version = \"$version\"" Cargo.toml; then
    echo "Version update in Cargo.toml succeeded"
else
    echo "Version update failed"
fi

echo "Committing Cargo.toml"
git add */Cargo.toml
git commit -m "Releasing version $version"

echo "Creating tag"
git tag -s "v$version" -m "Release $version"

echo "Building cargo release"
pushd m4ri-sys; cargo publish --no-verify; popd
echo "Waiting for crates.io to settle"; sleep 5
pushd m4ri-rust; cargo publish --no-verify; popd

echo "Pushing git objects"
git push
git push --tags
