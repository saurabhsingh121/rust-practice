
#[derive(Debug)]
enum Media {
    Book{title: String, author:String},
    Movie{title:String, director:String},
    Audiobook{title:String},
}

fn print_media(media:Media){
    println!("{:#?}", media)
}
fn main() {
   let auditobook = Media::Audiobook { title: String::from("An Audio Book") };
   let good_movie = Media::Movie { title: String::from("A Good Movie"), director: String::from("Good Director") };
   let book = Media::Book { title: String::from("Bad Book"), author: String::from("Bad Author") };
   print_media(auditobook);
   print_media(good_movie);
   print_media(book);
}
