FROM rustlang/rust:nightly

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld git libssl-dev pkg-config libpq-dev curl postgresql-client \
   && apt-get autoremove -y && apt-get clean -y
RUN rustup component add rustfmt --toolchain nightly

WORKDIR /workspaces/lemmy

RUN cargo install diesel_cli --no-default-features --features postgres

COPY scripts/devcontainer-entrypoint.sh devcontainer-entrypoint.sh
RUN chmod +x devcontainer-entrypoint.sh

ENTRYPOINT [ "devcontainer-entrypoint.sh" ]
CMD [ "cargo", "run"]