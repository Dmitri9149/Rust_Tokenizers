use WordsVector;

pub struct Pairs {
    pairs:HashMap<(&str,&str),i32>;
};

impl Pairs {

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
}

/*
=======================================================================
    pub fn separate_punctuation(self, s:&str) -> TextStage1 {
        let mut new_str = String::new();
        
        let mut it = self.text1.chars().peekable();

        while let Some(current) = it.next() {
            if let Some(&next) = it.peek() {
                if current != ' ' &&  s.contains(next) {
                    new_str.push(current);
                    new_str.push(' ');
                }  else { new_str.push(current) }
            }
        }
        TextStage1 {text1: new_str, ..self}
    }


// build vocab: (token, count) as HashMap<String, i32>
// by splitting the 'whole string' on white spaces
//
    pub fn build_vocab_s2(mut self) -> TextStage2 {
        for word in self.text0.split_whitespace() { 
            let count = self.vocab.entry(word.to_string()).or_insert(0);
            *count +=1; 
        }  
        TextStage2 {vocab:self.vocab, ..self }
    }
*/







