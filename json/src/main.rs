extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

fn main() {
    // this is stupid, why do i need to use String::from instead of being able to use a string directly?
    // when using 'str' as the type for the struct fields, rust complains that the size must be known at compiletime
    let alex = Person {
        first_name: String::from("alex"),
        last_name: String::from("meier"),
        age: 25,
    };
    let bert = Person {
        first_name: String::from("bert"),
        last_name: String::from("meier"),
        age: 26,
    };
    // now that we have some structs, lets marshall them to json, cant be that hard rigth?
    // great that i need a third party library to marshall something to json -_-
    let json = serde_json::to_string(&[alex, bert]);
    println!("{}", json.unwrap())
    // while that works, i am disappointed that my app now relies on 10 external crates
}
