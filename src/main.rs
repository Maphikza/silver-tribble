fn main() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    cape penguin, 35
    Fiordland penguin,60
    Invalid,data
    ";
    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 { // This deals with the header row and handles lines with only white spaces.
            continue;
        }

        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        // Vec can dynamically expand and <_> tells the compiler to infer the type of the vector's elements.
        // The split(',') will split the record into substrings which are then trimmed of whitespaces using field.trim()
        // As the trimming is done under the map iterator the .collect tells rust to collects results of the iterator into a vector called fields.


        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields); // prints to standard error
        }

        let name = fields[0];

        let maybe_length: Result<f32, _> = fields[1].parse();

        if maybe_length.is_err() {
            continue;
        }

        let length = maybe_length.unwrap();

        println!("{}, {}cm", name, length);
    }
}
