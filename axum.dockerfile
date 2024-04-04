# Due to lack of few things in alpine version, cannot use alpine here
FROM rust:1.77.1

WORKDIR /usr/src/app

COPY . .

RUN cargo build 

CMD [ "cargo", "run" ]