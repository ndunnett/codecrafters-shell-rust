FROM ubuntu:jammy
ARG DEBIAN_FRONTEND=noninteractive

# install dependencies
RUN set -eux; \
    apt-get update; \
    apt-get upgrade -y; \
    apt-get install -y --no-install-recommends \
        ca-certificates gcc libc6-dev pkg-config libssl-dev wget curl git zsh; \
    apt-get clean -y; \
    apt-get autoclean -y; \
    apt-get autoremove -y; \
    rm -rf /var/lib/apt/lists/*

# install codecrafters cli
RUN set -eux; \
    wget -qO - https://codecrafters.io/install.sh | sh

# create user
ARG USERNAME=dev
RUN set -eux; \
    useradd --create-home --user-group --no-log-init "$USERNAME"; \
    chown -R "$USERNAME:$USERNAME" "/home/$USERNAME"
USER "$USERNAME"

# install rust
RUN set -eux; \
    wget -qO - https://sh.rustup.rs | sh -s -- -y

# entrypoint
CMD ["sleep", "infinity"]
