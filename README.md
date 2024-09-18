# Raytracing Diorama

This project demonstrates a raytracing implementation, using various techniques studied in the field of computer graphics. The diorama created includes textured cubes, lighting, and other visual effects to showcase the capabilities of raytracing.

## Features

- **Textured Cubes**: Utilizes various textures, including dirt, moss, stone, and water, to build a 3D diorama.
- **Raytracing**: Implements raytracing to calculate light interaction with objects in the scene, such as reflection, refraction, and shadows.
- **Camera Movement**: Allows the user to move the camera around the diorama for different views using the keyboard.
- **Dynamic Elements**: Animated water cubes within the scene.
  
## Running the Project

To run this project with optimized performance, ensure you have [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/) installed.

1. Clone the repository:
   ```bash
   git clone https://github.com/wwIrvingww/release.git
   cd release
   ```

2. Build and run the project in release mode for improved performance:
   ```bash
   cargo run --release
   ```

3. Control the camera using the keyboard:
   - **W**: Move forward
   - **S**: Move backward
   - **A**: Move left
   - **D**: Move right
   - **Arrow keys**: Rotate camera

## Dependencies

The following Rust crates are used in the project:

- **minifb**: For creating a window and rendering the diorama.
- **nalgebra & nalgebra-glm**: For handling mathematical operations, particularly vector and matrix calculations.
- **image**: For loading and manipulating textures.
- **rayon**: For parallelizing tasks, improving performance.
- **once_cell**: For ensuring that resources like textures are loaded only once.

## Project Structure

- **src/main.rs**: Contains the main logic for initializing the window, loading textures, defining materials, and rendering the diorama.
- **textures/**: Contains the texture files used for various objects in the diorama.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---
