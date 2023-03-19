FROM ryusei1068/rust_ubuntu:latest as build

# build for sever side 
RUN USER=root cargo new --bin server
WORKDIR /server
COPY ./server/Cargo* ./
COPY ./server/src ./src
RUN cargo build --release

# build for client side
RUN USER=root cargo new --bin client
WORKDIR /client
COPY ./client/Cargo* ./
COPY ./client/src ./src
RUN cargo build --release

FROM ubuntu:22.04 
COPY --from=build /server/target/release/server ./
COPY --from=build /client/target/release/client ./

COPY ./local-chat.sh ./
RUN chmod 755 local-chat.sh

ENTRYPOINT ./local-chat.sh 
