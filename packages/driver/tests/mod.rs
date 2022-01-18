use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Command,
    thread,
};

use assert_cmd::prelude::CommandCargoExt;
use path_absolutize::Absolutize;

struct TestSetup {
    dir_name: String,
    stdout: Option<String>,
}

impl TestSetup {
    fn new() -> Self {
        let handle = thread::current();
        let name = handle.name().unwrap().to_string();
        let t_setup = TestSetup {
            dir_name: name,
            stdout: None,
        };
        t_setup.setup();
        return t_setup;
    }

    fn setup(&self) {
        let setup_test_dir_path = self.get_setup_test_dir_path();

        if setup_test_dir_path.exists() {
            fs::remove_dir_all(&setup_test_dir_path).unwrap();
        }

        fs::create_dir(setup_test_dir_path).unwrap();
    }

    fn get_setup_test_dir_path(&self) -> PathBuf {
        let driver_dir_path = std::env::current_dir().unwrap();

        let setup_test_dir_path = {
            let mut driver_path = driver_dir_path.clone();
            driver_path.push("tests");
            driver_path.push("setup");
            driver_path.push(self.dir_name.as_str());
            driver_path.absolutize().unwrap().to_path_buf()
        };

        return setup_test_dir_path;
    }

    fn get_built_exec_path(&self) -> PathBuf {
        let mut setup_path = self.get_setup_test_dir_path();
        setup_path.push("./output");
        return setup_path.absolutize().unwrap().to_path_buf();
    }

    pub fn create_file(&self, path: &str, source_code: &str) {
        let file_path = self.get_absolute_file_path(path);
        let file_dir = file_path.parent().unwrap();

        fs::create_dir_all(file_dir).unwrap();

        let mut file = File::create(file_path).unwrap();
        file.write_all(source_code.as_bytes()).unwrap();
    }

    pub fn clean(&self) {
        let setup_test_dir_path = self.get_setup_test_dir_path();
        fs::remove_dir_all(setup_test_dir_path).unwrap();
    }

    pub fn get_absolute_file_path(&self, path: &str) -> PathBuf {
        let mut setup_path = self.get_setup_test_dir_path();
        setup_path.push(path);
        let file_path = setup_path.absolutize().unwrap().to_path_buf();
        return file_path;
    }

    pub fn compile(&mut self, main_file_path: &str) {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        cmd.current_dir(self.get_setup_test_dir_path());
        cmd.arg(main_file_path);
        cmd.output().unwrap();

        let output = Command::new(self.get_built_exec_path()).output().unwrap();
        self.stdout = Some(String::from_utf8(output.stdout).unwrap());
    }

    pub fn assert(&self, expected_output: &str) {
        if let Some(actual_output) = &self.stdout {
            assert_eq!(actual_output, expected_output);
        }
    }
}

#[test]
fn test_simple_syscall_print() {
    let mut setup = TestSetup::new();

    let main_file = "
    import {syscallPrint} from \"compilerInternal\";

    syscallPrint(1, \"Hello World!\", 12);

    ";

    let main_file_path = "./main.ts";

    setup.create_file(main_file_path, main_file);
    setup.compile(main_file_path);

    setup.assert("Hello World!");

    setup.clean();
}

#[test]
fn test_modules_1() {
    let mut setup = TestSetup::new();

    let main_file = "
    import {syscallPrint} from \"compilerInternal\";
    import {isTrue} from \"./foo.ts\";


    if (isTrue()) {
       syscallPrint(1, \"its true\", 8);
     }

    if (!isTrue()) {
       syscallPrint(1, \"its false\", 9);
    }";

    let main_file_path = "./main.ts";

    setup.create_file(main_file_path, main_file);

    let foo_file = "
    export function isTrue() : boolean {
       return false;
    }";

    let foo_file_path = "./foo.ts";
    setup.create_file(foo_file_path, foo_file);

    setup.compile(main_file_path);

    setup.assert("its false");

    setup.clean();
}

#[test]
fn test_modules_2() {
    let mut setup = TestSetup::new();

    let main_file = "
    import {syscallPrint} from \"compilerInternal\";
    import {isTrue} from \"./foo.ts\";


    if (isTrue()) {
       syscallPrint(1, \"its true\", 8);
     }

    if (!isTrue()) {
       syscallPrint(1, \"its false\", 9);
    }";

    let main_file_path = "./main.ts";

    setup.create_file(main_file_path, main_file);

    let foo_file = "
    export function isTrue() : boolean {
       return true;
    }";

    let foo_file_path = "./foo.ts";
    setup.create_file(foo_file_path, foo_file);

    setup.compile(main_file_path);

    setup.assert("its true");

    setup.clean();
}
