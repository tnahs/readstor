# -*- mode: python ; coding: utf-8 -*-

block_cipher = None

a = Analysis(
    ["src/run.py"],
    pathex=["src"],
    binaries=[],
    # https://pyinstaller.readthedocs.io/en/stable/spec-files.html#adding-data-files
    # datas=[("/path/to/file", "/path/in/bundle"),...]
    datas=[("src/resources", "resources"), ("src/data", "data"),],
    hiddenimports=[],
    hookspath=[],
    runtime_hooks=[],
    excludes=[],
    win_no_prefer_redirects=False,
    win_private_assemblies=False,
    cipher=block_cipher,
    noarchive=False,
)

pyz = PYZ(a.pure, a.zipped_data, cipher=block_cipher)

exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.zipfiles,
    a.datas,
    [],
    name="ReadStor",
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    upx_exclude=[],
    runtime_tmpdir=None,
    console=False,
)

app = BUNDLE(
    exe,
    name="ReadStor.app",
    icon="src/resources/icon.icns",
    bundle_identifier="com.shant.ReadStor",
    info_plist={
        # Enables Retina display mode.
        # https://pyinstaller.readthedocs.io/en/stable/spec-files.html#spec-file-options-for-a-mac-os-x-bundle
        "NSPrincipalClass": "NSApplication",
        "NSHighResolutionCapable": True,
        # Allows Dark Mode.
        # https://stackoverflow.com/a/56285365
        # This causes the `Preferences` window to be blank.
        # https://github.com/pyinstaller/pyinstaller/issues/4627
        # https://github.com/ronaldoussoren/py2app/issues/271
        # https://github.com/gridsync/gridsync/issues/267#issuecomment-609980411
        # "NSRequiresAquaSystemAppearance": False,
    },
)
