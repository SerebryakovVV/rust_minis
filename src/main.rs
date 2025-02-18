use rusqlite as rsql;


#[derive(Debug)]
struct User {
    name: String
}

fn main() -> rsql::Result<()> {

    // let path = "tasks.db3";
    // let db = match rsql::Connection::open(path) {
    //     Err(_) => panic!(),
    //     Ok(con) => con
    // };

    
    // db.execute("DROP TABLE users", ())?;

    // let query_result = match db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY,name  TEXT NOT NULL)", ()) {
    //     Err(e) => {println!("{}", e); panic!("first");},
    //     Ok(r) => r 
    // };

    // let query_result = match db.execute("insert into users (name) values ('valentin')", ()) {
    //     Err(e) => {println!("{}", e); panic!("first");},
    //     Ok(r) => r 
    // };

    // let mut statement = db.prepare("select name from users")?;
    // let query_result = statement.query_map([], |el| {
    //     Ok(User{name:el.get(0)?})
    // })?;

  

    // for u in query_result {
    //     println!("{:?}", u);
        
    // }

    println!("Hello, world!");

    Ok(())
}
