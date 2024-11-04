#[derive(Debug)]
pub enum Media {
    Book{title: String, author:String},
    Movie{title:String, director:String},
    Audiobook{title:String},
    Podcast(u32),
    Placeholder,
}

impl Media {
    pub fn description(&self) -> String{
        match self {
            Media::Audiobook { title } => format!("Audiobook: {}", title),
            Media::Movie { title, director } => format!("Movie: {} {}", title, director),
            Media::Book { title, author } => format!("Book: {} {}", title, author),
            Media::Podcast(id) => format!("Podcast: {}", id),
            Media::Placeholder => format!("Placeholder")
        }
    }
}