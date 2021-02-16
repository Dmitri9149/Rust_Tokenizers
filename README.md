The tokenizer in the project is similar to Byte Pair Encoding tokenizer.

https://arxiv.org/pdf/1508.07909.pdf - 	the original article; 
https://leimao.github.io/blog/Byte-Pair-Encoding/ - some Python implementations. 
Here the implementation is in Rust. 

We have a text of words. Words are composed of unicode scalars. We start from the 
initial vocabuary of the scalars (chars) by splitting the text to colleclion of chars. 

After that we recursivly replace most frequent consequtive pair of tokens by a new unseen token.
Initial tokens are just chars. The token is a sequence of chars. The inspection of consequtive pairs is 
limited to word boundaries. To the  end of every word a special unicode symbol '\n{2581}' is attached to mark the 
end of word position. 

The number of merging operations is hyperparameter. 

Finally we get a vocabulary : (token: frequency) of tokens. 

To decode (decompose) a WORD to corresponding tokens: We order the set of awailable tokens by the 
lenght of tokens. Starting from the longest tokens we search in the WORD for the tokens and split it 
for the matched tokens. 

There are some problems in the approach. The measure of 'most frequency'. The (implicit) measure is 
quite 'flat': just the quantity of token (or a pair) in the vocabulary. But what is more frequent : 
100 of "abc" in the text or 100 of "there" ? We have 3 chars and 5 chars tokens, and , intuitively, the 100 of 
5 chars word is a bigger 'surprise' than 100 of 3 chars word. The question is to find the approppriate 
measure to compare the 'surprice' or 'most ofteness'. The theory of Generalise Species of Strands may 
help with it. The next tokenizer I am going to implement will use the theory. 



The sample output of the programm may be like this: 


