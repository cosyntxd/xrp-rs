import os
import json
import subprocess

rust_library_path = os.path.expanduser('~/.cargo/git/checkouts/robot.rs-83220dfd536c5f2a')
if not os.path.exists(rust_library_path):
    exit('Unable to setup the network table library because it is not downloaded, please install the cargo libraries')

env_git_path = 'WPI_LIB_GIT_INSTALL_PATH'
# I asked Claude to make me common paths
possible_paths = [
    os.path.expanduser("~/wpilib/2025"),
    "C:/Users/Public/wpilib/2025",

    os.path.expanduser("~/wpilib/2024"),
    "C:/Users/Public/wpilib/2024",

    "/usr/local/wpilib",
    "/opt/wpilib",
]
if env_git_path in os.environ:
    if not os.path.exists(os.environ(env_git_path)):
        exit(f'{env_git_path} is provided but is not a valid directory')
    else:
        install_path = os.environ(env_git_path)
elif False:
    pass
else:
# if not 'WPI_LIB_Git_INSTALL_PATH' in os.environ:
    user_input = input("would you like to download and compile wpilib here, this will take a few hours (y/n): ")
    if user_input.lower().strip() != "Y":
        exit()

    repo_url = 'https://github.com/wpilibsuite/allwpilib'
    subprocess.run(['git', 'clone', repo_url])

    #export JAVA_HOME=/usr/local/opt/openjdk@17/libexec/openjdk.jdk/Contents/Home
    #export PATH=$JAVA_HOME/bin:$PATH
    # run ./gradlew build --build-cache
    # ./gradlew clean build --build-cache --parallel --configure-on-demand --no-scan
    # nice -n -20
    install_path = "/allwpilib"


libraries = ["ntcore", "wpiHal", "wpimath", "wpinet", "wpiutil"]
source = install_path
desstination = rust_library_path

for lib in libraries:
    pass
