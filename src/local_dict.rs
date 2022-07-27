// #![allow(dead_code)]
#![allow(unused)]
//temp flag ends here

/*
 * Words (might be better to rename to word_pool later)
 * Each string in the 'words' contains possible ways to write the word.
 * i.e. colour vs color, 着替え vs 着換え
 *
 * Each item in 'words' also accompanies a unique identifying number within the entry.
 * Word's identifying number cannot be "None".
 *
 * Identifying numbers:
 * Readings/desc/notes MUST have an attached identifying number.
 * "None" is used for general description that applies to all/most of the words pool.
 * If a non-zero number is used, then those will get attached to the unique word with that number.
 *
 * Readings may contains IPA, pinyin (for Chinese), or yomikata (for Japanese)
 *
 * Desc is description
 *
 * Note is currently miscellaneous stuff that may be added to it. Example below.
 *
 * (courtesy of wiktionary)
 * "Colour"
 * "Australia, Canada, Ireland, New Zealand, South Africa and UK standard spelling of color."
 *
 * We can add this note under "colour" but not "color"
 */

use crate::errors::LdError;
use std::collections::{BTreeMap, HashSet};
use serde::{Serialize, Deserialize};
// use serde_json;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[derive(Debug)]
pub struct EntryWord {
    pub loc_id: u64,
    pub word: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[derive(Debug)]
pub struct EntryElement {
    pub entry_id: Option<u64>,
    pub elem: String,
}

//Dictionary Entries
#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[derive(Debug)]
pub struct DictEntry {
    words : Vec<EntryWord>,
    readings : Vec<EntryElement>,
    desc : Vec<EntryElement>,
    notes : Vec<EntryElement>,
}

impl DictEntry {
    pub fn create_entry(
        words : Vec<EntryWord>, //TODO: change vec to hashmap (or set)
        readings : Vec<EntryElement>,
        desc: Vec<EntryElement>,
        notes: Vec<EntryElement>)
    -> Result<DictEntry, LdError> {
        let mut word_identifier_list: HashSet<u64> = HashSet::new();

        for entry in &words {
            if word_identifier_list.contains(&entry.loc_id) {
                return Err(LdError::RepeatingWordId);
            }

            if entry.word.is_empty() {
                return Err(LdError::EmptyWord);
            }

            word_identifier_list.insert(entry.loc_id);
        }

        Ok(DictEntry {
            words,
            readings,
            desc,
            notes,
        })
    }
}

//chapters
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct BookLibrary {
    local_id_ctr: u64,
    entry_mapping: BTreeMap<u64, DictEntry>, //local_id, DictEntry
    entry_list_by_chapter: BTreeMap<(u64, u64), Vec<u64>>, //(book_id, chapter_id), vec<local_id>
    book_titles: BTreeMap<u64, String>, //bookId, book_titles
    chapter_titles: BTreeMap<u64, BTreeMap<u64, String>>, //bookId -> chapterId, chapter title
}

impl BookLibrary {
    pub fn new() -> BookLibrary {
        BookLibrary {
            local_id_ctr: 0,
            entry_mapping: BTreeMap::new(),
            entry_list_by_chapter: BTreeMap::new(),
            book_titles: BTreeMap::new(),
            chapter_titles: BTreeMap::new(),
        }
    }

    pub fn add_entry(&mut self, entry: DictEntry, book_id: u64, chapter_id: u64) {
        self.entry_mapping.insert(self.local_id_ctr, entry);

        self.entry_list_by_chapter.entry((book_id, chapter_id))
            .or_insert(Vec::new())
            .push(self.local_id_ctr);

        self.local_id_ctr += 1;
    }

    pub fn add_book_title(&mut self, book_id: u64, book_title: String) {
        self.book_titles.entry(book_id)
            .or_insert(book_title);

        //init chapters
        self.chapter_titles.entry(book_id)
            .or_insert(BTreeMap::new());
    }

    pub fn get_book_title(&self, book_id: u64) -> Option<&String>{
        self.book_titles
            .get(&book_id)
    }

    pub fn get_chapter_title(&self, book_id: u64, chapter_id: u64) -> Option<&String> {
        if let Some(map) = self.chapter_titles.get(&book_id) {
            return map.get(&chapter_id);
        }
        None
    }

    pub fn add_chapter_title(&mut self, book_id: u64, chapter_id: u64, chapter_title: String)
            -> Result<(), LdError> {
        if let Some(map) = self.chapter_titles.get_mut(&book_id) {
            map.entry(chapter_id).or_insert(chapter_title);

            Ok(())
        } else {
            Err(LdError::BookDoesNotExist)
        }
    }

