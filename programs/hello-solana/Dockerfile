# Get rust and solana toolchain
FROM ellipsislabs/solana:latest
# Install the Solana Verify CLI
RUN git clone https://github.com/bobur1/hello-solana.git /build
RUN git checkout a41597f
# Get the code of the program to build and verify
RUN cd programs/hello-solana && cargo build-sbf -- --locked --frozen