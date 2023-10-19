# last_engine

`last_engine` is not quite an engine but a framework to implement 3D-Applications in. 
It has features like a game loop with fixed update time and a variable draw time. 
It uses an Immediate Mode GUI (`Dear ImGui`) and OpenGL for rendering, for windowing `sdl2` is used.
Scenes are described with an ECS like structure called `chained_component_system`. 
