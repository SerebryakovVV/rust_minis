use rusqlite as rsql;


fn main() {

    let path = "../tasks.db3";
    let db = match rsql::Connection::open(path) {
        Err(_) => panic!(),
        Ok(con) => con
    };

    let query_result = match db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY,name  TEXT NOT NULL)", ()) {
        Err(_) => panic!(),
        Ok(r) => r 
    };

    

    println!("Hello, world!");
}
