# Experimenting with rust + Python

## Quick start

1. Set up the virtual environment:

```sh
python3 -m venv venv
sourvce venv/bin/activate
python -m pip install -r requirements.txt
```

2. Build the Python extension module

```sh
maturin develop
```

3. Run the tests

```sh
python test.py
```
