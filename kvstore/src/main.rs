// To get started, first install rust. 
// IDE: VS Code
// Additional Tools to install within VS Code: rust-analyzer

// To initialize project:
//$cargo new <project_name>
// To run code with args: 
//$cargo run -- arg1 arg2
// To run code without args: 
//$cargo run
// To check syntax:
//$cargo check
// To build:
//$cargo build 

use std::collections::HashMap;

// () mean unit, and it is equivalent to void in c++
fn main() ->Result<(), std::io::Error> {
    let mut arguments = std::env::args().skip(1);
    /*
    // For loop syntax
    for arg in arguments {
        println!("Got agr: {}", arg);
    }
    */
    // .unwrap() is not a good way to handle Option<String>. It will panic: crash the program
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    let mut db = Database::from_disk()?;
    // ! means it is a macro
    println!("Key: {}, Value: {}", key, value);
    //let db_borrow_mut = &mut db;
    db.insert(key, value);
    // We lent db to .insert(), since we use & (borrow), 
    // it gave back to us which allows us to use it subsequently
    //db.flush()?;

    // The following line will cause issues because cannot mutably borrow at the same time
    //db_borrow_mut.insert(String::new(), String::new());
    
    // drop takes the ownership of the object and drop it (free memory, close file io, etc.)
    //drop(db);
    Ok(())
/*
// Custom error handling if not sepcifying return for main()
    let write_result = write_database(key, value);
    match write_result {
        Ok(()) => {
            println!("It worked!")
        }
        Err(the_error) => {
            println!("We got an error {}", the_error)
        }
    }
*/
}

struct Database {
    hashmap: HashMap<String, String>,
}

// Add function and method associated with struct
impl Database {
    fn from_disk() -> Result<Database, std::io::Error> {
        let contents = std::fs::read_to_string("kv.db")?;
        // The following can be replace by ?
        // let contents = match contents {
        //     Ok(contents_string) => contents_string,
        //     Err(erro_value) => return Err(erro_value),
        // };
        let mut hashmap = HashMap::new();
        for line in contents.lines() {
            // mut is needed because we use .next() that mutates chunks
            let mut chunks = line.split('\t');
            let key = chunks.next().unwrap();
            let value = chunks.next().unwrap();
            // .to_string() turns &str to String
            hashmap.insert(key.to_string(), value.to_string());
        }
        Ok(Database { hashmap: hashmap })
    }

    // need to add a receiver of our method: self
    // need to make self mutable to call hashmap.insert()
    // & means we are borrowing
    fn insert(&mut self, key: String, value: String) {
        // todo! means it is working in progress
        //todo!("Implement")
        self.hashmap.insert(key, value);
    }

    // std::io::Result<()> is equivalent to Result<Database, std::io::Error>
    fn flush(&self) -> std::io::Result<()> {
        let content: String = todo!("Format the keys and values as a string");
        // the last line without ; means it will return 
        std::fs::write("kv.db", content)
    }

    // Important lession: we could have as many immutable borrow as possible
    // We could only have one mutable borrow at a time
    // Mutable borrow is exclusive borrow (across threads)
    // immutable borrow is confirming nobody is writing this
    // There is no need to do lock
    // This is what makes rust good
    // Baked in one exclusive writer and one reader
    // Remove data race, remove a bounch of other issues
}

/*
fn write_database(key: String, value: String) -> Result<(), std::io::Error> {
    let content = format!("{}\t{}", key, value);
    // the last line without ; means it will return 
    std::fs::write("kv.db", content)
}
*/

/*
fn bar() {
    // the ownership now is s3. 
    // The String data memory will be free when bar() goes out of scope
    // Only one variable can own the memory at a time
    let s3 = foo();
}

fn foo() -> String {
    // Memory allocated on the heap when dynamically allocate memory
    let s = String::from("Hello, world");
    // The memory will be freed then it is going out of scope
    // s owns memory allocated the String data
    // now s2 owns the memory allocated
    let s2 = s;
    // The following line is not compilable because it has been moved
    //println!("{}", s);

    s2
}
*/

// Microsoft internal website: aka.ms/rust
// The Rust Programming Language book: doc.rust-lang.org/stable/book/
// Rust by Example: doc.rust-lang/stable/rust-by-example