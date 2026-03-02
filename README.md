# 🦀 Learn Rust

> Kumpulan catatan dan contoh kode untuk belajar bahasa pemrograman **Rust** dari dasar hingga konsep-konsep penting seperti Ownership, Borrowing, dan Pattern Matching.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Edition](https://img.shields.io/badge/Edition-2021-blue?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

---

## 📋 Daftar Isi

- [Tentang Proyek](#-tentang-proyek)
- [Prasyarat](#-prasyarat)
- [Instalasi & Menjalankan](#-instalasi--menjalankan)
- [Struktur Proyek](#-struktur-proyek)
- [Materi Pembelajaran](#-materi-pembelajaran)
  - [1. Dasar-Dasar](#1-dasar-dasar)
  - [2. Operator](#2-operator)
  - [3. Kontrol Alur (Control Flow)](#3-kontrol-alur-control-flow)
  - [4. Fungsi](#4-fungsi)
  - [5. Struktur Data](#5-struktur-data)
  - [6. Ownership & Memory](#6-ownership--memory)
- [Roadmap Belajar](#-roadmap-belajar)
- [Referensi](#-referensi)

---

## 🎯 Tentang Proyek

Proyek ini adalah **catatan belajar Rust** yang berisi contoh-contoh kode untuk setiap topik fundamental. Setiap file di folder `src/` merepresentasikan satu topik pembelajaran, mulai dari variabel sederhana hingga konsep unik Rust seperti **Ownership** dan **Borrowing**.

**Total: 24 modul** yang mencakup seluruh dasar pemrograman Rust.

---

## 🔧 Prasyarat

- **Rust** (edisi 2021) — Install via [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Cargo** (otomatis terinstall bersama Rust)

Verifikasi instalasi:
```bash
rustc --version
cargo --version
```

---

## 🚀 Instalasi & Menjalankan

```bash
# Clone repository
git clone <repository-url>
cd learn_rust

# Build proyek
cargo build

# Jalankan proyek
cargo run
```

> **Catatan:** File-file di `src/` adalah catatan kode per-topik. Untuk menjalankan contoh tertentu, salin kodenya ke `src/main.rs` lalu jalankan `cargo run`.

---

## 📁 Struktur Proyek

```
learn_rust/
├── Cargo.toml              # Konfigurasi proyek & dependensi
├── Cargo.lock              # Lock file dependensi
├── README.md               # Dokumentasi (file ini)
├── src/
│   ├── variable.rs         # Variabel & deklarasi
│   ├── constants.rs        # Konstanta (const)
│   ├── int.rs              # Tipe integer & float
│   ├── str.rs              # String & operasi string
│   ├── boolean.rs          # Tipe boolean & logika
│   ├── types_data.rs       # Rangkuman semua tipe data
│   ├── operators.rs        # Operator aritmatika
│   ├── assign_operators.rs # Operator assignment (+=, -=, dll)
│   ├── comparison.rs       # Operator perbandingan
│   ├── logical_operators.rs# Operator logika (&&, ||, !)
│   ├── conditions.rs       # If, else if, else
│   ├── loops.rs            # Loop, while, for
│   ├── match.rs            # Pattern matching
│   ├── functions.rs        # Fungsi & return value
│   ├── scope.rs            # Scope & shadowing
│   ├── array.rs            # Array
│   ├── tuples.rs           # Tuple & destructuring
│   ├── vectors.rs          # Vector (dynamic array)
│   ├── hashmap.rs          # HashMap (key-value)
│   ├── structs.rs          # Struct & method (impl)
│   ├── enums.rs            # Enum & enum dengan data
│   ├── ownership.rs        # Ownership & move semantics
│   ├── borrowing.rs        # Borrowing (& dan &mut)
│   └── reference.rs        # Reference
└── target/                 # Build output (auto-generated)
```

---

## 📚 Materi Pembelajaran

### 1. Dasar-Dasar

#### 📄 `variable.rs` — Variabel
Deklarasi variabel menggunakan `let` dengan tipe data implisit.
```rust
let name = "Danz";
let age = 26;
println!("My name is : {}", name);
```

#### 📄 `constants.rs` — Konstanta
Konstanta menggunakan `const` dengan tipe data eksplisit. Nilainya tidak bisa diubah.
```rust
const BIRTHDAY: i32 = 2000;
const MINUTES_PER_HOUR: i32 = 60;
```

#### 📄 `int.rs` — Integer & Float
Tipe numerik `i32` untuk integer dan `f64` untuk float.
```rust
let age: i32 = 26;
let price: f64 = 10.99;
```

#### 📄 `str.rs` — String
Operasi string lengkap: `push_str`, concatenation, `format!`, slicing, `len()`, dan `contains()`.
```rust
let mut message = String::from("Hi");
message.push_str("Danz");

let s = String::from("Hi, Danz");
println!("Contains 'Danz': {}", s.contains("Danz"));
```

#### 📄 `boolean.rs` — Boolean
Tipe `bool` dengan operator logika `&&`, `||`, `!` dan penggunaannya dalam kondisi.
```rust
let is_programmer: bool = true;
let can_vote: bool = age >= 18;
```

#### 📄 `types_data.rs` — Rangkuman Tipe Data
Ringkasan semua tipe data Rust dalam satu file:

| Tipe | Contoh | Keterangan |
|------|--------|------------|
| `i32` | `5` | Integer 32-bit |
| `f64` | `5.88` | Float 64-bit |
| `char` | `'D'` | Karakter |
| `bool` | `false` | Boolean |
| `&str` | `"Hello"` | String slice |
| `(i32, &str, i32)` | `(1, "Danz", 26)` | Tuple |
| `[i32; 7]` | `[1,3,5,6,6,6,5]` | Array |
| `Vec<i32>` | `vec![1,2,3]` | Vector |
| `HashMap<K,V>` | `HashMap::new()` | HashMap |
| `Option<T>` | `None` | Nullable value |
| `Result<T,E>` | `Ok(5)` / `Err("err")` | Error handling |

---

### 2. Operator

#### 📄 `operators.rs` — Operator Aritmatika
```rust
let add = 5 + 20;   // Penjumlahan
let sub = 10 - 4;   // Pengurangan
let mul = 7 * 8;    // Perkalian
let div = 60 / 5;   // Pembagian
let rem = 10 % 3;   // Modulus (sisa bagi)
```

#### 📄 `assign_operators.rs` — Operator Assignment
```rust
let mut x = 10;
x += 5;   // x = 15
x -= 2;   // x = 13
x *= 3;   // x = 39
x /= 2;   // x = 19
x %= 3;   // x = 1
```

#### 📄 `comparison.rs` — Operator Perbandingan
```rust
println!("10 == 54: {}", a == b);  // Sama dengan
println!("10 != 54: {}", a != b);  // Tidak sama
println!("10 > 54: {}",  a > b);   // Lebih besar
println!("10 < 54: {}",  a < b);   // Lebih kecil
```

#### 📄 `logical_operators.rs` — Operator Logika
```rust
let logged_in = true;
let is_admin = false;

println!("Is regular user: {}", logged_in && !is_admin); // AND + NOT
println!("Has any access: {}",  logged_in || is_admin);  // OR
```

---

### 3. Kontrol Alur (Control Flow)

#### 📄 `conditions.rs` — If / Else
Kondisi dasar, `if` sebagai expression, dan `if else if` chain.
```rust
let score = 85;
if score >= 90 {
    println!("Grade A");
} else if score >= 80 {
    println!("Grade B");
} else {
    println!("Grade F");
}

// If sebagai expression
let greeting = if time < 18 { "Good Day." } else { "Good Evening." };
```

#### 📄 `loops.rs` — Perulangan
Tiga jenis loop: `loop`, `while`, dan `for` — termasuk `break` dan `continue`.
```rust
// Infinite loop dengan break
loop {
    if count == 10 { break; }
    count += 1;
}

// While dengan continue (skip)
while num <= 10 {
    if num == 5 { num += 1; continue; }
    num += 1;
}

// For loop dengan range
for count in 1..=10 {
    println!("Hi Danz");
}
```

#### 📄 `match.rs` — Pattern Matching
Pattern matching: dasar, multiple values dengan `|`, dan range `..=`.
```rust
match day {
    1 | 2 | 3 | 4 | 5 => println!("Weekday"),
    6 | 7             => println!("Weekend"),
    _                 => println!("Invalid day"),
}

match age {
    0..=12  => println!("Child"),
    13..=19 => println!("Teenager"),
    20..=64 => println!("Adult"),
    _       => println!("Senior"),
}
```

---

### 4. Fungsi

#### 📄 `functions.rs` — Fungsi
Deklarasi fungsi, parameter, return value, dan multiple return (tuple).
```rust
// Fungsi sederhana
fn say_hello() {
    println!("Hello Danz");
}

// Dengan parameter & return
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// Multiple return dengan Tuple
fn get_user() -> (String, i32) {
    return ("Danz".to_string(), 26);
}
```

---

### 5. Struktur Data

#### 📄 `array.rs` — Array
Array fixed-size: akses elemen, `.len()`, dan iterasi.
```rust
let fruits = ["Melon", "Orange", "Mango"];
println!("Length: {}", fruits.len());

for fruit in fruits {
    println!("{}", fruit);
}
```

#### 📄 `tuples.rs` — Tuple
Tuple untuk menyimpan data berbeda tipe, akses dengan index dan destructuring.
```rust
let person = ("Danz", 26, "Male");
println!("Name: {}", person.0);

// Destructuring
let (name, age, gender) = person;
```

#### 📄 `vectors.rs` — Vector
Vector (dynamic array): `push`, akses elemen, dan iterasi.
```rust
let mut animals = vec!["Dog", "Cat", "Bird"];
animals.push("Fish");
```

#### 📄 `hashmap.rs` — HashMap
Key-value store menggunakan `HashMap`.
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Danz", 96);
scores.insert("John", 85);

let score = scores.get("Danz");
```

#### 📄 `structs.rs` — Struct
Struct untuk custom type dan method dengan `impl`.
```rust
struct Animal {
    name: String,
    age: i32,
    is_wild: bool,
}

impl Animal {
    fn new(name: String, age: i32, ...) -> Animal {
        Animal { name, age, ... }
    }
}
```

#### 📄 `enums.rs` — Enum
Enum dasar, enum dengan data, dan pattern matching.
```rust
enum TrafficLight { Red, Yellow, Green }
enum LoginStatus {
    Success(String),
    Error(String),
    Pending,
}

match status {
    LoginStatus::Success(msg) => println!("{}", msg),
    LoginStatus::Error(msg)   => println!("{}", msg),
    LoginStatus::Pending      => println!("Pending"),
}
```

---

### 6. Ownership & Memory

> 💡 **Ini adalah konsep paling penting dan unik di Rust!**

#### 📄 `scope.rs` — Scope & Shadowing
Variabel hanya valid di dalam scope-nya. Shadowing memungkinkan re-deklarasi variabel.
```rust
let x = 10;
let x = x + 7;   // Shadowing: x = 17
let x = x * 3;   // Shadowing: x = 51

let x = 10;
let x = "Hello"; // Shadowing dengan tipe berbeda
```

#### 📄 `ownership.rs` — Ownership
Setiap value hanya punya satu owner. Assignment memindahkan (move) ownership.
```rust
let a = String::from("Hi Danz");
let b = a;            // a di-move ke b
// println!("{}", a); // ❌ Error! a sudah tidak valid

let b = a.clone();    // ✅ Clone untuk menyalin data
println!("{}", a);    // ✅ a masih valid
```

#### 📄 `borrowing.rs` — Borrowing
Meminjam value tanpa memindahkan ownership menggunakan `&` (immutable) dan `&mut` (mutable).
```rust
let a = String::from("Hi Danz");
let b = &a;           // Immutable borrow
println!("{}", b);    // ✅ a masih valid

let mut a = String::from("Hi Danz");
let b = &mut a;       // Mutable borrow
b.push_str("Hello");
```

> ⚠️ **Aturan Borrowing:**
> - Boleh multiple immutable borrow (`&T`) sekaligus
> - Hanya boleh satu mutable borrow (`&mut T`) pada satu waktu
> - Tidak boleh immutable + mutable borrow bersamaan

#### 📄 `reference.rs` — Reference
Referensi sebagai pointer aman yang merujuk ke data tanpa mengambil ownership.
```rust
let a = String::from("Hi Danz");
let b = &a;
println!("a = {}", a); // ✅ Keduanya valid
println!("b = {}", b);
```

---

## 🗺️ Roadmap Belajar

Urutan belajar yang disarankan:

```
1. variable.rs ──→ 2. constants.rs ──→ 3. int.rs ──→ 4. str.rs
       │
       ▼
5. boolean.rs ──→ 6. types_data.rs ──→ 7. operators.rs
       │
       ▼
8. assign_operators.rs ──→ 9. comparison.rs ──→ 10. logical_operators.rs
       │
       ▼
11. conditions.rs ──→ 12. loops.rs ──→ 13. match.rs
       │
       ▼
14. functions.rs ──→ 15. scope.rs
       │
       ▼
16. array.rs ──→ 17. tuples.rs ──→ 18. vectors.rs ──→ 19. hashmap.rs
       │
       ▼
20. structs.rs ──→ 21. enums.rs
       │
       ▼
22. ownership.rs ──→ 23. borrowing.rs ──→ 24. reference.rs
```

---

## 📖 Referensi

| Sumber | Link |
|--------|------|
| 📕 The Rust Programming Language | [doc.rust-lang.org/book](https://doc.rust-lang.org/book/) |
| 📗 Rust By Example | [doc.rust-lang.org/rust-by-example](https://doc.rust-lang.org/rust-by-example/) |
| 📘 Rust Reference | [doc.rust-lang.org/reference](https://doc.rust-lang.org/reference/) |
| 📙 Rustlings (Exercises) | [github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings/) |
| 🔧 Cargo Book | [doc.rust-lang.org/cargo](https://doc.rust-lang.org/cargo/) |
| 🌐 Rust Playground | [play.rust-lang.org](https://play.rust-lang.org/) |

---

<p align="center">
  Made with 🦀 by <strong>Danz</strong> — Happy learning Rust!
</p>
