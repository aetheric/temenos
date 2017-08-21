FROM ekidd/rust-musl-builder

# Reset the PATH to add /usr/local/sbin, /usr/sbin, and /sbin
ENV PATH=/home/rust/.cargo/bin:/usr/local/musl/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

# Add necessary build packages as root.
USER root
RUN apt-get update \
 && apt-get install -y \
		protobuf-compiler \
 && apt-get clean \
 && rm -rf /var/lib/apt-lists/* \
 && chown -R rust:rust /home/rust

# Switch back to the rust user.
USER rust

# Fetch all the needed dependencies
COPY Cargo.toml Cargo.lock /home/rust/src/
RUN sudo chown -R rust:rust /home/rust \
 && cargo fetch

# Copy the rest of the project files and build them.
COPY . /home/rust/src/
RUN sudo chown -R rust:rust /home/rust \
 && cargo build \
 && cargo test \
 && cargo bench \
 && cargo doc

# Start a new image
FROM scratch

# Grab the binary built in the previous step
COPY --from=0 \
     /home/rust/src/target/x86_64-unknown-linux-musl/release/temenos-identity \
     /

# Set the binary as the entrypoint
ENTRYPOINT [ "/temenos-identity" ]
CMD [ "" ]
