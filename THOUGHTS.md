# Misc thoughts while working on this

no intro menu? just load right into the game. simplifies game states. if we keep game states, hitting escape should go back to the main menu

need a "settings" pane
- volume
- number of birds

I don't think we need physics yet. I don't even really want 3d. but when i read some of the things about 2d, it still had to do a lot of transformations in 3d. so I think it might be simplest to just make a top down 3d game.

player is controllable using bevy_enhanced_input

start by adding a hard coded bird sound to the spatial_audio_2d example

player needs a h