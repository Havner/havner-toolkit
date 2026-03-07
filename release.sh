#!/bin/bash

PLUGIN="com.havner.toolkit.sdPlugin"
TARGET="x86_64-unknown-linux-gnu"
TARGETS=("x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu")
INSTALLED_TARGETS="$(rustup target list --installed)"

rm -rf "$PLUGIN"                                                       || exit 1
mkdir "$PLUGIN"                                                        || exit 1
cp -r assets/* "$PLUGIN"/                                              || exit 1

for TARGET in "${TARGETS[@]}"; do
	if grep -qx "$TARGET" <<<"$INSTALLED_TARGETS"; then
		cargo install --target "$TARGET" --path . --root "$PLUGIN/$TARGET"
	else
		echo "Skipping target '$TARGET' (not installed in rustup)."
	fi
done
