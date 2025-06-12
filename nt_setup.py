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

    os.path.expanduser("~/wpilib/2024/maven/edu/wpi/first"),
    "C:/Users/Public/wpilib/2024/maven/edu/wpi/first",

    "/usr/local/wpilib/maven/edu/wpi/first",
    "/opt/wpilib/maven/edu/wpi/first",
]

wpilib = None
for path in possible_paths:
    if os.path.exists(path):
        wpilib = path
        break

if wpilib is None:
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
    platform_suffix = 'osxuniversal'
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
    
    with zipfile.ZipFile(headers, 'r') as zip_ref:
        zip_ref.extractall(include_destination)

    with zipfile.ZipFile(libs, 'r') as zip_ref:
        zip_ref.extractall(libs_destination)