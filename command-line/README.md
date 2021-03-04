# About

We can run a really simple application just to see the binary working on any Torizon installation. You can run it directly from Docker Hub for armv7 and armv8: 🐋

```
# docker run reininy/rust-torizon
```

# Building the image

Running for armv7:

```
# docker build .
```

Running for armv8:

```
# docker build --build-arg IMAGE_ARCH=aarch64-unknown-linux-gnu --build-arg PKG_ARCH=aarch64 --build-arg TARGET_IMAGE=arm64v8-debian-base
```



