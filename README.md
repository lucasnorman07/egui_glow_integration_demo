# Egui demo with custom OpenGL graphics using Winit and Glow

This is a simple demo application using Winit that renders an Egui UI and a custom OpenGL cube with Glow. The UI has controls to the right for changing the cube's position, rotation and scale. It is not optimized with proper error handling, but it works :)

## Steps to run

```powershell
git clone git@github.com:lucasnorman07/egui_glow_integration_demo
cd egui_glow_integration_demo
cargo run
```

## Crates that were used:

-   winit = "0.30.9" \
    Used to create the window and event loop. \
    Usefull resources:
    -   https://docs.rs/winit/latest/

-   glutin = "0.32.0" \
    Used to create a display which is then used to create an OpenGL surface and context. \
    Usefull resources:
    -   https://docs.rs/glutin/latest/
    -   https://users.rust-lang.org/t/i-cant-find-glutin-tutorials/93482

-   egui = "0.31.1" \
    Used to construct the user interface. \
    Usefull resources:
    -   https://docs.rs/egui/latest/
    -   https://www.egui.rs/#demo

-   egui-winit = "0.31.1" \
    Used to convert winit events to egui raw events. \
    Usefull resources:
    -   https://docs.rs/egui-winit/latest/

-   egui_glow = "0.31.1" \
    Used to render the egui ui with glow (OpenGL). \
    Usefull resources:
    -   https://docs.rs/egui_glow/latest/

-   glow = "0.16.0" \
    Provides OpenGl functions pointers similar to the gl crate, but is designed specifically for Rust. \
    Usefull resources:
    -   https://docs.rs/glow/latest/

-   cgmath = "0.18.0" \
    Used to handle 3D math and matrices. \
    Usefull resources:
    -   https://docs.rs/cgmath/latest/

-   bytemuck = "1.23.0" \
    Used for type convertions to byte slices. \
    Usefull resources:
    -   https://docs.rs/bytemuck/latest/

-   image = "0.25.6" \
    Used to load images for the textures. \
    Usefull resources:
    -   https://docs.rs/image/latest/
