# mini

A Chip8 emulator written in rust that has a web and normal desktop frontend!

## Overview

This is a simple implentation of the famous chip8 for putting in my first steps into emulation(kinda). We have a backend written in rust, desktop frontend and finnaly a web frontend written using rust, webassembly and javascript

I recommend using it from it's web frontend where you have to pick a chip8 game you wanna play!

## KEYBINDINGS

| keyboard | chip8 |
| -------- | ----- |
| 1        | 1     |
| 2        | 2     |
| 3        | 3     |
| 4        | C     |
| Q        | 4     |
| W        | 5     |
| E        | 6     |
| R        | D     |
| A        | 7     |
| S        | 8     |
| D        | 9     |
| F        | E     |
| Z        | A     |
| X        | 0     |
| C        | B     |
| V        | F     |

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

Open your web browser and go to `http://localhost:8000/wasm/web/`

## Reviewer?

Go to this website and try the project(yea, yea dont thank me :): [Mini emulator!](https://lxsh-s.github.io/mini/wasm/web/)

Also, I recommend you to play `UFO` as it is really easy to play!(It's in the repo inside file named as `games_chip8`)

## AI declaration

AI was used in commit: `4fa3bff73a136feee68c0ba6a52b1d48c60655b4`, I couldnt figure out why I was not able to get the keyboard input. Lost all my patience here: `24384b09c6d26499f40ec21f348d3359b6c48999`
