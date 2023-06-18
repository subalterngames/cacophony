import re
from json import loads
from pathlib import Path

"""
Bump the version of all crates and version string(s).
"""

version = "0.1.0"

# Cargo.toml versions.
members = loads(re.search(r"(members(.*?)])", Path("../Cargo.toml").read_text(encoding="utf-8")).group(0).split("=")[1])
for member in members:
    path = Path(f"../{member}/Cargo.toml").resolve()
    text = path.read_text(encoding="utf-8")
    if version in text:
        print(f"Warning! The version is already {version} in {path}")
    text = re.sub(r'(name\s*=\s*"' + member + r'"\nversion\s*=\s*)"(.*?)"', r'\1' + f'"{version}"', text)
    path.write_text(text)

# Version string in common/lib.rs
path = Path("../common//src/lib.rs").resolve()
text = path.read_text(encoding="utf-8")
text = re.sub(r'(pub const VERSION: &str)\s*=\s*"(.*?)"', r'\1 = ' + f'"{version}"', text)
path.write_text(text)
