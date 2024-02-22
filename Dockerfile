FROM rust:alpine3.19

RUN apk add openssh
RUN apk add pkgconfig
RUN apk add alpine-sdk openssl-dev
RUN mkdir -p /app
WORKDIR /app
RUN mkdir -p target/
RUN mkdir -p config/
RUN apk add --no-cache musl-dev


COPY . /app	
RUN cargo fetch

RUN cargo build
CMD [ "cargo ", "run"]
