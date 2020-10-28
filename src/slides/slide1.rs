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

    // Variablen überschatten
    let three = 3.0;

    // Tupel
    let point2 = (4, 5);
    let point2: (u8, u32) = (6, 15);
    let point3 = (4.0, 2, 3.8);

    // Zugriff auf Tupelelemente
    let x = point2.0; // 6
    let y = point2.1; // 15

    // Tupel destructuring
    let (x, y, z) = point3; // x = 5, y = 7, z = 1
    let (x, _, z) = point3; // x = 5, y wird weggeschmissen, z = 1
    let (x, ..) = point3; // x = 5, alles andere wird weggeschmissen

    // Blocks / Scopes
    {
        // Hier wird ein anderer point2 erstellt
        let point2: (usize, usize) = (49, 30);
        let (x, y) = point2; // x = 49, y = 30
    }

    // Alte Variable existiert noch
    let (x, y) = point2; // x = 6, y = 15

    // Rückgabewert aus Block
    let sum = {
        let one = 1;
        let four = 4;
        four + one
    };

    // Stack allokierter Array
    let mut stack_array: [u32; 5] = [0; 5];
    stack_array = [0, 1, 2, 3, 4];

    // Heap allokierter Array
    let mut vec = Vec::new();
    vec = vec![0, 1, 2, 3, 4];

    // Slices
    let slice_middle = &vec[1..3]; // [1, 2]
    let slice_to_end = &vec[2..]; // [2, 3, 4]
    let slice_from_start = &vec[..3]; // [0, 1, 2]
}

fn next_slide() {
    control_flow();
}
