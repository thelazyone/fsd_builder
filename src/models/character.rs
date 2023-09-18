use super::element::Element;

pub struct Character {
    pub name : String,
    pub points : u32,
}

impl Element for Character {
    fn get_name (&self) -> String {
        self.name.clone()
    }

    fn get_points (&self) -> u32 {
        self.points
    }
}