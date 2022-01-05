use parity_wordlist::random_phrase;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(default_value="1")]
    word_count: usize,
    #[structopt(long, short)]
    with_hyphens: bool,
}

fn main() {
   let options = Options::from_args(); 
   let result = match options.with_hyphens {
       false => random_phrase(options.word_count),
       true => {
           let words = random_phrase(options.word_count);
           str::replace(&words, " ", "-")
       },
   };
    println!("{:?}", result);
}
