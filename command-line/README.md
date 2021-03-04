# About

Rust is a excelent choice for Embedded Systems, it's a compilled language and you have control over memory and performance.
 
We can run a really simple application just to see the binary working on any Torizon installation. You can run it directly from Docker Hub for armv7 and armv8: 🐋

```
docker run reininy/rust-torizon
```

# Building the image

A Docker Container to cross compile your Rust applications:

```
ARG IMAGE_ARCH=armv7-unknown-linux-gnueabihf
# For arm64 use:
# ARG IMAGE_ARCH=aarch64-unknown-linux-gnu
ARG PKG_ARCH=armhf
# For arm64 use:
# ARG PKG_ARCH=aarch64
ARG TARGET_IMAGE=arm32v7-debian-base
# For arm64 use:
# ARG PKG_ARCH=arm64v8-debian-base
FROM rustembedded/cross:${IMAGE_ARCH} as build

ARG PKG_ARCH
ARG IMAGE_ARCH
ARG TARGET_IMAGE

ENV CARGO_HOME=/cargo
ENV PATH=/cargo/bin:$PATH
ENV PKG_CONFIG_PATH="/usr/lib/${IMAGE_ARCH}/pkgconfig"
ENV PKG_CONFIG_ALLOW_CROSS="true"
ENV USER=root

RUN dpkg --add-architecture ${PKG_ARCH} && \
    apt-get update && \
    apt-get install 

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /usr/local/bin/rustup.sh && \
bash /usr/local/bin/rustup.sh -y && \
rustup default stable && \
rustup target add x86_64-unknown-linux-gnu && \
rustup target add ${IMAGE_ARCH}

COPY hello/ .
RUN cargo build --target=${IMAGE_ARCH} --release

FROM torizon/$TARGET_IMAGE
COPY --from=build /target/a*/release/hello /usr/bin/

CMD /usr/bin/hello
```

On the first stage of the image, that starts from `FROM rustembedded/cross:${IMAGE_ARCH} as build`  , we are setting the necessaries enviroment variables for the cross compilation, adding the PKG_ARCH architecture and of course, installing Rust.