    //stdout only
    //for dev purposes
    pub fn db_view_book(&self, target_book_id: u64, target_chapter_id: Option<u64>) {
        let mut display_list = Vec::new();
        for ((book_id, chapter_id), id_list) in &self.entry_list_by_chapter {
            match target_chapter_id {
                Some(t_ch_id) => {
                    if *book_id == target_book_id &&
                    *chapter_id == t_ch_id {
                        display_list.append(&mut id_list.clone());
                    }

                },
                None => {
                    if *book_id == target_book_id {
                        display_list.append(&mut id_list.clone());
                    }
                },
            }
        }

        let b_title = self.book_titles.get(&target_book_id);
        let c_map = self.chapter_titles.get(&target_book_id);
        match (b_title, target_chapter_id, c_map) {
            (None, _, _) => {
                println!("Book title is missing.");
            },
            (Some(b_title), None, _) => {
                println!("Book title: {}", b_title);
            },
            (Some(b_title), Some(c_id), Some(c_map)) => {
                println!("Book title: {}", b_title);
                if let Some(c_title) = c_map.get(&c_id) {
                    println!("Chapter title: {}", c_title);
                } else {
                    println!("Chapter title missing.");
                }
            },
            (Some(_), Some(_), None) => unreachable!(),
        }

        for id in display_list {
            println!("{:?}", &self.entry_mapping[&id]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dict_entry_create_should_return_entry() {
        let output = DictEntry::create_entry(
            vec!(EntryWord{ loc_id: 0, word: "Word1".to_string() },
                EntryWord{ loc_id: 1, word: "Word2".to_string() }),
            vec!(EntryElement{ entry_id: None, elem: "Reading1".to_string() }),
            vec!(EntryElement{ entry_id: Some(1), elem: "Desc1".to_string()} ),
            vec!(EntryElement{ entry_id: Some(2), elem: "Note1".to_string()} ));

        assert!(output.is_ok());

        let expected = DictEntry {
            words: vec!(EntryWord{ loc_id: 0, word: "Word1".to_string()},
                EntryWord{ loc_id: 1, word: "Word2".to_string()}),
            readings: vec!(EntryElement{ entry_id: None, elem: "Reading1".to_string()}),
            desc: vec!(EntryElement{ entry_id: Some(1), elem: "Desc1".to_string()}),
            notes: vec!(EntryElement{ entry_id: Some(2), elem: "Note1".to_string()})
        };

        assert_eq!(output.unwrap(), expected);
    }

    #[test]
    fn dict_entry_create_should_return_err_on_repeated_word_id() {
        let output = DictEntry::create_entry(
            vec!(EntryWord{ loc_id: 0, word: "Word1".to_string()},
                EntryWord{ loc_id: 0, word: "Word2".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "Reading1".to_string()}),
            vec!(EntryElement{ entry_id: Some(1), elem: "Desc1".to_string()}),
            vec!(EntryElement{ entry_id: Some(2), elem: "Note1".to_string()}));

        assert_eq!(output, Err(LdError::RepeatingWordId));
    }

    #[test]
    fn dict_entry_create_should_return_err_on_empty_word() {
        let output = DictEntry::create_entry(
            vec!(EntryWord{ loc_id: 1, word: "Word1".to_string()},
               EntryWord{ loc_id: 2, word: "".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "Reading1".to_string()}),
            vec!(EntryElement{ entry_id: Some(1), elem: "Desc1".to_string()}),
            vec!(EntryElement{ entry_id: Some(2), elem: "Note1".to_string()}));

        assert_eq!(output, Err(LdError::EmptyWord));
    }

    #[test]
    fn chapters_new_correct_init() {
        let test = BookLibrary::new();
        assert_eq!(test.local_id_ctr, 0);
    }

    #[test]
    fn chapters_add_get_titles() {
        let mut test = BookLibrary::new();
        let test_book_id = 1;
        let test_book_title = String::from("book title 1337");
        let test_chapter_id = 4;
        let test_chapter_title = String::from("chapter title leet");

        test.add_book_title(test_book_id,
            test_book_title.clone());
        let res = test.add_chapter_title(test_book_id,
            test_chapter_id,
            test_chapter_title.clone());

        assert!(res.is_ok());
        assert_eq!(test_book_title,
            *test.get_book_title(test_book_id).unwrap());
        assert_eq!(test_chapter_title,
            *test.get_chapter_title(test_book_id, test_chapter_id).unwrap());
    }

    #[test]
    fn chapters_add_entry_should_put_correct_local_id_per_bookchapter() {
        let entry_a = DictEntry::create_entry(
            vec!(EntryWord{ loc_id: 1, word: "A Word1".to_string()},
                EntryWord{ loc_id: 2, word: "A Word2".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "A Reading1".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "A Desc1".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "A Note1".to_string()}));

        let entry_b = DictEntry::create_entry(
            vec!(EntryWord{ loc_id: 1, word: "B Word1".to_string()},
                EntryWord{ loc_id: 2, word: "B Word2".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "B Reading1".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "B Desc1".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "B Note1".to_string()}));

        let entry_c = DictEntry::create_entry(
            vec!(EntryWord{ loc_id: 1, word: "C Word1".to_string()},
                EntryWord{ loc_id: 2, word: "C Word2".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "C Reading1".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "C Desc1".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "C Note1".to_string()}));

        let entry_d = DictEntry::create_entry(
            vec!(EntryWord{ loc_id: 1, word: "D Word1".to_string()},
                EntryWord{ loc_id: 2, word: "D Word2".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "D Reading1".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "D Desc1".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "D Note1".to_string()}));

        let entry_e = DictEntry::create_entry(
            vec!(EntryWord{ loc_id: 1, word: "E Word1".to_string()},
                EntryWord{ loc_id: 2, word: "E Word2".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "E Reading1".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "E Desc1".to_string()}),
            vec!(EntryElement{ entry_id: None, elem: "E Note1".to_string()}));

        let mut test = BookLibrary::new();

        test.add_entry(entry_a.unwrap(), 1, 1);
        test.add_entry(entry_b.unwrap(), 1, 2);
        test.add_entry(entry_d.unwrap(), 2, 1);
        test.add_entry(entry_c.unwrap(), 1, 3);
        test.add_entry(entry_e.unwrap(), 1, 1);

        let mut expected: BTreeMap<(u64, u64), Vec<u64>> = BTreeMap::new();

        expected.insert((1,1), vec![0,4]);
        expected.insert((1,2), vec![1]);
        expected.insert((1,3), vec![3]);
        expected.insert((2,1), vec![2]);

        assert_eq!(test.entry_list_by_chapter, expected);
    }
}

