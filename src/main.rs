// #![allow(dead_code)]
#![allow(non_snake_case)]
//temp flag ends here

mod errors;
mod local_dict;

use local_dict::Chapters;
use local_dict::DictEntry;
use local_dict::EntryElements;

use crate::local_dict::EntryWords;

fn main() -> Result<(), errors::LdError> {
    //shameful testing stuff

    let entry_A = DictEntry::create_entry(
                    // 0,
                    vec!(EntryWords(1, "A Word1".to_string()),
                        EntryWords(2, "A Word2".to_string())),
                    vec!(EntryElements(None,"A Reading1".to_string())),
                    vec!(EntryElements(None,"A Desc1".to_string())),
                    vec!(EntryElements(None,"A Note1".to_string())))?;
    let entry_B = DictEntry::create_entry(
                    // 0,
                    vec!(EntryWords(1, "B Word1".to_string()),
                        EntryWords(2, "B Word2".to_string())),
                    vec!(EntryElements(None,"B Reading1".to_string())),
                    vec!(EntryElements(None,"B Desc1".to_string())),
                    vec!(EntryElements(None,"B Note1".to_string())))?;
    let entry_C = DictEntry::create_entry(
                    // 0,
                    vec!(EntryWords(1, "C Word1".to_string()),
                        EntryWords(2, "C Word2".to_string())),
                    vec!(EntryElements(None,"C Reading1".to_string())),
                    vec!(EntryElements(None,"C Desc1".to_string())),
                    vec!(EntryElements(None,"C Note1".to_string())))?;
    let entry_D = DictEntry::create_entry(
                    // 0,
                    vec!(EntryWords(1, "D Word1".to_string()),
                        EntryWords(2, "D Word2".to_string())),
                    vec!(EntryElements(None,"D Reading1".to_string())),
                    vec!(EntryElements(None,"D Desc1".to_string())),
                    vec!(EntryElements(None,"D Note1".to_string())))?;
    let entry_E = DictEntry::create_entry(
                    // 0,
                    vec!(EntryWords(1, "E Word1".to_string()),
                        EntryWords(2, "E Word2".to_string())),
                    vec!(EntryElements(None,"E Reading1".to_string())),
                    vec!(EntryElements(None,"E Desc1".to_string())),
                    vec!(EntryElements(None,"E Note1".to_string())))?;

    let mut newbook = Chapters::new();

    newbook.add_entry(entry_A, 1, 1);
    newbook.add_entry(entry_B, 1, 2);
    newbook.add_entry(entry_C, 1, 3);
    newbook.add_entry(entry_D, 2, 1);
    newbook.add_entry(entry_E, 2, 2);

    println!("Book 1 Chapter 2");
    newbook.db_view_book(1, Some(2));
    println!();
    println!("Book 1 All Chapters");
    newbook.db_view_book(1, None);

    Ok(())
}
