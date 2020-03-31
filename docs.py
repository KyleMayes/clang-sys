#!/usr/bin/env python3

import os
import shutil
import subprocess

TARGET = ".docs"
VERSION = "clang_9_0"

if os.path.isdir(TARGET):
    shutil.rmtree(TARGET)

os.mkdir(TARGET)

for (name, features) in [("default", VERSION), ("runtime", f"runtime,{VERSION}")]:
    subprocess.call(["cargo", "clean"])
    subprocess.call(["cargo", "doc", f"--features={features}", "--no-deps"])
    print(f"Copying docs to {TARGET}/{name}...")
    shutil.copytree(f"target/doc", f"{TARGET}/{name}")

os.chdir(TARGET)
subprocess.call(["git", "init"])
subprocess.call(["git", "remote", "add", "origin", "git@github.com:KyleMayes/clang-sys.git"])
subprocess.call(["git", "checkout", "--orphan", "gh-pages"])
subprocess.call(["git", "add", "-A"])
subprocess.call(["git", "commit", "-m", "\"Update documentation\""])
subprocess.call(["git", "push", "origin", "gh-pages", "--force"])

os.chdir("..")
shutil.rmtree(TARGET)
