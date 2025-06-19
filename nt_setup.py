import os
import time
import shutil
import zipfile
import platform
import subprocess
from pathlib import Path

start_time = time.time()

if not shutil.which("cargo"):
    exit("Rust/Cargo is not properly installed on this system")


rust_library_path = os.path.expanduser('~/.cargo/git/checkouts/robot.rs-83220dfd536c5f2a/7b4c487')


lock_file = os.path.join(rust_library_path, "wpilib-hal", "Cargo.lock")
if os.path.exists(lock_file):
    os.remove(lock_file)

xrp_library_path = os.path.join(os.path.dirname(__file__), "library")
cargo_fetch_result = subprocess.run(
    ["cargo", "fetch"],
    capture_output=True,
    cwd=xrp_library_path,
    text=True,
    check=False
)

if not os.path.exists(rust_library_path):
    exit("Unable to find the rust network table library, please make sure `cargo fetch` works")

if cargo_fetch_result.stderr:
    exit(cargo_fetch_result.stderr)


rust_libs = os.path.join(rust_library_path, "wpilib-hal", "libs")
rust_inc = os.path.join(rust_library_path, "wpilib-hal", "include")

shutil.rmtree(rust_libs, ignore_errors=True)
shutil.rmtree(rust_inc, ignore_errors=True)

os.makedirs(rust_libs)
os.makedirs(rust_inc)

rust_inc = os.path.join(rust_library_path, "wpilib-hal", "include")

if not os.path.exists(rust_libs) or not os.path.exists(rust_inc):
    exit("failed to clear the include and libs directory")


possible_paths = [
    os.path.expanduser("~/wpilib/2025/maven/edu/wpi/first/"),
    "C:/Users/Public/wpilib/2025/maven/edu/wpi/first",

    "/usr/local/wpilib/maven/edu/wpi/first",
    "/opt/wpilib/maven/edu/wpi/first",

    os.path.expanduser("~/wpilib/2024/maven/edu/wpi/first"),
    "C:/Users/Public/wpilib/2024/maven/edu/wpi/first",
]

wpilib = None
for path in possible_paths:
    if os.path.exists(path):
        wpilib = path
        break

if wpilib == None:
    exit("wpilib not detected on your system")


file_ext = [".dll", ".lib", ".dylib", ".a", "lib", "so", "h", "c", "cpp"]
system = platform.system().lower()
if system == 'windows': # spyware
    platform_suffix = 'windowsx86-64'
elif system == 'darwin':  # macOS' 
    platform_suffix = 'osxuniversal' # osxx86-64
else:  # Linux and other Unix-like
    platform_suffix = 'linuxx86-64'

libraries = ["ntcore", "hal", "wpimath", "wpinet", "wpiutil"]

print(f"Copying libraries from: {wpilib}")
print(f"Libraries destination: {rust_libs}")
print(f"Headers destination: {rust_inc}")
print(f"Target platform: {platform_suffix}")
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
                if any([member.endswith(ext) for ext in file_ext]):
                    
                    filename = os.path.basename(member)
                    target_path = os.path.join(destination, filename)

                    with zip_ref.open(member) as source, open(target_path, 'wb') as target:
                        target.write(source.read())

    # headers like their folders
    with zipfile.ZipFile(headers, 'r') as zip_ref:
        zip_ref.extractall(rust_inc)
    # libraries do not
    extract_dylibs(libs, rust_libs)

print("\nPatching library")

try:
    result = subprocess.run(
        ['git', '--version'], 
        capture_output=True, 
        text=True, 
        check=True
    )
except subprocess.CalledProcessError:
    exit("`git --version` failed")
except FileNotFoundError:
    exit("git is not installed or not in PATH")

original_cwd = os.getcwd()
current_directory = os.path.dirname(os.path.abspath(__file__))
patch_file = os.path.join(current_directory, "nt_fix.patch")

# git diff -b upstream/master > /Users/ryan/Github/xrp-rs/nt_fix.patch
try:
    patch_result = subprocess.run(
        ['git', 'apply', patch_file], 
        cwd=rust_library_path
    )

except subprocess.CalledProcessError as e:
    exit(f"Failed to apply patches: {e.stderr}")

# git bitching about this patch but python would never hurt me <3 ily
# also rust likes to be funny and switch to a broken version because theres no Cargo.lock to stop it
cargo_toml_path = os.path.join(rust_library_path, "wpilib-hal", "Cargo.toml")
with open(cargo_toml_path, 'r') as f:
    content = f.read()
updated_content = content.replace('bindgen = \"0.53.1\"', 'bindgen = \"=0.66.1\"')
with open(cargo_toml_path, 'w') as f:
    f.write(updated_content)

print("\nClearing existing bindings")

current_directory = os.path.dirname(os.path.abspath(__file__))
debug_rust_deps = os.path.join(current_directory, "library", "target", "debug", "build")
for dependency in os.listdir(debug_rust_deps):
    if "wpilib-hal" in dependency:
        wpilib_dependency = os.path.join(debug_rust_deps, dependency)
        shutil.rmtree(wpilib_dependency, ignore_errors=True)

elapsed = time.time() - start_time
print(f"\nSucessfully executed in {elapsed:.2f}s")
