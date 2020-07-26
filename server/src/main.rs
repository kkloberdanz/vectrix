#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod vecstorage;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/get/<table>/<col>/<row>")]
fn get_cell(table: String, col: u64, row: u64) -> Result<String, String> {
    let file = vecstorage::find_file(table, col, row)?;
    let x = vecstorage::file_get_int(file, row);
    Ok(format!("{}", x))
}

#[post("/set/int/<table>/<col>/<row>/<value>")]
fn set_int(table: String, col: u64, row: u64, value: i64) -> Result<String, String> {
    let file = vecstorage::find_file(table, col, row)?;
    let x = vecstorage::file_set_int(file, row, value);
    Ok("ok".to_string())
}

fn lookup_file() -> Result<i32, String> {
    let file = vecstorage::find_file("employee".to_string(), 0, 30)?;
    vecstorage::file_set_int(file, 100, 108743242);
    let x = vecstorage::file_get_int(file, 100);
    println!("read {} from file", x);
    let ret = vecstorage::file_free(file);
    println!("got status code: {} from closing file", ret);
    Ok(ret)
}

fn main() {
    let x = vecstorage::print_hello(144);
    println!("i got back: {}", x);
    lookup_file();
    rocket::ignite().mount("/", routes![hello, get_cell, set_int]).launch();
}
