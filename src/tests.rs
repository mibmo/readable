mod word_count {
    use crate::word_count;

    #[test]
    fn empty_string() {
        assert_eq!(word_count(""), 0);
    }

    #[test]
    fn only_spaces() {
        assert_eq!(word_count(" "), 0);
        assert_eq!(word_count("  "), 0);
    }

    #[test]
    fn simple() {
        assert_eq!(word_count("hello"), 1);
        assert_eq!(word_count("hello world"), 2);
    }

    #[test]
    fn trailing() {
        assert_eq!(word_count("hello world "), 2);
        assert_eq!(word_count("hello world  "), 2);
        assert_eq!(word_count("hello world   "), 2);
    }

    #[test]
    fn leading() {
        assert_eq!(word_count(" hello world"), 2);
        assert_eq!(word_count("  hello world"), 2);
        assert_eq!(word_count("   hello world"), 2);
    }

    #[test]
    fn surrounded() {
        assert_eq!(word_count(" hello world "), 2);
        assert_eq!(word_count("  hello world  "), 2);
        assert_eq!(word_count("   hello world   "), 2);
    }
}

mod sentence_count {
    use crate::sentence_count;

    #[test]
    fn empty_string() {
        assert_eq!(sentence_count(""), 1);
    }

    #[test]
    fn only_spaces() {
        assert_eq!(sentence_count(" "), 1);
        assert_eq!(sentence_count("  "), 1);
        assert_eq!(sentence_count(" . "), 1);
        assert_eq!(sentence_count("  .  "), 1);
        assert_eq!(sentence_count(" .  . "), 1);
    }

    #[test]
    fn simple() {
        assert_eq!(sentence_count("hello"), 1);
        assert_eq!(sentence_count(" hello "), 1);
        assert_eq!(sentence_count("hello . world "), 2);
        assert_eq!(sentence_count(" hello . world "), 2);

        assert_eq!(sentence_count("hello . world ."), 2);
        assert_eq!(sentence_count(" hello . world . "), 2);
    }
}

mod gunning_fog {
    use crate::gunning_fog;

    #[test]
    fn empty_string() {
        assert_eq!(gunning_fog(""), 0.0);
    }

    #[test]
    fn simple() {
        assert!((gunning_fog("The fog index is commonly used to confirm that text can be read easily by the intended audience. Texts for a wide audience generally need a fog index less than 12. Texts requiring near-universal understanding generally need an index less than 8.") - 14.17).abs() <= 0.1);
    }
}

mod flesch_kincaid_grade {
    use crate::flesch_kincaid_grade;

    #[test]
    fn empty_string() {
        assert_eq!(flesch_kincaid_grade(""), 0.0);
    }

    #[test]
    fn simple() {
        assert!((flesch_kincaid_grade("The Flesch–Kincaid readability tests are readability tests designed to indicate how difficult a passage in English is to understand. There are two tests, the Flesch Reading-Ease, and the Flesch–Kincaid Grade Level. Although they use the same core measures (word length and sentence length), they have different weighting factors.
The results of the two tests correlate approximately inversely: a text with a comparatively high score on the Reading Ease test should have a lower score on the Grade-Level test. Rudolf Flesch devised the Reading Ease evaluation; somewhat later, he and J. Peter Kincaid developed the Grade Level evaluation for the United States Navy.") - 11.2).abs() <= 0.1);
    }
}
