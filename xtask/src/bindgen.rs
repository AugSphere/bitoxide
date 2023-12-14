use std::{
    fs::OpenOptions,
    io::{BufRead as _, BufReader, Read as _, Result as IoResult, Seek, Write},
    path::{Path, PathBuf},
    process::Command,
};

use base64::write::EncoderWriter;

use crate::Profile;

pub fn generate_js_bindings(profile: Profile, wasm_paths: Vec<PathBuf>, output_path: &Path) {
    for path in wasm_paths {
        wasm_to_js(&path, output_path, profile == Profile::Dev);
    }
}

fn wasm_to_js(wasm_path: &Path, output_path: &Path, debug: bool) {
    println!(
        "Generating js {} debug from {wasm_path:?}",
        if debug { "with" } else { "without" }
    );
    let crate_name = wasm_path.file_stem().unwrap().to_str().unwrap().to_owned();
    let wasm_b64 = encode_wasm_js_decl(&*wasm_path, &*output_path, &*crate_name, debug);
    join_with_binder(wasm_b64, output_path, &crate_name);
}

/// Creates the WASM file, encodes it into base64, and creates a JavaScript
/// declaration of the contents of the encoded WASM file.
fn encode_wasm_js_decl(
    wasm_path: &Path,
    wasm_output: &Path,
    crate_name: &str,
    debug: bool,
) -> String {
    struct Writable {
        contents: Vec<u8>,
        line_len: usize,
    }

    impl Writable {
        fn new() -> Writable {
            let contents = "const wasm_b64 = \"".to_owned().into_bytes();
            let line_len = contents.len();

            Writable { contents, line_len }
        }

        fn finish(mut self) -> String {
            self.contents.push(b'\"');
            self.contents.push(b';');
            return String::from_utf8(self.contents).unwrap();
        }
    }

    impl Write for Writable {
        fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
            let mut idx = 0;

            while idx < buf.len() {
                let chars_left_on_line = 79 - self.line_len;

                if chars_left_on_line == 0 {
                    self.contents.push(b'\\');
                    self.contents.push(b'\n');
                    self.line_len = 0;
                    continue;
                }

                let chars_left_on_buffer = buf.len() - idx;

                let chars_to_write = chars_left_on_line.min(chars_left_on_buffer);

                self.contents
                    .write(&buf[idx..idx + chars_to_write])
                    .unwrap();
                idx += chars_to_write;
                self.line_len += chars_to_write;
            }

            Ok(buf.len())
        }

        fn flush(&mut self) -> IoResult<()> {
            Ok(())
        }
    }

    // run wasm-bindgen
    let mut command = Command::new("wasm-bindgen");
    command
        .arg("--target")
        .arg("web")
        .arg("--no-typescript")
        .arg(&wasm_path)
        .arg("--out-dir")
        .arg(&wasm_output);

    if debug {
        command.arg("--debug");
        command.arg("--keep-debug");
    }

    command.output().expect("Cannot run wasm-bindgen");

    let wasm_file = OpenOptions::new()
        .read(true)
        .open(wasm_output.join(format!("{}_bg.wasm", crate_name)))
        .map(|file| BufReader::new(file))
        .expect("Cannot read the wasm file.");

    // encode
    let mut writable = Writable::new();
    {
        let mut encoder =
            EncoderWriter::new(&mut writable, &base64::engine::general_purpose::STANDARD);

        for byte in wasm_file.bytes() {
            encoder.write_all(&[byte.unwrap()]).unwrap();
        }
        encoder.flush().unwrap();
        encoder.finish().unwrap();
    }

    writable.finish()
}

fn join_with_binder(mut js_str: String, wasm_output: &Path, crate_name: &str) {
    let mut js_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(wasm_output.join(format!("{}.js", crate_name)))
        .expect("Cannot open the bundler js file");
    let mut reader = BufReader::new(&js_file);

    let mut buffer = String::new();
    js_str += "\n";
    loop {
        buffer.clear();
        match reader
            .read_line(&mut buffer)
            .expect("Cannot read the js file.")
        {
            0 => break,
            _ => {}
        }

        // stop reading from here. we'll have our own initializer.
        if buffer.contains("function initSync(module) {") {
            break;
        }

        js_str += &buffer;
    }

    js_str += include_str!("./addendum.js");
    js_file
        .rewind()
        .expect("Failed to rewind to start of js file");
    js_file
        .write_all(js_str.as_bytes())
        .expect("Failed to write updated js file")
}
