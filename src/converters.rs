use std::collections::HashMap;

pub fn convert_picker (mode: &str, file: &str) -> () {
    // Format tuples should contain function calls:
    // [0] should be reader, [1] should be writer
    // However if it has direct func to convert, do not use that HashMap
    let formats = HashMap::from([
        ("Markdown", (0,0)),
        ("HTML",     (0,0)),
        ("PDF",      (0,0)),
    ]);
    match mode {
        "MD to HTML"  => (),
        "MD to PDF"   => (),
        "HTML to PDF" => (),
        _             => (),
    };
}
