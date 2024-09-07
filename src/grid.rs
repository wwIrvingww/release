use nalgebra_glm::Vec3;

// Define el tipo de objeto que puede haber en el grid. Por ahora, lo dejamos como un enum.
#[derive(Clone, Debug)]
pub enum GridObject {
    Empty,      // Representa una celda vacía
    Cube,       // Representa un cubo
    Sphere,     // Representa una esfera
    // Otros objetos pueden agregarse aquí
}

pub struct Grid3D {
    size: usize,                         // Tamaño del grid (10x10x10)
    cells: Vec<Vec<Vec<GridObject>>>,    // Vec anidado para representar cada celda del grid
}

impl Grid3D {
    // Constructor para crear un nuevo grid 3D con el tamaño especificado
    pub fn new(size: usize) -> Self {
        let empty_grid = vec![vec![vec![GridObject::Empty; size]; size]; size];  // Inicializar todas las celdas como vacías
        Grid3D {
            size,
            cells: empty_grid,
        }
    }

    // Método para colocar un objeto en una posición específica (x, y, z)
    pub fn place_object(&mut self, x: usize, y: usize, z: usize, object: GridObject) {
        if x < self.size && y < self.size && z < self.size {
            self.cells[x][y][z] = object;
        } else {
            println!("Posición fuera del rango del grid.");
        }
    }

    // Método para obtener un objeto en una posición específica (x, y, z)
    pub fn get_object(&self, x: usize, y: usize, z: usize) -> &GridObject {
        if x < self.size && y < self.size && z < self.size {
            &self.cells[x][y][z]
        } else {
            println!("Posición fuera del rango del grid.");
            &GridObject::Empty
        }
    }

    // Método para imprimir el estado del grid (para depuración)
    pub fn print_grid(&self) {
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    match self.get_object(x, y, z) {
                        GridObject::Empty => print!("[ ]"),
                        GridObject::Cube => print!("[C]"),
                        GridObject::Sphere => print!("[S]"),
                    }
                }
                println!(); // Salto de línea después de cada fila
            }
            println!("---"); // Separador entre capas del eje Z
        }
    }
}
