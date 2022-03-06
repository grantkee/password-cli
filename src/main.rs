use parity_wordlist::random_phrase;
use structopt::StructOpt;
use rand::Rng;

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(default_value="1")]
    word_count: usize,
    #[structopt(long, short)]
    password: bool,
}

fn main() {
   let options = Options::from_args(); 
   let result = match options.password {
       false => random_phrase(options.word_count),
       true => {
           let mut rng = rand::thread_rng();
           let words = random_phrase(options.word_count);
           let mut pword= String::new();

           for c in words.chars() {
               let up = rng.gen();
               if up {
                   pword.push(c);
                } else {
                    // to_uppercase returns an array of chars
                    for c in c.to_uppercase() {
                        pword.push(c);
                    }
                }
           }
           let num = rand::thread_rng().gen_range(0..9999999).to_string();
           pword.push_str(" ");
           pword.push_str(&num);
           str::replace(&pword, " ", "-")
       },
   };
    println!("{:?}", result);
}
