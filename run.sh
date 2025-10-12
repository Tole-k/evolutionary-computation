deactivate
uv sync -n
source .venv/bin/activate
uv pip uninstall evolutionary
maturin develop --release --manifest-path evolutionary/Cargo.toml
uv pip install -n ./evolutionary
uv run streamlit run client/streamlit_app.py