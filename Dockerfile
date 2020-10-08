# Globals
ARG PROJ=do-apps-rust
ARG PORT=8080

# Builder
FROM rustlang/rust:nightly-slim as builder

# 'Import' Global ARG
ARG PROJ

RUN USER=root cargo new --bin ${PROJ}

WORKDIR /${PROJ}

COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

# Replace the project's hyphens with underscores
# Stackoverflow: https://stackoverflow.com/a/61827679/609290
RUN /bin/bash -c 'rm ./target/release/deps/${PROJ//-/_}*'

RUN cargo build --release


# Runtime
FROM debian:buster-slim as runtime

# 'Import' Global ARG
ARG PROJ
ARG PORT

WORKDIR /app

COPY --from=builder /${PROJ}/target/release/${PROJ} ./x

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    USER=appuser

RUN groupadd ${USER} \
    && useradd -g ${USER} ${USER} && \
    chown -R ${USER}:${USER} /app

USER ${USER}

EXPOSE ${PORT}

ENTRYPOINT ["./x"]