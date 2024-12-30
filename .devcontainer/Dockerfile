FROM mcr.microsoft.com/devcontainers/rust:latest

# 安装依赖
RUN apt-get update && apt-get install -y \
    libx11-dev \
    libxi6 \
    libxi-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxinerama-dev \
    libgl1-mesa-dev \
    libgl1-mesa-glx \
    libglu1-mesa-dev \
    libxkbcommon0 \
    libxkbcommon-x11-0 \
    x11-xserver-utils \
    mesa-utils \
    xvfb \
    pkg-config \
    libglx-mesa0 \
    libgl1 \
    libglvnd0 \
    libglx0 \
    libegl1 \
    libgles2 \
    mesa-utils-extra

RUN mkdir -p /usr/share/glvnd/egl_vendor.d && \
    echo '{"file_format_version": "1.0.0", "ICD": {"library_path": "libEGL_mesa.so.0"}}' > /usr/share/glvnd/egl_vendor.d/50_mesa.json

RUN echo '#!/bin/bash\n\
export LIBGL_ALWAYS_SOFTWARE=1\n\
export MESA_GL_VERSION_OVERRIDE=3.0\n\
export GALLIUM_DRIVER=llvmpipe\n\
Xvfb :99 -screen 0 1024x768x24 -ac +extension GLX +render -noreset &\n\
sleep 2\n\
exec "$@"' > /usr/local/bin/start-xvfb.sh \
    && chmod +x /usr/local/bin/start-xvfb.sh

ENV DISPLAY=:99
ENV LIBGL_ALWAYS_SOFTWARE=1
ENV MESA_GL_VERSION_OVERRIDE=3.0
ENV GALLIUM_DRIVER=llvmpipe