What is the design process for a bevy app? This is mostly a tech demo. I need a dozen tech demos to attach to make a game. Start with acheivable goals. I need a checklist

1. Think up what you want the game to have. This should probably be a very minimal game to start, but if you have ideas you should write them.

2. Then figure out how to translate that into bevy components, resources, events, and systems (and maybe other things. i need to check the docs for all the derive macros. jeez this is complex).

3. Then pick what sorts of third party libraries to use. For example, you might use an existing physics library like rapier or avian. For input, you might choose between leafwing or bevy_enhanced_input. I need descriptions for each to know why I might choose one over the other. I think its a good idea to prioritize popular libraries (lots of github stars/contributors/forks) over new ones unless there is a strong reason; its great to have other people find and solve bugs. It's necessary as a one developer design shop. Since 

====

# Random Inspirations

Vampire survivors brought us the low input game. just joystick. you don't need a bunch of button pressing because theres pretty much just the joystick and an accept button.

I present, the no input game. I just need a place to chill. I don't want my computer to work like everyone else's. I want to build something weird.

My goal is to build an app that runs for 100% of the time on my computer. It will add a peaceful backdrop to whatever I'm doing. 

# 1

## what should the game have?

- [ ] No input.
- [ ] spatial 3d audio
- [ ] multiple birds flying in a set order between nearby locations.



Just a peaceful space to sit and listen to birds from anywhere in the world.


#  what should the game have eventually, but doesn't need right now?

- simplest ai for the birds. fly between random trees. after 3 trees, fly off screen, fade the audio, and despawn.

- next level bird ai. choose between finding food or water or a quiet spot to chirp

- next level bird ai. flocking movement where birds will move together. move away from birds of other types at a far distance. move away from birds of the same type at a small distance. follow a randomly chosen leader. 

- Accurate simulation of different types of birds. actually read some academic papers.

- Gameplay. I just want something pretty to look at right now.

- Configurable controls from a config file

- Addons (written in lua? or a wasm sandbox?)

- Huge maps with [big_space](https://docs.rs/big_space/latest/big_space/)

- Multiplayer

- Procedurally generated maps

- User modifications to maps

- User generated maps

- grass simulations

- bugs. invisible directly to the player, but will impact the 

- plant growth

- smells. invisible to the player, but animals will interact with them.

- ai should react to sounds

- SDF? This is some awesome math but its a lot to think about. maybe thats another tech demo. a pallete of different shapes. chose level and addition mode and scale 
