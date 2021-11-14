# Falling Rust

Falling-sand game written in Rust, Bevy and egui.

Simulating simplified physics. 

## Features
- Simulation of many different elements: sand, rock, wood, water, acid, oil, lava, fire, ash, smoke
- Editor with several tools: place circle, square or spray
- 512 x 512 cells sandbox to play around with
- Place a source for any element
- Liquid drains
- Pause and step the simulation
- Clear the level

![Screenshot](images/screenshot.png)

## How does it work
Cellular automata.

Sand falls down and slides diagonally.

![Falling sand](images/falling-sand.gif)

Water falls down, diagonally but also sideways.

![Falling water](images/falling-water.gif)

Fire moves in all directions with a tendency upwards. It turns burnable elements into fire. Burns out over time.

![Burning fire](images/burning-fire.gif)

