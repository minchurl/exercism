/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackId(usize, usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T> {
    idx: usize, 
    value: T, 
}

struct ComputeCell<'a, T> {
    idx: usize, 
    value: T, 
    dependencies: Vec<CellId>, 
    compute_func: Box<dyn 'a + Fn(&[T]) -> T>,
    callback_funcs: Vec<Option<Box<dyn 'a + FnMut(T)>>>, 
}

impl<T: Copy + PartialEq> InputCell<T> {
    fn new(_idx: usize, _initial: T) -> Self {
        Self {
            idx: _idx, 
            value: _initial, 
        }
    }
}

impl<'a, T: Copy + PartialEq> ComputeCell<'a, T> {
    fn new(_idx: usize, _initial: T, _dependencies: Vec<CellId>, _compute_func: Box<dyn Fn(&[T]) -> T>) -> Self {
        Self {
            idx: _idx, 
            value: _initial, 
            dependencies: _dependencies,
            compute_func: _compute_func, 
            callback_funcs: Vec::new(),
        }
    }
    fn add_callback<F: 'a + FnMut(T)>(&mut self, _callback: F) -> Option<CallbackId> {
        let callback_idx = self.callback_funcs.len();
        self.callback_funcs.push(Some(Box::new(_callback)));
        Some(CallbackId(self.idx, callback_idx))
        // unimplemented!()
    }

    fn remove_callback(&mut self, callback: CallbackId) -> Result<(), RemoveCallbackError> {
        if callback.1 >= self.callback_funcs.len() || self.callback_funcs[callback.1].is_none() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        self.callback_funcs[callback.1] = None;
        Ok(())
    }

    fn execute_callback(&mut self, argument: T) {
        for callback_func in self.callback_funcs.iter_mut() {
            match callback_func {
                Some(x) => {
                    let y = x.as_mut();
                    y(argument);
                }, 
                None => {}
            };
        }
    }
}


pub struct Reactor<'a, T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    input_cells: Vec<InputCell<T>>, 
    compute_cells: Vec<ComputeCell<'a, T>>, 
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            input_cells: Vec::new(), 
            compute_cells: Vec::new(), 
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, _initial: T) -> InputCellId {
        let new_input_cell_idx = self.input_cells.len();
        let new_input_cell = InputCell::new(new_input_cell_idx, _initial);
        self.input_cells.push(new_input_cell);
        InputCellId(new_input_cell_idx)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    fn check_invaild_dependency_cell(&self, dependencies: &[CellId]) -> Option<CellId> {
        let max_input_cell_idx = self.input_cells.len();
        let max_compute_cell_idx = self.compute_cells.len();
        if let Some(&x) = dependencies.iter().find(|&&x| {
            match x {
                CellId::Input(InputCellId(i)) => i >= max_input_cell_idx, 
                CellId::Compute(ComputeCellId(i)) => i >= max_compute_cell_idx, 
            }
        }) {
            return Some(x);
        }
        None
    }
    fn get_arguments(&self, dependencies: &[CellId]) -> Vec<T> {
        let arguments = dependencies.iter().map(|&x| {
            match x {
                CellId::Input(InputCellId(i)) => self.input_cells[i].value, 
                CellId::Compute(ComputeCellId(i)) => self.compute_cells[i].value, 
            }
        }).collect::<Vec<T>>();

        arguments
    }

    fn calculate_compute_cell(&self, cell_id: ComputeCellId) -> Option<T> {
        let ComputeCellId(idx) = cell_id;
        if idx >= self.compute_cells.len() {
            return None;
        }
        let cell = &self.compute_cells[idx];
        let arguments = self.get_arguments(&(cell.dependencies)[..]);
        let res = (cell.compute_func)(&arguments[..]);
        Some(res)
    }

    pub fn create_compute<F: 'static + Fn(&[T]) -> T>(
        &mut self,
        _dependencies: &[CellId],
        _compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        if let Some(x) = self.check_invaild_dependency_cell(_dependencies) {
            println!("{:?} {}", x, self.input_cells.len());
            return Err(x);
        }
        let arguments = self.get_arguments(_dependencies);
        let _initial = (_compute_func)(&arguments[..]);

        let new_compute_cell_idx = self.compute_cells.len();

        let new_compute_cell = ComputeCell::new(new_compute_cell_idx, _initial, _dependencies.to_vec(), Box::new(_compute_func));

        self.compute_cells.push(new_compute_cell);

        Ok(ComputeCellId(new_compute_cell_idx))
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(InputCellId(i)) => {
                if i < self.input_cells.len() {
                    Some(self.input_cells[i].value)
                }
                else {
                    None
                }
            }
            CellId::Compute(ComputeCellId(i)) => {
                if i < self.compute_cells.len() {
                    Some(self.compute_cells[i].value)
                }
                else {
                    None
                }
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.

    fn update_compute_cells(&mut self) {
        for i in 0..self.compute_cells.len() {
            let val = self.calculate_compute_cell(ComputeCellId(i)).unwrap();
            if self.compute_cells[i].value != val {
                self.compute_cells[i].execute_callback(val);
            }
            self.compute_cells[i].value = val;
        }
    }


    pub fn set_value(&mut self, _id: InputCellId, _new_value: T) -> bool {
        let InputCellId(idx) = _id;
        if idx >= self.input_cells.len() {
            return false;
        }
        self.input_cells[idx].value = _new_value;
        self.update_compute_cells();
        true
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        _id: ComputeCellId,
        _callback: F,
    ) -> Option<CallbackId> {
        let ComputeCellId(idx) = _id;
        if idx >= self.compute_cells.len() {
            return None;
        }
        self.compute_cells[idx].add_callback(_callback)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        let ComputeCellId(idx) = cell;
        println!("{} {:?} {}", idx, callback, self.compute_cells.len());
        if idx >= self.compute_cells.len() || idx != callback.0 {
            return Err(RemoveCallbackError::NonexistentCell);
        }
        self.compute_cells[idx].remove_callback(callback)
    }
}
