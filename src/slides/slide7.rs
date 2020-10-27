use super::*;

pub fn closures() {}

fn make_counter(initial_value: usize) -> impl FnMut() -> usize {
    let mut counter = initial_value;

    let counter_closure = move || {
        counter += 1;
        counter
    };

    counter_closure
}

#[cfg(test)]
#[test]
fn test_counter_closure() {
    let mut counter = make_counter(0);
    let one = counter();
    let two = counter();
    let three = counter();

    assert_eq!(one, 1);
    assert_eq!(two, 2);
    assert_eq!(three, 3);
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
