FROM rustlang/rust:nightly-slim as build
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
#RUN cargo build --release
RUN cargo build
RUN mkdir -p /build-out
#RUN cp target/release/huawei-b525-vpn-connection-api /build-out/
RUN cp target/debug/huawei-b525-vpn-connection-api /build-out/

FROM ubuntu:trusty
ENV DEBIAN_FRONTEND=noninteractive

# Install Chrome
RUN apt-get update && apt-get -y install wget x11vnc xvfb fluxbox wmctrl && rm -rf /var/lib/apt/lists/*
# Set the Chrome repo.
RUN wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add - \
    && echo "deb http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list
# Install Chrome.
RUN apt-get update && apt-get -y install google-chrome-stable && rm -rf /var/lib/apt/lists/*

# Copy Application
COPY --from=build /build-out/huawei-b525-vpn-connection-api /

# Add docker script to run
COPY ./docker/run.sh /
RUN chmod +x /run.sh
ENV CHROME=/usr/bin/google-chrome-unstable
WORKDIR /root
CMD /run.sh