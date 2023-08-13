from shutil import move
from typing import Optional
from os import chdir, getcwd, walk
from subprocess import call, check_output, CalledProcessError
from pathlib import Path
from ftplib import FTP, error_perm
import re
from github import Repository, Github


def upload_directory(ftp: FTP, folder: str = None) -> None:
    if folder is not None:
        chdir(folder)
        ftp.cwd(folder)
    for fi in Path(getcwd()).resolve().iterdir():
        if fi.is_file():
            with fi.open("rb") as bs:
                ftp.storbinary(f'STOR {fi.name}', bs)
            print(f"Uploaded: {fi.name}")

def get_latest_version() -> Optional[str]:
    try:
        resp = check_output(["git", "describe", "--tags", "--abbrev=0"])
        return str(resp).strip()
    except CalledProcessError:
        return None
    
def ftp_login() -> FTP:
    ftp = FTP("subalterngames.com")
    ftp_credentials = Path("credentials/ftp.txt").read_text().split("\n")
    ftp.login(user=ftp_credentials[0], passwd=ftp_credentials[1])
    print("Logged into FTP")
    return ftp
    

def ftp_website(ftp: FTP) -> None:
    cwd = getcwd()
    root_remote = "subalterngames.com/cacophony"
    ftp.cwd(root_remote)
    print("Set cwd")
    chdir("../html")
    upload_directory(ftp)
    upload_directory(ftp, folder="images")
    upload_directory(ftp, folder="../fonts/noto")
    print("...Done!")
    ftp.cwd("/subalterngames.com/cacophony")
    chdir(cwd)


def ftp_cwd(ftp: FTP, folder: str) -> None:
    try:
        ftp.cwd(folder)
    except error_perm:
        ftp.mkd(folder)
        ftp.cwd(folder)


def create_builds() -> None:
    # Compare versions.
    version = re.search(r'version = "(.*?)"', Path("../Cargo.toml").read_text()).group(1)
    latest_version = get_latest_version()
    if version == latest_version:
        print("Can't upload. Update the version.")
        exit()

    # Tag.
    token: str = Path("credentials/github.txt").resolve().read_text(encoding="utf-8").strip()
    repo: Repository = Github(token).get_repo("subalterngames/cacophony")
    repo.create_git_tag(tag=version, message=version, type="commit", object=repo.get_commits()[0].sha)
    print("Tagged.")

    # Build the releases.
    workflow = repo.get_workflow("build")
    workflow.create_dispatch(ref="main", inputs={"version": version})


f = ftp_login()
ftp_website(f)
f.close()
create_builds()