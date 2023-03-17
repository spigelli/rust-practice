#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    email: String,
    id: u32,
    image: Option<String>,
}

impl User {

    fn profile_card(&self) -> String {
        let image = match &self.image {
            Some(image) => format!("<img src=\"{}\" alt=\"{}\" />", image, self.first_name),
            None => String::from(""),
        };
        format!("
            <div class=\"profile-card\">
                <div class=\"profile-card__image\">
                    {}
                </div>
                <div class=\"profile-card__info\">
                    <h2 class=\"profile-card__name\">{} {}</h2>
                    <p class=\"profile-card__email\">{}</p>
                </div>
            </div>
        ", image, self.first_name, self.last_name, self.email)
    }
    fn copy(&self) -> User {
        User {
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            id: self.id,
            image: self.image.clone(),
        }
    }
}

fn test_user() {
    let jeremy = User {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        email: String::from("johndoe@asdf.com"),
        id: 1,
        image: Some(String::from("https://via.placeholder.com/150")),
    };
    // Debug
    dbg!(jeremy.copy());
    println!("{}", jeremy.profile_card());
}

fn main() {
    println!("Hello, world!");
    test_user();
}