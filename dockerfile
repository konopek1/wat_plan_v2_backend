FROM ekidd/rust-musl-builder AS build
COPY . ./
RUN sudo chown -R rust:rust .
RUN cargo build --release

FROM scratch
COPY --from=build /home/rust/target/release/server_craweler_wat /
ENV PORT 8181
EXPOSE ${PORT}
CMD ["/server_craweler_wat"]