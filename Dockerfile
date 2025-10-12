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

COPY pyproject.toml test.py /app/
COPY client /app/client
COPY evolutionary /app/client
COPY data /app/client


RUN cd /app
RUN python3 -m pip install uv
RUN python3 -m uv sync
RUN cargo build --release --manifest-path evolutionary/Cargo.toml
RUN python3 -m maturin develop --release --manifest-path evolutionary/Cargo.toml
RUN python3 -m uv pip install -n ./evolutionary
RUN python3 -m uv run streamlit run client/streamlit_app.py

EXPOSE 8080

CMD [ "streamlit", "run", "client/streamlit_app.py" ]