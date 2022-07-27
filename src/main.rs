// #![allow(dead_code)]
// #![allow(non_snake_case)]
#![allow(unused)]
//temp flag ends here

mod errors;
mod local_dict;

use local_dict::BookLibrary;
use local_dict::DictEntry;
use local_dict::EntryElement;

use crate::local_dict::EntryWord;

fn main() -> Result<(), errors::LdError> {
    //shameful (or shameless) dirty testing
    // let entry_A = DictEntry::create_entry(
    //                 // 0,
    //                 vec!(EntryWords(1, "A Word1".to_string()),
    //                     EntryWords(2, "A Word2".to_string())),
    //                 vec!(EntryElements(None,"A Reading1".to_string())),
    //                 vec!(EntryElements(None,"A Desc1".to_string())),
    //                 vec!(EntryElements(None,"A Note1".to_string())))?;
    // let entry_B = DictEntry::create_entry(
    //                 // 0,
    //                 vec!(EntryWords(1, "B Word1".to_string()),
    //                     EntryWords(2, "B Word2".to_string())),
    //                 vec!(EntryElements(None,"B Reading1".to_string())),
    //                 vec!(EntryElements(None,"B Desc1".to_string())),
    //                 vec!(EntryElements(None,"B Note1".to_string())))?;
    // let entry_C = DictEntry::create_entry(
    //                 // 0,
    //                 vec!(EntryWords(1, "C Word1".to_string()),
    //                     EntryWords(2, "C Word2".to_string())),
    //                 vec!(EntryElements(None,"C Reading1".to_string())),
    //                 vec!(EntryElements(None,"C Desc1".to_string())),
    //                 vec!(EntryElements(None,"C Note1".to_string())))?;
    // let entry_D = DictEntry::create_entry(
    //                 // 0,
    //                 vec!(EntryWords(1, "D Word1".to_string()),
    //                     EntryWords(2, "D Word2".to_string())),
    //                 vec!(EntryElements(None,"D Reading1".to_string())),
    //                 vec!(EntryElements(None,"D Desc1".to_string())),
    //                 vec!(EntryElements(None,"D Note1".to_string())))?;
    // let entry_E = DictEntry::create_entry(
    //                 // 0,
    //                 vec!(EntryWords(1, "E Word1".to_string()),
    //                     EntryWords(2, "E Word2".to_string())),
    //                 vec!(EntryElements(None,"E Reading1".to_string())),
    //                 vec!(EntryElements(None,"E Desc1".to_string())),
    //                 vec!(EntryElements(None,"E Note1".to_string())))?;

    // let mut newbook = BookLibrary::new();

    // let entry_Bx = entry_B.clone();
    // newbook.add_entry(entry_A, 1, 1);
    // newbook.add_entry(entry_B, 1, 2);
    // newbook.add_entry(entry_Bx, 1, 2);
    // newbook.add_entry(entry_C, 1, 3);
    // newbook.add_entry(entry_D, 2, 1);
    // newbook.add_entry(entry_E, 2, 2);

    // newbook.add_book_title(1, "Heaven Burns Red".to_string());
    // newbook.add_book_title(1, "shoul not appear".to_string());
    // newbook.add_chapter_title(1, 2, "Burn My Soul".to_string())?;

    // // println!("Book 1 Chapter 2");
    // // newbook.db_view_book(1, Some(2));
    // // println!();
    // // println!("Book 1 All Chapters");
    // // newbook.db_view_book(1, None);
    // // println!();
    // // println!("Book 2 No title");
    // // newbook.db_view_book(2, None);

    // let json_string = serde_json::to_string(&newbook);

    let e_word = EntryWord{ loc_id: 1, word: "entry_world_ser_test".to_string() };

    let e_word_json_string = serde_json::to_string(&e_word).unwrap();
    println!("{}", &e_word_json_string);

    let e_word_struct: EntryWord = serde_json::from_str(&e_word_json_string).unwrap();
    println!("{:#?}", &e_word_struct);

    let e_element = EntryElement{ entry_id: None, elem: "entry_element_ser_test".to_string() };
    let e_element_json_string = serde_json::to_string(&e_element).unwrap();
    println!("{}", &e_element_json_string);

    let e_elem_struct: EntryElement = serde_json::from_str(&e_element_json_string).unwrap();
    println!("{:#?}", e_elem_struct);

    assert!(e_elem_struct.entry_id.is_none());
    println!("assert success");

    Ok(())
}
