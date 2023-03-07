
#[derive(Debug)]
struct StringCells {
    cells: Vec<Box<String>>, 
}

impl StringCells {
    fn new() -> Self {
        Self {
            cells: Vec::new(),
        }
    }

    fn append_new_cell(&mut self, _new_cell: String) {
        self.cells.push(Box::new(_new_cell));
    }
}
fn main() {
    let x: String = String::from("asdf");
    let y: String = String::from("qwer");
    let mut string_cells = StringCells::new();
    string_cells.append_new_cell(x);
    string_cells.append_new_cell(y);
    println!("{:?}", string_cells);
    println!("Hello, world!");
}
