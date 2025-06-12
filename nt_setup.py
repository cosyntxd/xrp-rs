import os
import json
import glob
import zipfile
import platform
import subprocess

rust_library_path = os.path.expanduser('~/.cargo/git/checkouts/robot.rs-83220dfd536c5f2a/7b4c487')
if not os.path.exists(rust_library_path):
    exit('Unable to setup the network table library because it is not downloaded, please install the cargo libraries')

possible_paths = [
    os.path.expanduser("~/wpilib/2025/maven/edu/wpi/first/"),
    "C:/Users/Public/wpilib/2025/maven/edu/wpi/first",

    "/usr/local/wpilib/maven/edu/wpi/first",
    "/opt/wpilib/maven/edu/wpi/first",

    os.path.expanduser("~/wpilib/2024/maven/edu/wpi/first"),
    "C:/Users/Public/wpilib/2024/maven/edu/wpi/first",

]


for path in possible_paths:
    if os.path.exists(path):
        wpilib = path
        break

if wpilib == None:
    exit("wpilib not detected on your system")

system = platform.system().lower()
if system == 'windows':
    lib_ext = '.dll'
    static_ext = '.lib'
    lib_prefix = ''
    platform_suffix = 'windowsx86-64'
elif system == 'darwin':  # macOS
    lib_ext = '.dylib'
    static_ext = '.a'
    lib_prefix = 'lib' 
    platform_suffix = 'osxuniversal' # osxuniversal

    # platform_suffix = 'osxx86-64' # osxuniversal
else:  # Linux and other Unix-like
    lib_ext = '.so'
    static_ext = '.a'
    lib_prefix = 'lib'
    platform_suffix = 'linuxx86-64'


libraries = ["ntcore", "hal", "wpimath", "wpinet", "wpiutil"]

libs_destination = os.path.join(rust_library_path, 'wpilib-hal', 'libs')
include_destination = os.path.join(rust_library_path, 'wpilib-hal', 'include')

print(f"Copying libraries from: {wpilib}")
print(f"Target platform: {platform_suffix}")
print(f"Libraries destination: {libs_destination}")
print(f"Headers destination: {include_destination}")
print()


for lib in libraries:
    print(f"Processing {lib}")
    lib_path = os.path.join(wpilib, lib, f"{lib}-cpp")
    if not os.path.exists(lib_path):    
        exit(f"wpilib wpilib does not have {lib} library")
    
    versions = []
    for lib_version in os.listdir(lib_path):
        lib_path_versioned = os.path.join(lib_path, lib_version)
        
        if os.path.isfile(lib_path_versioned):
            continue
        year, major, _ = lib_version.split('.')
        version_num = int(year) * 1000 + int(major)
        versions.append((version_num, lib_version))

    versions.sort(key=lambda x: x[1], reverse=True)
    newest_path = os.path.join(lib_path, versions[0][1])

    headers = os.path.join(newest_path, f"{lib}-cpp-{versions[0][1]}-headers.zip")
    libs = os.path.join(newest_path, f"{lib}-cpp-{versions[0][1]}-{platform_suffix}.zip")
    
    def extract_dylibs(zip_path, destination):
        with zipfile.ZipFile(zip_path, 'r') as zip_ref:
            for member in zip_ref.namelist():
                print(member)
                if (member.endswith('.dylib') or member.endswith('.h') or member.endswith('.inc')) and not member.endswith('/'):
                    filename = os.path.basename(member)
                    target_path = os.path.join(destination, filename)

                    with zip_ref.open(member) as source, open(target_path, 'wb') as target:
                        target.write(source.read())

    # extract_dylibs(headers, include_destination)
    extract_dylibs(libs, libs_destination)
        
    with zipfile.ZipFile(headers, 'r') as zip_ref:
        zip_ref.extractall(include_destination)

    # with zipfile.ZipFile(libs, 'r') as zip_ref:
    #     zip_ref.extractall(libs_destination)
# use 0.72
# .whitelist_type from .allowlist_type for everything

"""
import os
import json
import glob
import shutil
import zipfile
import platform
import subprocess
from pathlib import Path

# ~/.cargo/git/checkouts/robot.rs-83220dfd536c5f2a/7b4c487
rust_library_path = Path.home() / ".cargo" / "git" / "checkouts" / "robot.rs-83220dfd536c5f2a" / "7b4c487"
rust_libs = rust_library_path / "wpilib-hal" / "libs"
rust_inc = rust_library_path / "wpilib-hal" / "include"

if not os.path.exists(rust_library_path):
    if not os.path.exists(Path.home() / ".cargo"):
        exit("You have not even installed rust yet")
    else:
        exit('Unable to setup the network table library because it is not downloaded, please install the cargo libraries')

shutil.rmtree(rust_libs)
shutil.rmtree(rust_inc)

rust_libs.mkdir()
rust_inc.mkdir()



"""