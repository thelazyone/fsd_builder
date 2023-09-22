use super::element::Element;

#[derive(Clone, PartialEq)]
pub struct Support {
    pub name : String,
    pub points : u32,
}

impl Element for Support {
    fn get_name (&self) -> String {
        self.name.clone()
    }

    fn get_points (&self) -> u32 {
        self.points
    }
}