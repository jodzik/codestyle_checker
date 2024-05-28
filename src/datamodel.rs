use std::path::PathBuf;

struct File {
    path: PathBuf,
    file_type: FileType,
    functions: Vec<Function>,
}

enum FileType {
    SourceC,
    HeaderC,
}

struct Function {
    name: String,
    return_type: String,
    parameters: Vec<Parameter>,
    is_static: bool,
    is_unsafe: bool,
    body: String,
}

struct Parameter {
    
}
