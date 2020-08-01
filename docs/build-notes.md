https://github.com/beeware/briefcase/issues/66

`PyInstaller` requires CPython installation built with `--enable-framework`. To install with `pyenv`:
``` bash
env PYTHON_CONFIGURE_OPTS="--enable-framework" pyenv install 3.7.7
pyenv local 3.7.7
```

Additionally, `PyInstaller` has trouble building when using `poetry`. So use Python's built-in virtual environments to build.
``` bash
python -m venv venv
source venv/bin/activate
pip install psutil pyqt5 jinja2 pyinstaller
pyinstaller build.spec
```

via https://github.com/pyenv/pyenv/wiki#how-to-build-cpython-with-framework-support-on-os-x