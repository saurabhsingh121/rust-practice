mod content;
use content::{media::Media, catalog::Catalog};
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

    let item = catalog.get_by_index(100);
    let placeholder = Media::Placeholder;
    println!("{:#?}", item.unwrap_or(&placeholder));
}
