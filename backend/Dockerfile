FROM rust:1.61.0 as builder
WORKDIR /Users/apatel578/Desktop/project/SE577-SoftwareArchitecture/backend
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/Architecture-server /usr/local/bin/Architecture-server
COPY ./resources/repositories.yaml /usr/local/bin/repositories.yaml
CMD ["Architecture-server", "/usr/local/bin/repositories.yaml"]
