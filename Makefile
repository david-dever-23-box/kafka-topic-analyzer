VERSION ?= $(GIT_VERSION)

SERVICE_NAME := kafka-topic-analyzer

GIT_ROOT := github.com
GIT_ORG := david-dever-23-box
REPO := kafka-topic-analyzer

GIT_VERSION := $(shell git describe --tags)

# https://github.com/rust-lang-nursery/docker-rust/tree/master/1.25.0/stretch/Dockerfile
RUST_IMAGE_VERSION := "library/rust:1.25.0-stretch"

all: docker-pull-rust docker-build build version

build:
	cargo install

# Pull rust image for docker build
docker-pull-rust:
	docker pull $(RUST_IMAGE_VERSION)

# Build in docker
docker-build: docker-pull-rust
	docker run --rm \
		-e "USER=root" \
		-v "$$PWD/Makefile":/$(SERVICE_NAME)/Makefile \
		-v "$$PWD/src":/$(SERVICE_NAME)/src \
		-v "$$PWD/Cargo.lock":/$(SERVICE_NAME)/Cargo.lock \
		-v "$$PWD/Cargo.toml":/$(SERVICE_NAME)/Cargo.toml \
		-v "$$PWD/Cross.toml":/$(SERVICE_NAME)/Cross.toml \
		-w /$(SERVICE_NAME) \
		$(RUST_IMAGE_VERSION) \
		make build

version:
	@echo "$(VERSION)"

clean:
	@rm -rf db bin/ pkg/ tmp/ *.o *.a *.so

.PHONY: version
