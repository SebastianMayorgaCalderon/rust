#[derive(Debug, PartialEq)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn desc(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     return format!("Book:{} by {}", title, author);
        // } else if let Media::Movie { title, director } = self {
        //     return format!("Movie:{} by {}", title, director);
        // } else if let Media::AudioBook { title } = self {
        //     return format!("AudioBook:{}", title);
        // } else {
        //     return "No Media type".to_string();
        // }
        //best way to match enums
        match self {
            Media::Book { title, author } => {
                return format!("Book:{} by {}", title, author);
            }
            Media::Movie { title, director } => {
                format!("Movie:{} by {}", title, director)
            }
            Media::AudioBook { title } => format!("AudioBook:{}", title),
            Media::Podcast(ep_number) => format!("Podcast:{}", ep_number,),
            Media::Placeholder => "No Media type".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Catalog {
    media: Vec<Media>,
}

impl Catalog {
    fn new() -> Catalog {
        Catalog { media: Vec::new() }
    }
    fn add(&mut self, media: Media) {
        self.media.push(media);
    }
    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if index <= self.media.len() {
            Some(&self.media[index])
        } else {
            None
        }
    }
}

// fn main() {
//     let audio_book = Media::AudioBook {
//         title: String::from("Tio Pelon"),
//     };
//     let book = Media::Book {
//         title: String::from("Anillos de Poder"),
//         author: String::from("Goro Pelon"),
//     };
//     let movie = Media::Movie {
//         title: String::from("The chenchui"),
//         director: String::from("Goro junior Pelon"),
//     };
//     let podcast = Media::Podcast(10);
//     let placeholder = Media::Placeholder;
//     let mut catalog = Catalog::new();
//     catalog.add(audio_book);
//     catalog.add(book);
//     catalog.add(movie);
//     catalog.add(podcast);
//     catalog.add(placeholder);
//     println!("--------------------------------------------------------");
//     // match catalog.get_by_index(0) {
//     //     Some(media) => println!("{:?}", media),
//     //     None => println!("No value"),
//     // }

//     // if let Some(media) = catalog.get_by_index(30) {
//     //     println!("The value is: {}", media.desc());
//     // }else{
//     //     println!("No value")
//     // }

//     let item: Option<&Media> = catalog.get_by_index(20);
//     let placeholder = Media::Placeholder;

//     println!("{:?}", item.unwrap_or(&placeholder));
//     println!("--------------------------------------------------------");
// }

#[derive(Debug)]
struct Account {
    balance: i32,
}

fn main() {
    let mut accounts: Vec<Account> = vec![/*Account { balance: 0 }, Account { balance: 10 }*/];
    // Add code here:
    let account = accounts.first_mut();
    if let Some(account) = account {
        account.balance += 30;
        println!("Account balance: {:#?}", account);
    } else {
        println!("No account found");
    }
}
