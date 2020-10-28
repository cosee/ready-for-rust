use super::*;

pub fn closures() {}

fn make_counter(initial_value: usize) -> (impl FnMut() -> usize, impl FnMut() -> usize) {
    let mut counter = initial_value;

    let count_closure = move || {
        counter += 1;
        counter
    };

    let reset_closure = move || {
        counter = 0;
        counter
    };

    (count_closure, reset_closure)
}

#[cfg(test)]
#[test]
fn test_counter_closure() {
    let (mut count, mut reset) = make_counter(0);

    assert_eq!(count(), 1);
    assert_eq!(count(), 2);
    assert_eq!(count(), 3);

    assert_eq!(reset(), 0);

    assert_eq!(count(), 4); // todo: reset closure has its own counter?! This definitely works in Golang. is this because of Rust ownership?
    assert_eq!(count(), 5);
}

fn add_one(operand: &mut usize) {
    *operand += 1;
}

fn apply_function<T>(f: &dyn Fn(&mut T), t: &mut T) {
    f(t);
}

#[cfg(test)]
#[test]
fn test_apply_function() {
    let mut zero = 0;
    apply_function(&add_one, &mut zero);

    let mut zero2 = 0;
    let add_one_closure = |operand: &mut usize| *operand += 1;
    apply_function(&add_one_closure, &mut zero2);

    assert_eq!(zero, 1);
    assert_eq!(zero, zero2);
}
