#!/bin/sh

docker run -it --rm \
	-v $PWD:/workdir \
	registry.gitlab.com/rust_musl_docker/image:stable-1.39.0 \
	cargo build --release -vv --target=x86_64-unknown-linux-musl
