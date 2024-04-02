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


fn get_usize_id(cell_id: CellId) -> usize {
    match cell_id {
        CellId::Input(input_cell_id) => input_cell_id.0,
        CellId::Compute(compute_cell_id) => compute_cell_id.0,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T>
{
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    cell_ids: Vec<CellId>,
    values: Vec<Option<T>>,
    dependencies: Vec<Option<Vec<CellId>>>,
    compute_funcs: Vec<Option<Box<dyn 'a + Fn(&[T]) -> T>>>,
    callback_funcs: Vec<Vec<Option<Box<dyn 'a + FnMut(T)>>>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            cell_ids: Vec::new(),
            values: Vec::new(),
            dependencies: Vec::new(),
            compute_funcs: Vec::new(),
            callback_funcs: Vec::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, _initial: T) -> InputCellId {
        let id = self.cell_ids.len();

        self.cell_ids.push(CellId::Input(InputCellId(id)));
        self.values.push(Some(_initial));
        self.dependencies.push(None);
        self.compute_funcs.push(None);
        self.callback_funcs.push(Vec::new());

        InputCellId(id)
    }

    fn update_compute_value(&mut self, idx: usize) {
        let dependencies_value = self.dependencies[idx]
            .clone()
            .unwrap()
            .iter()
            .map(|&i| self.values[get_usize_id(i)].unwrap())
            .collect::<Vec<T>>()
        ;
        let arguments = dependencies_value.as_slice();

        self.values[idx] = Some((self.compute_funcs[idx].as_ref().unwrap())(arguments));
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
    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(
        &mut self,
        _dependencies: &[CellId],
        _compute_func: F,
    ) -> Result<ComputeCellId, CellId> {


        let id = self.cell_ids.len();

        if let Some(err_id) = _dependencies.iter().position(|&depencendics_id| get_usize_id(depencendics_id) >= id) {
            return Err(_dependencies[err_id]);
        }


        self.cell_ids.push(CellId::Compute(ComputeCellId(id)));
        self.values.push(None);
        self.dependencies.push(Some(_dependencies.to_vec()));
        self.compute_funcs.push(Some(Box::new(_compute_func)));
        self.callback_funcs.push(Vec::new());

        self.update_compute_value(self.cell_ids.len() - 1);

        Ok(ComputeCellId(id))
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        let id = get_usize_id(id);
        if id >= self.cell_ids.len() {
            return None;
        }

        self.values[id]
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

        let id = get_usize_id(CellId::Input(_id));

        if id >= self.cell_ids.len() || self.values[id] == None {
            return false;
        }

        self.values[id] = Some(_new_value);

        for idx in id + 1..self.cell_ids.len() {
            if let CellId::Input(InputCellId(_)) = self.cell_ids[idx]{
                continue;
            }
            let origin_value = self.values[idx];
            self.update_compute_value(idx);

            if origin_value != self.values[idx] {
                for func in &mut self.callback_funcs[idx] {
                    if func.is_none() {
                        continue;
                    }
                    (func.as_mut().unwrap())(self.values[idx].unwrap());
                }
            }
        }

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
    pub fn add_callback<F: 'a + FnMut(T)>(
        &mut self,
        _id: ComputeCellId,
        _callback: F,
    ) -> Option<CallbackId> {
        let idx = _id.0;
        if idx >= self.cell_ids.len() {
            return None;
        }
        if let CellId::Input(_) = self.cell_ids[idx] {
            return None;
        }

        self.callback_funcs[idx].push(Some(Box::new(_callback)));
        Some(CallbackId(idx, self.callback_funcs[idx].len() - 1))
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
        let CallbackId(idx0, idx1) = callback;

        if idx != idx0 || idx >= self.cell_ids.len() {
            return Err(RemoveCallbackError::NonexistentCell);
        }

        if idx1 >= self.callback_funcs[idx0].len() || self.callback_funcs[idx0][idx1].is_none() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }

        self.callback_funcs[idx0][idx1] = None;

        Ok(())

        // todo!(
        //     "Remove the callback identified by the CallbackId {callback:?} from the cell {cell:?}"
        // )
    }
}
