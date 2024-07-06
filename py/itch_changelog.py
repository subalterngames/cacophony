from pathlib import Path
import re

version = re.search(r'\[workspace.package]\nversion = "(.*?)"$', Path("../Cargo.toml").read_text(), flags=re.MULTILINE).group(1)
changelog = re.search(f'## {version}' + r'((.|\n)*?)##', Path("../changelog.md").read_text(), flags=re.MULTILINE).group(1).strip()
itch = re.sub(r'- (.*?)$', r'<li>\1</li>', changelog, flags=re.MULTILINE)
print(f"<ul>\n{itch}\n</ul>")