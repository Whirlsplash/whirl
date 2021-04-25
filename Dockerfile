# # NOTICE
# This Dockerfile is *probably* not stable as of 2021. 04. 25.
# 
# Can you use it? Yes, but you probably shouldn't.

FROM rustlang/rust:nightly-slim AS build

WORKDIR /src/whirl

COPY . .

RUN cargo build --release

FROM ubuntu:18.04

COPY --from=build /src/whirl/target/release/whirl /usr/local/bin/whirl

CMD ["usr/local/bin/whirl", "run"]
