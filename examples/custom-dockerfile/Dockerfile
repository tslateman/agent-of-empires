# Custom sandbox image
#
# Extends the base sandbox with project-specific tools.
# Build: docker build -t my-sandbox:latest examples/custom-dockerfile/
# Use:   aoe add --sandbox-image my-sandbox:latest .

FROM ghcr.io/tslateman/aoe-sandbox:lite

# System packages
RUN apt-get update && apt-get install -y \
    postgresql-client \
    redis-tools \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Python packages
RUN pip3 install --break-system-packages \
    pandas \
    requests \
    pytest

# Go (example of installing a language runtime)
RUN curl -fsSL https://go.dev/dl/go1.23.0.linux-amd64.tar.gz \
    | tar -C /usr/local -xzf - \
    && echo 'export PATH=$PATH:/usr/local/go/bin' >> /root/.bashrc
