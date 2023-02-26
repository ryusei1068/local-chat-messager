FROM ryusei1068/rust_ubuntu:latest as build

# build for sever program
RUN USER=root cargo new --bin server
WORKDIR /server
COPY ./serverCargo* ./
COPY ./server/src ./src
RUN cargo build --release

# build for client program
RUN USER=root cargo new --bin client
WORKDIR /client
COPY ./client/Cargo* ./
COPY ./client/src ./src
RUN cargo build --release

FROM ryusei1068/rust_ubuntu:latest
COPY --from=build /server/target/release/server ./
COPY --from=build /client/target/release/client ./

#COPY ./local-chat.sh ./
#RUN chmod 755 local-chat.sh
#
#CMD ./local-chat.sh
