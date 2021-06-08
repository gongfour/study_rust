use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn open_file()
{
    // File::open()은 io::Result<File>을 반환
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn open_file2()
{
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        // 에러의 종류를 나열할 수 있음
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e), // 생성 에러
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error) // 그 외 에러
            }
        },
    };
}

fn unwrap_expect()
{
    // 앞선 예제는 너무 장황하다.
    // panic을 쉽게 일으키는 방법
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn propagate_error1() -> Result<String, io::Error>
{
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn propagate_error2() -> Result<String, io::Error>
{
    // ? 연산자를 사용하면 쉽게 에러를 전파할 수 있다.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    
    // 더 간단하게도 가능
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

pub fn run()
{
    // open_file();
    // open_file2();
    unwrap_expect();

    propagate_error1();
    propagate_error2();
}