
#[derive(Debug)]
enum Media {
    Book{title: String, author:String},
    Movie{title:String, director:String},
    Audiobook{title:String},
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String{
        match self {
            Media::Audiobook { title } => format!("Audiobook: {}", title),
            Media::Movie { title, director } => format!("Movie: {} {}", title, director),
            Media::Book { title, author } => format!("Book: {} {}", title, author),
            Media::Podcast(id) => format!("Podcast: {}", id),
            Media::Placeholder => format!("Placeholder")
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items : Vec<Media>
}

impl Catalog {
    fn new() -> Self {
        Catalog{items: vec![]}
    }

    fn add(&mut self, media:Media) {
        self.items.push(media);
    } 
}



fn print_media(media:Media){
    println!("{:#?}", media)
}
fn main() {
   let auditobook = Media::Audiobook { title: String::from("An Audio Book") };
   let good_movie = Media::Movie { title: String::from("A Good Movie"), director: String::from("Good Director") };
   let book = Media::Book { title: String::from("Bad Book"), author: String::from("Bad Author") };
   let podcast = Media::Podcast(10);
   let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();

    catalog.add(auditobook);
    catalog.add(good_movie);
    catalog.add(book);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("{:#?}", catalog);
}
