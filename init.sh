#!/usr/bin/env bash

set -eo pipefail

#
# Quick script to convert a standard Postgres data directory
# into a collection of branch-able snapshots using a copy-on-write
# file system of choice (only btrfs supported for now).
#
# The target directory for this script is whatever is specified in
# the $PGDATA environment variable.
#

# scan the args for configuration
fs=btrfs
for arg in "$@"; do
	case "$arg" in
		--btrfs)
			fs='master'
			;;
	esac
        # TODO: support zfs, xfs
done

# convert the segment data directories in $PGDATA/base to $fs subvolumes
while read -r SEGMENT; do
  echo "converting $SEGMENT to $fs subvolume..."
  mv "$SEGMENT" "${SEGMENT}_old" 
  $fs subvolume create "$SEGMENT"
  cp ${SEGMENT}_old/* "$SEGMENT/"
  rm -rf "${SEGMENT}_old"
done <<<$(find "$PGDATA/base" -maxdepth 1 -type d | tail -n +2)

echo "$fs conversion of $PGDATA complete!"
