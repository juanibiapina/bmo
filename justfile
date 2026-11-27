test:
    cargo test

test-desktop:
    cargo test -- --include-ignored

# Build against the fork's local checkout at ../gpui-kit, where the fix
# branches are written, without changing Cargo.toml or the lock file. Cargo
# warns that the override alters the dependency list: the crates are the same
# ones, read from a folder instead of from git.
local-fork:
    mkdir -p .cargo
    printf 'paths = [\n%s]\n' "$(for crate in shell base component component-macros fps kit assets webview; do printf '  "../gpui-kit/crates/%s",\n' "$crate"; done)" > .cargo/config.toml
    @echo "bmo now builds against ../gpui-kit; remove .cargo/config.toml to build against the published fork"

# Publish the fork with every fix branch applied, and move bmo to it.
publish-fork:
    scripts/publish-fork
    cargo update -p gpui-shell -p gpui-base
