mod simple_user_input;
use simple_user_input::get_input;

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, ErrorKind, Seek, SeekFrom, Write};

const SET_NAME: &'static str = "Unstable Unicorns Base Set 2nd edition";
const SET_RELEASE_DATE: &'static str = "2019-11-00";
const INPUT_FILE: &'static str = "data/Base_2nd_edition_cards_to_add.txt";
const OUTPUT_CARD_SET_FILE: &'static str = "data/Unstable_Unicorns_2nd_edition_card_set.xml";
const OUTPUT_CARD_DECK_FILE: &'static str = "data/Unstable_Unicorns_2nd_edition_card_libary.txt";

#[derive(Eq, PartialEq)]
enum CardParams {
    Amount = 0,
    CardType = 1,
    Name = 2,
    Image = 3,
    Description = 4,
}
use CardParams::*;

struct InputData {
    input_file: String,
    set_output_file: String,
    deck_output_file: String,
    set_name: String,
    set_release_date: String,
}

impl InputData {
    pub fn new(
        input_file: String,
        set_output_file: String,
        deck_output_file: String,
        set_name: String,
        set_release_date: String,
    ) -> Self {
        Self {
            input_file,
            set_output_file,
            deck_output_file,
            set_name,
            set_release_date,
        }
    }
}

fn show_correct_input_type() {
    println!("Input type not correct!");
    println!("Input file should have lines like this:");
    println!(
        "<amount_in_deck> ; <card_name> ; <card_type> ; <card_image_url> ; <card_description>"
    );
    println!("For card pictures and all info about cards check http://unstablegameswiki.com/index.php?title=Main_Page");
}

fn read_input() -> InputData {
    let mut input = vec![];
    println!("Input file (default: {})", INPUT_FILE);
    input.push(get_input("──> "));

    println!("Set output file (default: {})", OUTPUT_CARD_SET_FILE);
    input.push(get_input("──> "));

    println!("Deck output file (default: {}", OUTPUT_CARD_DECK_FILE);
    input.push(get_input("──> "));

    println!("Set name (default: {})", SET_NAME);
    input.push(get_input("──> "));

    println!("Set release date (default: {})", SET_RELEASE_DATE);
    input.push(get_input("──> "));

    for (num, arg) in input.iter_mut().enumerate() {
        if arg == "" {
            *arg = match num {
                0 => INPUT_FILE.to_owned(),
                1 => OUTPUT_CARD_SET_FILE.to_owned(),
                2 => OUTPUT_CARD_DECK_FILE.to_owned(),
                3 => SET_NAME.to_owned(),
                4 => SET_RELEASE_DATE.to_owned(),
                _ => {
                    show_correct_input_type();
                    panic!("Input not correct!");
                }
            }
        }
    }

    InputData::new(
        input.get(0).unwrap().to_owned(),
        input.get(1).unwrap().to_owned(),
        input.get(2).unwrap().to_owned(),
        input.get(3).unwrap().to_owned(),
        input.get(4).unwrap().to_owned(),
    )
}

fn open_file(path: &str) -> std::fs::File {
    match OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(path)
    {
        Ok(_goes_into_output_file) => _goes_into_output_file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(_file_created_successfully) => _file_created_successfully,
                Err(e) => panic!("Wasn't able to create output file: {:?}", e),
            },
            other_error => panic!("Wasn't able to open the output file: {:?}", other_error),
        },
    }
}

fn main() {
    let input_data = read_input(); // Read input

    let mut input_file = open_file(&input_data.input_file);
    input_file.seek(SeekFrom::Start(0)).unwrap(); // Put cursor at start of file
    let input_file = BufReader::new(input_file);

    let mut set = open_file(&input_data.set_output_file);
    set.seek(SeekFrom::Start(0)).unwrap(); // Put cursor at start of file
    set.set_len(0).unwrap(); // Clears set output file
    let mut set = BufWriter::new(set);

    let mut deck = open_file(&input_data.deck_output_file);
    deck.seek(SeekFrom::Start(0)).unwrap(); // Put cursor at start of file
    deck.set_len(0).unwrap(); // Clears deck output file
    let mut deck = BufWriter::new(deck);

    let set_ini = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<cockatrice_carddatabase version=\"3\">\n\t<sets>\n\t\t<set>
        \t<name>{set_name}</name>
        \t<longname>{set_name}</longname>
        \t<settype>Unstable Unicorns</settype>
        \t<releasedate>{date_of_release}</releasedate>\n\t\t</set>\n\t</sets>\n\t<cards>",
        set_name = input_data.set_name,
        date_of_release = input_data.set_release_date
    );
    writeln!(set, "{}", set_ini).unwrap();

    let mut card_params: Vec<String>;
    for line in input_file.lines() {
        card_params = line
            .unwrap()
            .split(";")
            .map(|s| s.trim().to_owned())
            .collect();

        if card_params.len() != 5 {
            show_correct_input_type();
            break;
        }

        let to_write = format!(
            "<card>
            <name>{name} UU</name>
            <set picURL=\"{image}\"></set>
            <color></color>
            <manacost></manacost>
            <cmc></cmc>
            <maintype>{card_type}\t</maintype>
            <type>{card_type}\t</type>
            <pt></pt>
            <tablerow>0</tablerow>
            <text>{description} </text>
            <font></font>\n\t\t</card>",
            name = card_params.get(Name as usize).unwrap(),
            image = card_params.get(Image as usize).unwrap(),
            card_type = card_params.get(CardType as usize).unwrap(),
            description = card_params.get(Description as usize).unwrap()
        );

        writeln!(set, "\t\t{}", to_write).unwrap();

        writeln!(
            deck,
            "{amount} {card_name} UU",
            amount = card_params.get(Amount as usize).unwrap(),
            card_name = card_params.get(Name as usize).unwrap()
        )
        .unwrap();
    }
    writeln!(set, "\t</cards>\n</cockatrice_carddatabase>").unwrap();

    println!("All done!");
}
