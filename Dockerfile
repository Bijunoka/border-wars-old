FROM rust:latest

ADD . /project/
WORKDIR /project
RUN apt-get update && apt-get install libasound2-dev libudev-dev -y
WORKDIR /project/border-wars
RUN cargo build --release
RUN cp target/release/server ../.
EXPOSE 8080
WORKDIR /project
ENTRYPOINT ["./server"]
