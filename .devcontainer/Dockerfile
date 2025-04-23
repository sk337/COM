# Use the official Rust devcontainer image as the base
FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# Set environment variables for non-interactive apt
ENV DEBIAN_FRONTEND=noninteractive

LABEL dev.containers.featureSet.version=1

# Update package index and install required packages
RUN apt-get update && \
	apt-get install -y --no-install-recommends \
	automake \
	gcc \
	g++ \
	make \
	libncurses-dev \
	nasm \
	libsdl-net1.2-dev \
	libsdl2-net-dev \
	libpcap-dev \
	libslirp-dev \
	fluidsynth \
	libfluidsynth-dev \
	libavdevice58 \
	libavformat-dev \
	libavcodec-dev \
	libavcodec-extra \
	libavcodec-extra58 \
	libswscale-dev \
	libfreetype-dev \
	libxkbfile-dev \
	libxrandr-dev \
	imagemagick \
	librsvg2-bin \
	icnsutils \
	&& apt-get clean \
	&& rm -rf /var/lib/apt/lists/*

# Build or update DOSBox-X
RUN if [ -d /opt/dosbox-x ]; then \
		cd /opt/dosbox-x && \
		git pull && \
		./build-debug && \
		sudo make install; \
	else \
		git clone --depth=1 https://github.com/joncampbell123/dosbox-x.git /opt/dosbox-x && \
		cd /opt/dosbox-x && \
		./build-debug && \
		sudo make install; \
	fi
