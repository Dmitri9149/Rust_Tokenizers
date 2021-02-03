use WordsVector;
use TextStage2;


pub struct Pairs {
    pairs:HashMap<(&str,&str),i32>;
};

impl Pairs {
// make empty Pairs
    pub fn empty() -> Pairs {
        let e = HashMap<_,_>::new;
        Pairs {pairs:e}
    }
    pub fn from_words_vector(self, ww:&WordsVector) -> {
        let mut hsh= HashMap<_,_>::new;
        for word in ww.words {
            let it = word.split_whitespace().peekable();

            while let Some(current) = it.next() {
                if let Some(&next) = it.peek() {
                    let count = self.pairs.entry((current,next)).or_insert(0);
                    *count +=1;          
                }
            }
        }
        Pairs {pairs:self.pairs}
    }

    pub fn from_vocab(self, ww:&TextStage2) -> {
        let mut hsh= HashMap<_,_>::new;
        for key,value in ww.vocab {
            let it = key.split_whitespace().peekable();

            while let Some(current) = it.next() {
                if let Some(&next) = it.peek() {
                    let count = self.pairs.entry((current,next)).or_insert(0);
                    *count +=1;          
                }
            }
        }
        Pairs {pairs:self.pairs}
    }



}

