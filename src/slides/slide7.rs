use super::*;

pub fn closures() {}

fn make_counter(initial_value: usize) -> (impl FnMut() -> usize, impl FnMut() -> usize) {
    let mut counter = initial_value;

    let counter_closure = move || {
        counter += 1;
        counter
    };

    let reset_counter = move || {
        counter = initial_value;
        counter
    };

    // Beide closures haben ihre eigene Kopie von counter.
    // Da conunter einer der Datentypen ist, die Copy
    // implementieren, kann er u.U. als Kopie seines
    // Wertes übergeben werden, wo bei anderen Datentypen
    // die Ownership übergeben würde.

    (counter_closure, reset_counter)
}

#[cfg(test)]
#[test]
fn test_counter_closure() {
    let (mut count, mut reset) = make_counter(0);
    let one = count();
    let two = count();
    let three = count();

    assert_eq!(one, 1);
    assert_eq!(two, 2);
    assert_eq!(three, 3);

    assert_eq!(0, reset());

    // Da reset seinen eigenen counter hat wurde der
    // Zähler für counter nicht zurückgesetzt.
    // Der geneigte Zuschauer / Leser kann versuchen
    // make_counter dahingehend anzupassen.
    // Ein paar Wegweiser dafür:
    // https://doc.rust-lang.org/stable/std/rc/index.html
    // https://doc.rust-lang.org/stable/std/cell/index.html
    // https://doc.rust-lang.org/stable/std/cell/index.html#introducing-mutability-inside-of-something-immutable

    assert_eq!(count(), 4);
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
