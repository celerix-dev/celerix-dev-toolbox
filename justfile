#!/usr/bin/env just --justfile
#set windows-shell := ["pwsh.exe", "-NoLogo","-Command"]
set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-cu"]

default:
    @just --list

# start tauri dev mode
dev:
    npm run tauri dev

# clean
clean:
    npm run tauri clean

# build universal macos package
build-universal:
    npm run build-universal