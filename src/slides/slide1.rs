use super::*;

pub fn syntax() {
    // Variablen deklarieren
    let five = 5;

    // Variablen mit explizizen Type-Annotationen deklarieren
    let four: u8 = 4;

    // Veränderbare Variablen
    let mut three = 2;
    three = 4;

    // Unveränderbare Referenz
    let three_ref = &three;
    // *three_ref = 3; // three kann nicht über &three_ref verändert werden
}

#[cfg(test)]
#[test]
fn mutable_ref() {
    let mut three = 2;
    assert_eq!(three, 2);

    // Veränderbare Referenz
    let three_ref_mut = &mut three;
    *three_ref_mut = 3;

    assert_eq!(three, 3);
}

#[cfg(test)]
#[test]
fn shadowing() {
    let v = 42;
    assert_eq!(v, 42);

    // Variable überschatten
    let v = 13.37;
    assert_eq!(v, 13.37)
}

#[cfg(test)]
#[test]
fn tuples() {
    // Tupel
    let point2 = (4, 5);
    let point2: (u8, u32) = (6, 15);
    let point3 = (4.0, 2, 3.8);

    // Zugriff auf Tupelelemente
    assert_eq!(point2.0, 6);
    assert_eq!(point2.1, 15);

    // Tupel destructuring
    let (x, y, z) = point3;
    assert_eq!(y, 2);

    let point3 = (2, 6.7, 14);
    let (x, _, z) = point3; // y wird weggeschmissen
    assert_eq!(z, 14);
    assert_eq!(y, 2); // das alte y ist immernoch vorhanden

    let point3 = (5, 30, "hi");
    let (x, ..) = point3; // alles nach x wird weggeschmissen
    assert_eq!(x, 5);
    // Die alten Werte wurden nicht überschrieben
    assert_eq!(z, 14);
    assert_eq!(y, 2);

    let point4 = (4.0, 2, 3.8, 5);
    // Mehrere Elemente in der Mitte ignorieren
    let (four, .., five) = point4;
    assert_eq!(four, 4.0);
    assert_eq!(five, 5);

    // Blocks / Scopes
    {
        // hier wird ein anderer point2 erstellt
        let point2: (usize, usize) = (49, 30);
        assert_eq!(point2, (49, 30));
    }

    assert_eq!(point2, (6, 15)); // alte Variable existiert noch
}

#[cfg(test)]
#[test]
fn return_from_block() {
    // Rückgabewert aus Block
    let sum = {
        let one = 1;
        let four = 4;
        four + one
    };

    assert_eq!(sum, 5)
}

#[cfg(test)]
#[test]
fn stack_array() {
    // Stack allokierter Array
    let mut stack_array: [u32; 5] = [0; 5];
    assert_eq!(stack_array, [0, 0, 0, 0, 0]);

    stack_array = [0, 1, 2, 3, 4];
    assert_eq!(stack_array, [0, 1, 2, 3, 4]);

    let initialized_array = [0, 1, 2, 3, 4];
    assert_eq!(initialized_array, stack_array);
}

#[cfg(test)]
#[test]
fn vector_slices() {
    // Heap allokierter Array
    // hier hinter verbirgt sich der in std
    // implementierte Typ Vec<T>
    let vec = vec![0, 1, 2, 3, 4];

    // Slices
    assert_eq!(&vec[1..3], &[1, 2]);
    assert_eq!(&vec[2..], &[2, 3, 4]);
    assert_eq!(&vec[..3], &[0, 1, 2]);
}

#[cfg(test)]
#[test]
fn string_primitive() {
    // Zeichenkette fester Länge
    let hello: &str = "Hello";

    // Zeichenkette dynamischer Länge
    let mut hello_dyn: String = String::from(hello);
    hello_dyn.push_str(", world!");

    assert_eq!(hello_dyn, "Hello, world!");

    // Slices / Sub-strings
    assert_eq!(&hello[1..], "ello");
    assert_eq!(&hello_dyn[1..5], &hello[1..]);
}

fn next_slide() {
    control_flow();
}
