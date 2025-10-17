FROM python:3.13-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl build-essential \
 && rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y \
 && . "$HOME/.cargo/env" \
 && rustup default stable

ENV PATH="/root/.cargo/bin:${PATH}"

RUN python3 --version && cargo --version && rustc --version

WORKDIR /app

COPY pyproject.toml /app/
COPY client /app/client
COPY evolutionary /app/evolutionary
COPY data /app/data
COPY README.md /app/README.md

RUN python3 -m pip install uv
RUN uv sync
RUN cargo build --release --manifest-path evolutionary/Cargo.toml

EXPOSE 1415
CMD [ "uv", "run", "python", "-m", "streamlit", "run", "client/streamlit_app.py", "--server.port", "1415", "--server.address", "0.0.0.0"]
