```shell
docker run --rm -it --executable=bash -v $(pwd):/io ghcr.io/pyo3/maturin
rm -rf target
cd modules/py
maturin publish -i 3.7 -i 3.8 -i 3.9 -i 3.10
```
