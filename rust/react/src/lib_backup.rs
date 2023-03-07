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
pub struct CallbackId();

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


fn get_idx(id : CellId) -> usize {
    match id {
        CellId::Input(InputCellId(x)) => x, 
        CellId::Compute(ComputeCellId(y)) => y, 
    }
}


pub struct Reactor<T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    // dummy: ::std::marker::PhantomData<T>,
    ids: Vec<CellId>, 
    values: Vec<Option<T>>, 
    dependencies: Vec<Option<Vec<CellId>>>, 
    compute_funcs: Vec<Option<Box<dyn Fn(&[T]) -> T>>>, 
    cell_number: usize, 
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<T: Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        Self {
            ids: Vec::new(), 
            values: Vec::new(), 
            dependencies: Vec::new(), 
            compute_funcs: Vec::new(), 
            cell_number: 0, 
        }
        // unimplemented!()
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, _initial: T) -> InputCellId {
        self.ids.push(CellId::Input(InputCellId(self.cell_number)));
        self.values.push(Some(_initial));
        self.dependencies.push(None);
        self.compute_funcs.push(None);
        self.cell_number = self.cell_number + 1;


        InputCellId(self.cell_number - 1)
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

    fn update_value(&mut self, idx: usize) {
        let arguments = self.dependencies[idx].clone().unwrap().iter()
        .map(|&x| {get_idx(x)})
        .map(|x| {self.values[x].unwrap()})
        .collect::<Vec<T>>()
        ;
        
        let func = self.compute_funcs.get(idx).unwrap().as_ref().unwrap();
        self.values[idx] = Some(func(&arguments[..]));

    }

    fn update_values(&mut self) {
        for idx in 0..self.cell_number {
            if let CellId::Input(_) = self.ids[idx] {
                continue;
            }
            self.update_value(idx);
        }
    }

    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        _dependencies: &[CellId],
        _compute_func: F,
    ) -> Result<ComputeCellId, CellId> {

        if let Some(&i) = _dependencies.iter().find(|&&x| {
            get_idx(x) >= self.cell_number
        }) {
            return Err(i);
        }
        self.ids.push(CellId::Compute(ComputeCellId(self.cell_number)));
        self.values.push(None);
        self.dependencies.push(Some(_dependencies.to_vec()));
        self.compute_funcs.push(Some(Box::new(_compute_func)));

        self.cell_number = self.cell_number + 1;
        self.update_value(self.cell_number - 1);
        Ok(ComputeCellId(self.cell_number - 1))
    }


    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        let idx = get_idx(id);
        if idx >= self.cell_number || self.ids[idx] != id {
            return None;
        }
        self.values[idx]
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, _id: InputCellId, _new_value: T) -> bool {
        let InputCellId(idx) = _id;
        if idx >= self.cell_number {
            return false;
        }

        if self.ids[idx] != CellId::Input(_id) {
            return false;
        }

        self.values[idx] = Some(_new_value);
        self.update_values();

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
    pub fn add_callback<F: FnMut(T)>(
        &mut self,
        _id: ComputeCellId,
        _callback: F,
    ) -> Option<CallbackId> {
        unimplemented!()
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
        unimplemented!(
            "Remove the callback identified by the CallbackId {:?} from the cell {:?}",
            callback,
            cell,
        )
    }
}
