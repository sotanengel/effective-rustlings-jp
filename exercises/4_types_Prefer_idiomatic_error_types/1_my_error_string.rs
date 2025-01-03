// TODO: StringはErrorを実装していないため、formatが使えるように独自の型を定義し、Errorを実装してください。
fn do_something(flag: bool) -> Result<(), String> {
    if flag {
        Ok(())
    } else {
        String::from("Something went wrong")
    }
}

fn main() {
    match do_something(false) {
        Ok(_) => println!("Success!"),
        Err(e) => {
            let error_message = format!("Failed: {}", e);
            println!("{}", error_message);
        }
    }
}
