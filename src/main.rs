use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    age: u32,
    email: String,
}


fn main() {
    let json_string = r#"
    {
        "name": "John Doe",
        "age": 30,
        "email": "john.doe@example.com"
    }
    "#;

    let user: User = serde_json::from_str(json_string).unwrap();

    println!("{:?}", user);

    let user = User {
        name: "John Doe".to_string(),
        age: 30,
        email: "john.doe@example.com".to_string(),
    };

    let json_string = serde_json::to_string(&user).unwrap();

    println!("{}", json_string);
}
