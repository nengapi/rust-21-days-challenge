# เฉลย Day 1

ดูโค้ดตัวอย่างได้ที่ [ที่นี่](https://github.com/nengapi/rust-21-days-challenge/tree/main/solutions/day1)

### Project Structure
เราเลือกใช้เป็น  App + Folder Module
```
📂 day1
├─ 📂 src
│  │
│  ├─ 📂 utils
│  │  ├─ 📄 mod.rs    # 👈 entrypoint (similar to index.js in JS).
│  │  ├─ 📄 echo.rs   # 👈 Contain echo function.
│  │  ├─ 📄 cat.rs    # 👈 Contain cat function.
│  │  └─ 📄 ls.rs     # 👈 Contain ls function.
│  │
│  └─ 📄 main.rs      # 👈 `mod utils;` และ `use utils::*;` 
│
└─ 📦 Cargo.toml
```
<br />

📄 echo.rs
```rs, editable
pub fn main(args: Vec<String>) {
    let args: Vec<String> = args.into_iter().skip(1).collect();
    println!("{}", args.join(" ")); //แสดงผลข้อความ
}
```
อธิบายแบบละเอียด
- **บรรทัดที่ 2** เอาไว้ตัดคำสั่งออกเช่น `[cmd] hello world` จะกลายเป็น `hello world` 

<br />

📄 cat.rs
```rs, editable
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn main(args: Vec<String>) {
    let path: &Path = Path::new(&args[1]); // เอาไว้เก็บ path ของไฟล์
    let file: File = match File::open(path) { // เอาไว้เปิดไฟล์
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            std::process::exit(1);
        }
    };
    let reader: BufReader<File> = BufReader::new(file); // เอาไว้อ่านไฟล์

    for line in reader.lines() { // เอาไว้อ่านทุกบรรทัดในไฟล์
        match line {
            Ok(content) => println!("{}", content),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}
```
อธิบายแบบละเอียด
- **บรรทัดที่ 1-3** ดึง library ที่เราจำเป็นใช้งาน 
  - `std::fs::File` เป็น lib สำหรับใช้งานกับไฟล์
  - `std::io::{BufRead, BufReader}` เป็น lib สำหรับใช้งานกับการอ่านและเขียนไฟล์
  - `std::path::Path` เป็น lib สำหรับใช้งานกับ path

> [!NOTE]
> ในภาษา Rust การจัดการ Error จะต้องระบุอยู่เสมอ

> [!TIP]
> เราสามารถย่อรูปการจัดการ Error ได้โดยใช้ `?` เช่น `File::open(path)?` จะทำให้การจัดการ Error ง่ายขึ้น
<br />

📄 ls.rs
```rs, editable
use std::fs;
use std::path::Path;

pub fn main(args: Vec<String>) {
    let path: &Path = if args.len() > 1 { 
        Path::new(&args[1])
    } else {
        Path::new(".")
    };

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(dir_entry) => {
                        if let Some(file_name) = dir_entry.file_name().to_str() {
                            println!("{}", file_name);
                        }
                    }
                    Err(e) => eprintln!("Error reading directory entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
}
```
อธิบายแบบละเอียด
- **บรรทัดที่ 16** เอาไว้ตรวจสอบว่าไฟล์มีชื่อหรือไม่ ถ้ามีก็จะทำการแสดงผล

> [!NOTE]
> if let เป็นรูปย่อของการใช้ `match` เพื่อตรวจสอบว่ามีค่าหรือไม่ ถ้ามีก็จะทำการแสดงผล <br />
> มาดูรูปเต็มๆ กัน ถ้าไม่ใช่ if let ก็จะต้องเป็นอย่างนี้
> ```rs, editable
> let file_name = dir_entry.file_name().to_str();
> match file_name {
>     Some(name) => println!("{}", name),
>     None => eprintln!("Failed to convert file name to string"),
> }
> ```
<br />

📄 mod.rs
```rs, editable
pub mod echo;
pub mod cat;
pub mod ls;
```
อธิบายแบบละเอียด
- เอาไว้ประกาศชื่อของโมดูลที่จะใช้งาน
<br />

📄 main.rs
```rs, editable
mod utils;
use std::env;
use utils::*;

fn main() {
    let args: Vec<String> = env::args().collect(); // เอาไว้รับค่าจากคำสั่งที่รันจาก CLI

    // ทดลองใช้งาน echo
    // utils::echo::main(args);

    // ทดลองใช้งาน cat กับ test.txt
    // utils::cat::main(args);

    // ทดลองใช้งาน ls
    // utils::ls::main(args);
}
```
> [!TIP]
> `use utils::*;` จะทำให้เราสามารถเรียกใช้งาน function จากไฟล์อื่นๆ ได้ทั้งหมดในโฟลเดอร์ `utils` โดยไม่ต้องระบุ function ทีละตัว

### ทดลองใช้งาน
ไปที่โฟลเดอร์ `solutions/day1` แล้ว uncomment คำสั่งที่ต้องการใช้งาน จากนั้นรันคำสั่งด้านล่างใน terminal

สำหรับ echo
```bash
cargo run hello world
```

สำหรับ cat
```bash
cargo run test.txt
```

สำหรับ ls
```bash
cargo run .
```
