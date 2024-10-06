# set the base image to Debian
# https://hub.docker.com/_/debian/
FROM rust:1.79

ENV TARGET=""

# install rust targets
# RUN rustup target add x86_64-unknown-linux-gnu
# RUN rustup target add x86_64-pc-windows-msvc
# RUN rustup target add x86_64-apple-darwin
# RUN rustup target add aarch64-apple-darwin

# install tauri-cli
RUN cargo install tauri-cli

# update the repository sources list and install dependencies
RUN apt-get update

# install tauri prereqisites
# for tauri 2.0 use webkitgtk 4.1
RUN apt-get install -y libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf

# # replace shell with bash so we can source files
RUN rm /bin/sh && ln -s /bin/bash /bin/sh

# nvm environment variables
ENV NVM_DIR /usr/local/nvm
ENV NODE_VERSION 22.3.0

# install nvm
# https://github.com/creationix/nvm#install-script
RUN curl --silent -o- https://raw.githubusercontent.com/creationix/nvm/v0.31.2/install.sh | bash

# # install node and npm
RUN source $NVM_DIR/nvm.sh \
    && nvm install $NODE_VERSION \
    && nvm alias default $NODE_VERSION \
    && nvm use default

# # add node and npm to path so the commands are available
ENV NODE_PATH $NVM_DIR/v$NODE_VERSION/lib/node_modules
ENV PATH $NVM_DIR/versions/node/v$NODE_VERSION/bin:$PATH

ENTRYPOINT cd /app && npm ci && cargo tauri build -t $TARGET
# ENTRYPOINT ["tail", "-f", "/dev/null"]