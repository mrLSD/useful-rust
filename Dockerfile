FROM ubuntu:latest
MAINTAINER Evgeny Ukhanov <mrlsd@ya.ru>
RUN apt-get update && apt-get install -y \
    git \
    su \
    lsb \
    gcc \
    curl \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir /rs
RUN curl -s https://static.rust-lang.org/rustup.sh | sh
WORKDIR /rs
ENTRYPOINT ["bash"]
CMD ["-l"]