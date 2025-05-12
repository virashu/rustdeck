#[allow(dead_code, reason = "WIP")]
pub enum ErrorKind {
    NotALibrary = 193,
}

pub fn report_error(e: libloading::Error) {
    match e {
        libloading::Error::LoadLibraryExW { source } => {
            // source.0.raw_os_error();
            let err_code_1 = std::io::Error::last_os_error().raw_os_error();

            // This freaking thing cannot just give me the number, it only has format as public...
            let f = format!("{:?}", &source);
            let err_code_2: i32 = f
                .split_once(",")
                .unwrap()
                .0
                .strip_prefix("Os { code: ")
                .unwrap()
                .parse()
                .unwrap();

            let err_code = err_code_1.unwrap_or(err_code_2);

            match err_code {
                193 => {
                    println!("Not a library");
                }
                _ => {
                    println!("{:?}", source);
                }
            }
        }
        _ => {
            println!("{:?}", e);
        }
    }
}
