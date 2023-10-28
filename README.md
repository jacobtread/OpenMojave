# OpenMojave

> VERY EARLY STAGES

This is intended to be an open-source re-implementation of Fallout: New Vegas that is capable of running and using the original game assets (You'd have to get these yourself of course) 

I'm creating this in order to allow the game to take advantage of newer hardware as the game is still a 32bit game and is slowly running into issues with newer hardware as it ages.

I am writing this using *Rust* and the [Fyrox](https://github.com/FyroxEngine/Fyro) game engine, I intend for it to be a semi drop in replacement for the original game *give or take*


I've started working on getting the basic menu working which you can see below:

![Demo menu](images/menu.png)





## Whats done

- [x] Asset loading
  - .bsa packed asset format can be loaded by the engine and the resources within can be used
- [x] Font loading and rendering
  - The .fnt bitmap fonts for the game can be loaded and used in the UI

## Whats not done

- [ ] The game