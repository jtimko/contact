#[warn(unused_must_use)]
struct Contact {
    name: String,
    phone: String,
}

struct User {
    name: String,
    email: String,
    phone: String,
    // _g_long: String,
    // _g_lat: String,
    contacts: Vec<Contact>,
}


impl User {
    fn get_name(&self) {
        println!("{}", self.name);
    }
    fn contact_list(&self) {
        for user in &self.contacts {
            println!("{} - {}", user.name, user.phone);
        }
    }
}

fn main() {
    
    let mut user = User {
        name: String::from("John Doe"),
        email: String::from("jdoe@example.com"),
        phone: String::from("777-777-7777"),
        contacts: Vec::new(),
    };
    user.contacts.push(Contact {
        name: String::from("Jane Doe"),
        phone: String::from("888-888-8888"),
    });
    user.contacts.push(Contact {
        name: String::from("James Bond"),
        phone: String::from("007-007-0007"),
    });

    user.contact_list();

}
