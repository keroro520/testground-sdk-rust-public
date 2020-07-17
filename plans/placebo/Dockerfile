ARG build_image
ARG run_image

# === BUILER IMAGE === #
FROM ${build_image} AS builder
WORKDIR /build
COPY . .

# "/build" is workdir.
# "/build/plan" is our source directory which has been specified
# in [testground daemon](https://github.com/testground/testground/blob/a4a1a83b56db92983e7b2a851751c59a00a4cbb1/pkg/daemon/build.go#L108)
RUN cd plan && cargo build --release # && cp target/release/placebo /testplan

# === RUNNER IMAGE === #

FROM ${run_image}
COPY --from=builder /testplan /testplan
EXPOSE 6060
ENTRYPOINT [ "/testplan" ]
