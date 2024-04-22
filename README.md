# chimitheque_people_keycloak_exporter

Export Chimithèque users into a keycloak import file.

Static compilation with:
```bash
docker pull messense/rust-musl-cross:x86_64-musl
alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:x86_64-musl'
rust-musl-builder cargo build --release
```

Run in the directory of the Chimithèque `storage.db*` files.