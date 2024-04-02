/// What should the type of _function be?
pub fn map<F, T, U>(input: Vec<T>, mut _function: F) -> Vec<U>
    where F: FnMut(T) -> U
{
    let mut output: Vec<U> = Vec::new();

    for i in input {
        output.push(_function(i));
    }


    output
}