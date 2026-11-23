# mini

A Chip8 emulator written in rust that has a web and normal desktop frontend!

## Overview

This is a simple implentation of the famous chip8 for putting in my first steps into emulation(kinda). We have a backend written in rust, desktop frontend and finnaly a web frontend written using rust, webassembly and javascript

## Installtion (debug, not for reviewers)

Clone the repo

```
git clone https://github.com/lxsh-S/mini.git
cd mini/
```

Rebuild

```
cd wasm
wasm-pack build --target web
```

Host it locally using python

```
cd ~/dev/mini/ # Will definetly differ for you (go to project root)
python -m http.server 8000
```

Open your web browser and go to `http://localhost:8000`

## Reviewer?

Go to this website and try the project(yea, yea dont thank me :) : ``
