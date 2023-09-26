from fat_macho import FatWriter


writer = FatWriter()
root_dir = "../target/"
dst = f"{root_dir}aarch64-apple-darwin/release/bundle/osx/Cacophony.app/Contents/MacOS/cacophony"
with open(dst, "rb") as f:
    writer.add(f.read())
with open(f"{root_dir}x86_64-apple-darwin/release/cacophony", "rb") as f:
    writer.add(f.read())
writer.generate()
writer.write_to(dst)
