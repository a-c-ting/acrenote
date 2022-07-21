// #![allow(dead_code)]
// #![allow(unused)]
//temp flag ends here

/*
 * Each string in the 'words' vec contains possible ways to write the word.
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

#[derive(Clone, Debug, PartialEq)]
pub struct EntryWords(pub u64, pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct EntryElements(pub Option<u64>, pub String);

#[derive(Debug, PartialEq)]
pub struct DictEntry {
    words : Vec<EntryWords>,
    readings : Vec<EntryElements>,
    desc : Vec<EntryElements>,
    notes : Vec<EntryElements>,
}

impl DictEntry {
    pub fn create_entry(
        words : Vec<EntryWords>, //TODO: change vec to hashmap (or set)
        readings : Vec<EntryElements>,
        desc: Vec<EntryElements>,
        notes: Vec<EntryElements>)
    -> Result<DictEntry, LdError> {
        let mut word_identifier_list: HashSet<u64> = HashSet::new();

        for entry in &words {
            if word_identifier_list.contains(&entry.0) {
                return Err(LdError::EntryError);
            }

            if entry.1.is_empty() {
                return Err(LdError::EmptyWordError);
            }

            word_identifier_list.insert(entry.0);
        }

        Ok(DictEntry {
            words,
            readings,
            desc,
            notes,
        })
    }

    // pub fn submit_entry() {
        //format entry to send to database
    // }

}

//chapters
#[derive(Debug)]
pub struct Chapters {
    local_id_ctr: u64,
    entry_mapping: BTreeMap<u64, DictEntry>, //local_id, DictEntry
    chapter_lists: BTreeMap<(u64, u64), Vec<u64>>, //(book_id, chapter_id), Vec of local_id in that chapter
    _book_titles: BTreeMap<u64, String>, //bookId, book_titles
    _chapter_titles: BTreeMap<u64, BTreeMap<u64, String>>, //bookId -> chapterId, chapter title
}

impl Chapters {
    pub fn new() -> Chapters {
        Chapters {
            local_id_ctr: 0,
            entry_mapping: BTreeMap::new(),
            chapter_lists: BTreeMap::new(),
            _book_titles: BTreeMap::new(),
            _chapter_titles: BTreeMap::new(),
        }
    }

    pub fn add_entry(&mut self, entry: DictEntry, book_id: u64, chapter_id: u64) {
        self.entry_mapping.insert(self.local_id_ctr, entry);

        self.chapter_lists.entry((book_id, chapter_id))
            .or_insert(Vec::new())
            .push(self.local_id_ctr);

        //TODO: add titles handling
        self.local_id_ctr += 1;
    }

    //stdout only
    pub fn db_view_book(&self, target_book_id: u64, target_chapter_id: Option<u64>) {
        let mut display_list = Vec::new();
        for ((book_id, chapter_id), id_list) in &self.chapter_lists {
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

        for loc_id in display_list {
            println!("{:?}", &self.entry_mapping[&loc_id]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dict_entry_create_should_return_entry() {
        let output = DictEntry::create_entry(
            vec!(EntryWords(0, "Word1".to_string()),
                EntryWords(1, "Word2".to_string())),
            vec!(EntryElements(None,"Reading1".to_string())),
            vec!(EntryElements(Some(1),"Desc1".to_string())),
            vec!(EntryElements(Some(2),"Note1".to_string())));

        assert!(output.is_ok());

        let expected = DictEntry {
            words: vec!(EntryWords(0, "Word1".to_string()),
                EntryWords(1, "Word2".to_string())),
            readings: vec!(EntryElements(None,"Reading1".to_string())),
            desc: vec!(EntryElements(Some(1),"Desc1".to_string())),
            notes: vec!(EntryElements(Some(2),"Note1".to_string()))
        };

        assert_eq!(output.unwrap(), expected);
    }

    #[test]
    fn dict_entry_create_should_return_err_on_repeated_word_id() {
        let output = DictEntry::create_entry(
            vec!(EntryWords(0, "Word1".to_string()),
                EntryWords(0, "Word2".to_string())),
            vec!(EntryElements(None,"Reading1".to_string())),
            vec!(EntryElements(Some(1),"Desc1".to_string())),
            vec!(EntryElements(Some(2),"Note1".to_string())));

        assert_eq!(output, Err(LdError::EntryError));
    }

    #[test]
    fn dict_entry_create_should_return_err_on_empty_word() {
        let output = DictEntry::create_entry(
            vec!(EntryWords(1, "Word1".to_string()),
               EntryWords(2, "".to_string())),
            vec!(EntryElements(None,"Reading1".to_string())),
            vec!(EntryElements(Some(1),"Desc1".to_string())),
            vec!(EntryElements(Some(2),"Note1".to_string())));

        assert_eq!(output, Err(LdError::EmptyWordError));
    }

    #[test]
    fn chapters_new() {
        let test = Chapters::new();
        assert_eq!(test.local_id_ctr, 0);
    }

    #[test]
    fn chapters_add_entry_should_put_correct_local_id_per_bookchapter() {
        let entry_A = DictEntry::create_entry(
            vec!(EntryWords(1, "A Word1".to_string()),
                EntryWords(2, "A Word2".to_string())),
            vec!(EntryElements(None,"A Reading1".to_string())),
            vec!(EntryElements(None,"A Desc1".to_string())),
            vec!(EntryElements(None,"A Note1".to_string())));

        let entry_B = DictEntry::create_entry(
            vec!(EntryWords(1, "B Word1".to_string()),
                EntryWords(2, "B Word2".to_string())),
            vec!(EntryElements(None,"B Reading1".to_string())),
            vec!(EntryElements(None,"B Desc1".to_string())),
            vec!(EntryElements(None,"B Note1".to_string())));

        let entry_C = DictEntry::create_entry(
            vec!(EntryWords(1, "C Word1".to_string()),
                EntryWords(2, "C Word2".to_string())),
            vec!(EntryElements(None,"C Reading1".to_string())),
            vec!(EntryElements(None,"C Desc1".to_string())),
            vec!(EntryElements(None,"C Note1".to_string())));

        let entry_D = DictEntry::create_entry(
            vec!(EntryWords(1, "D Word1".to_string()),
                EntryWords(2, "D Word2".to_string())),
            vec!(EntryElements(None,"D Reading1".to_string())),
            vec!(EntryElements(None,"D Desc1".to_string())),
            vec!(EntryElements(None,"D Note1".to_string())));

        let entry_E = DictEntry::create_entry(
            vec!(EntryWords(1, "E Word1".to_string()),
                EntryWords(2, "E Word2".to_string())),
            vec!(EntryElements(None,"E Reading1".to_string())),
            vec!(EntryElements(None,"E Desc1".to_string())),
            vec!(EntryElements(None,"E Note1".to_string())));

        let mut test = Chapters::new();

        test.add_entry(entry_A.unwrap(), 1, 1);
        test.add_entry(entry_B.unwrap(), 1, 2);
        test.add_entry(entry_D.unwrap(), 2, 1);
        test.add_entry(entry_C.unwrap(), 1, 3);
        test.add_entry(entry_E.unwrap(), 1, 1);

        let mut expected: BTreeMap<(u64, u64), Vec<u64>> = BTreeMap::new();

        expected.insert((1,1), vec![0,4]);
        expected.insert((1,2), vec![1]);
        expected.insert((1,3), vec![3]);
        expected.insert((2,1), vec![2]);

        assert_eq!(test.chapter_lists, expected);
    }
}

