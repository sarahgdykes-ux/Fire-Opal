# Fire Opal Game Engine

A custom 2D game engine built from scratch in Rust, designed for learning and creating complex 2D games (RPGs, strategy games).

## Architecture

The engine is built with a modular crate structure:

- **engine-core**: Core math library (Vec2, Mat3, Rect, Color), time management, error types
- **engine-ecs**: Entity Component System with entity pool, component storage, system scheduling, and world container
- **engine-graphics**: Software rasterizer, sprite system, camera system, texture loading (PNG support), and animation system
- **engine-physics**: Rigid body dynamics, AABB colliders, collision detection/resolution, and spatial hash broadphase
- **engine-input**: Keyboard, mouse, and input mapping system
- **engine-audio**: Sound loading and playback using rodio (temporarily disabled due to build issues)
- **engine-ui**: UI elements (buttons, labels, panels) with layout and interaction
- **engine**: Main engine loop with winit window integration

## Features

### Core Systems
- **ECS Architecture**: Flexible entity-component-system for game object composition
- **Software Rasterizer**: CPU-based rendering with pixel/line/triangle drawing to in-memory framebuffer
- **Physics Engine**: AABB collision detection, impulse-based resolution, spatial partitioning
- **Input System**: Keyboard, mouse, and action mapping with deadzone support
- **Camera System**: 2D camera with position, zoom, rotation, and world-to-screen transforms
- **UI System**: Buttons, labels, panels with hover/click states and z-ordering
- **Animation System**: Frame-based animations with play modes (once, loop, ping-pong)

### Graphics
- Software rasterizer rendering to in-memory framebuffer
- Texture loading (PNG)
- Sprite batching with z-ordering
- View culling
- Frame-based animations
- Animation play modes (Once, Loop, PingPong)

The software rasterizer draws to an in-memory framebuffer, which the engine copies to the window each frame using [softbuffer](https://crates.io/crates/softbuffer). The framebuffer resizes with the window.

### Physics
- Static, dynamic, and kinematic bodies
- AABB collision detection
- Collision resolution with restitution and friction
- Spatial hash grid for broadphase optimization

### Input
- Keyboard state tracking (pressed, just_pressed, just_released)
- Mouse tracking (position, buttons, scroll)
- Action mapping (keys/buttons to game actions)
- Axis mapping (WASD to movement vectors)

### UI
- UI elements: Panels, Buttons, Labels
- Hover and click state detection
- Z-ordering for layered UI
- Visibility and enabled states
- UI manager for element lifecycle

## Building

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Run the platformer example
cargo run --package platformer
```

**Note**: If you encounter Windows file lock errors during build, try:
```bash
cargo clean
cargo build --package platformer --jobs 1
```

## Current Status

The engine is functionally complete with:
- ✅ Full ECS system
- ✅ Software rasterizer (renders to memory)
- ✅ Physics engine with collision
- ✅ Input system (keyboard, mouse, action mapping)
- ✅ UI system (buttons, labels, panels)
- ✅ Animation system
- ✅ Main game loop with winit (variable `update` + fixed-timestep `fixed_update`)
- ✅ Window display via softbuffer

Not done yet: the camera is not applied by the renderer, and there is no text rendering (labels draw as plain rectangles).

## Example Game

A simple platformer example is included in `examples/platformer`. It demonstrates:
- Player movement with WASD
- Jumping with spacebar or UI button
- Gravity and floor collision
- Camera following the player
- UI system with score display and jump button

The example opens a window showing the player, floor and UI; the FPS counter is in the window title.

## Design Philosophy

- **From-scratch implementation**: Core engine logic built from first principles for learning
- **Data-oriented design**: ECS architecture for cache-friendly performance
- **Modular structure**: Independent subsystems that can be used separately
- **Cross-platform**: Uses winit for window management (Windows, Linux, macOS, Web via WASM)

## Future Enhancements

- Apply the camera transform in the renderer
- Text rendering (bitmap font)
- Re-enable audio system (currently disabled due to Windows build issues)
- Hardware-accelerated rendering (wgpu migration)
- More physics features (rotation, polygons, continuous collision detection)
- Scene management
- Particle system
- Tilemap renderer
- Asset hot-reloading
- More UI elements (sliders, text inputs, scroll views)
- Advanced animation features (blending, easing functions)

## License

MIT
