#!/usr/bin/env bash
workspace_dir="/workspaces/lemmy"

cd ${workspace_dir}
git config --global --add safe.directory ${workspace_dir}
cargo check

exec "$@"