FROM alpine:3.19 as build
WORKDIR /build
COPY . .
RUN apk add --no-cache gcc musl-dev rust cargo git
RUN cargo build --release

FROM alpine:3.19
COPY --from=build ./target/release/iptables_exporter /usr/local/bin/
CMD [ "iptables_exporter" ]
