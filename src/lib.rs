#[cfg(test)]
mod tests;

const WORD_DELIMITER_CHARS: &[char] = &[' ', ',', '.'];
const VOWEL_CHARS: &[char] = &['a', 'e', 'i', 'o', 'u'];

/// Iterate over the words in a string
#[derive(Debug)]
struct Words<'a> {
    s: &'a str,
    i: usize,
}

impl<'a> Iterator for Words<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.i + leading_delimiter_chars(&self.s[self.i..]);
        let end = start
            + &self.s[start..]
                .find(WORD_DELIMITER_CHARS)
                .unwrap_or_else(|| self.s.len() - start);

        self.i = end;

        if end > self.s.len() || start >= end {
            None
        } else {
            Some(&self.s[start..end])
        }
    }
}

impl<'a> Words<'a> {
    fn from(s: &'a str) -> Self {
        Self { s, i: 0 }
    }
}

/// Amount of delimiter characters leading the string
#[inline(always)]
fn leading_delimiter_chars(s: &str) -> usize {
    s.chars()
        .take_while(|c| WORD_DELIMITER_CHARS.iter().any(|e| e == c))
        .count()
}

/// Amount of words in a string
#[inline(always)]
fn word_count(s: &str) -> usize {
    Words::from(s).count()
}

// @FIXME: very bad algorithm. really needs to be better for the readability tests to be accurate
/// Approximate syllable count of a word
#[inline(always)]
fn syllable_count(word: &str) -> usize {
    word.chars().filter(|c1| is_vowel(*c1)).count()
}

/// Is the given character a vowel
#[inline(always)]
fn is_vowel(c: char) -> bool {
    VOWEL_CHARS
        .iter()
        .any(|e| e.to_string() == c.to_lowercase().to_string())
}

/// Amount of paragraphs in a string
#[inline(always)]
fn sentence_count(paragraph: &str) -> usize {
    let words = word_count(paragraph);
    let ending_dot = paragraph.trim_end().chars().last() == Some('.');
    if words <= 1 {
        1
    } else {
        (paragraph.chars().filter(|c| *c == '.').count() + if ending_dot { 0 } else { 1 }).max(1)
    }
}

/// Calculate the Gunning Fog index of a paragraph
pub fn gunning_fog(paragraph: &str) -> f32 {
    let (words, complex) = Words::from(paragraph).fold((0.0, 0.0), |(words, complex), word| {
        (
            words + 1.0,
            if syllable_count(word) >= 3 {
                complex + 1.0
            } else {
                complex
            },
        )
    });

    let index = 0.4 * (words / sentence_count(paragraph) as f32 + 100.0 * (complex / words));
    if index.is_nan() {
        0.0
    } else {
        index
    }
}

/// Caluculate the Flesch Kincaid grade level of a paragraph
pub fn flesch_kincaid_grade(paragraph: &str) -> f32 {
    let (total_words, total_syllables) = Words::from(paragraph)
        .fold((0.0, 0.0), |(words, syllables), word| {
            (words + 1.0, syllables + syllable_count(word) as f32)
        });
    let total_sentences = sentence_count(paragraph) as f32;

    let grade =
        (0.39 * (total_words / total_sentences)) + (11.8 * (total_syllables / total_words)) - 15.59;

    println!(
        "total_words: {}\ntotal_syllables: {}\ntotal_sentences: {}\ngrade: {}",
        total_words, total_syllables, total_sentences, grade
    );

    if grade.is_nan() {
        0.0
    } else {
        grade
    }
}

// @TODO: flesch_kincaid_readability_score
