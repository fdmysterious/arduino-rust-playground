FROM rust:bullseye

# Install dependencies
RUN apt-get update -y
RUN apt-get install -y sudo
RUN apt-get install -y libudev-dev gcc-avr avrdude avr-libc pkg-config build-essential

# Create user
RUN adduser builder

# Install various tools
USER builder
RUN cargo install cargo-generate
RUN cargo install ravedude