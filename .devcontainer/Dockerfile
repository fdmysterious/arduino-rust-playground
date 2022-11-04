FROM rust:bullseye

# Arguments
ARG SERIALGID=986 # GID of group used to access serial devices

# Install dependencies
RUN apt-get update -y
RUN apt-get install -y sudo
RUN apt-get install -y libudev-dev gcc-avr avrdude avr-libc pkg-config build-essential

# Create user
RUN delgroup dialout
RUN addgroup dialout --gid ${SERIALGID}
RUN adduser builder
RUN usermod -aG dialout builder

# Install various tools
USER builder
RUN cargo install cargo-generate
RUN cargo install ravedude