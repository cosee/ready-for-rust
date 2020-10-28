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

    // Veränderbare Referenz
    let three_ref_mut = &mut three;
}

#[cfg(test)]
#[test]
fn test_shadowing() {
    let v = 42;
    assert_eq!(v, 42);

    // Variablen überschatten
    let v = 13.37;
    assert_eq!(v, 13.37)
}

#[cfg(test)]
#[test]
fn test_tuples() {
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
    let (x, _, z) = point3; // y wird weggeschmissen
    assert_eq!(z, 3.8);
    let (x, ..) = point3; // alles nach x wird weggeschmissen
    assert_eq!(x, 4.0);

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
fn test_block() {
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
fn test_stack_array() {
    // Stack allokierter Array
    let mut stack_array: [u32; 5] = [0; 5];
    assert_eq!(stack_array, [0, 0, 0, 0, 0]);

    stack_array = [0, 1, 2, 3, 4];
    assert_eq!(stack_array, [0, 1, 2, 3, 4]);
}

#[cfg(test)]
#[test]
fn test_vector_slices() {
    // Heap allokierter Array
    let vec = vec![0, 1, 2, 3, 4];

    // Slices
    assert_eq!(&vec[1..3], &[1, 2]);
    assert_eq!(&vec[2..], &[2, 3, 4]);
    assert_eq!(&vec[..3], &[0, 1, 2]);
}

fn next_slide() {
    control_flow();
}
