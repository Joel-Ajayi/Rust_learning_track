fn fix_order() {
    cook_order();
    super::serve_order();
}
fn cook_order() {}

pub struct Brakefast {
    pub toast: String,
    fruit: String,
}

impl Brakefast {
    pub fn summer(toast: &str) -> Brakefast {
        Brakefast {
            toast: toast.to_string(),
            fruit: "Melon".to_string(),
        }
    }
}
