This is experimental project (the algorithm is emerging in permanent experiments with text tokenization).

The main idea is similar to the Byte Pair Encoding tokenizer: 
https://arxiv.org/pdf/1508.07909.pdf - 	the original article; 
https://leimao.github.io/blog/Byte-Pair-Encoding/ - some Python implementations. 
Here the implementation is in Rust. 

We have a text and split it in words. Words are composed of unicode scalars. We start from the 
initial vocabuary of the scalars (chars) by splitting the text to colleclion of chars. 

The 'Alice's Adventures in Wonderland by Lewis Carroll' of Gutenberg project is used here : 
http://www.gutenberg.org/ebooks/11 as the text.

We recursivly replace most frequent consequtive pair of tokens by the new token composed of the merged tokens in the pair.

Initial tokens are just chars. A token is a sequence of chars. Finally all tokens will merge in the initial words in the text.

The procedure was initially invented as splitting of a rare words to more usual components. But in the project I try to use it as a universal algorithm for tokenization of a text to basic tokens, which may be considered as a basis for vectorization of the text: the basic tokens we will find will be the basis (orthogonal) for the text vectorization. This is the program. 

We start from something similar to 'chars' (like scalar unicodes or graphemes). The good point: all words are splitted in chars : dog => 'd o g' , cat => 'c a t'. Not a good point: chars are very unspesific , and if the chars will be our basis vectors the 'bag of chars' will not work: 'd o g' and 'g o d' will be the same vectors if we will try to represent the words as sum of basis vectors 'd' , 'o', 'g'. 

On the other edge of the tokenization process we will have as out tokens the very initial words: 'alice', 
'her', 'head' etc.... will be same time the initial words and out basis vectors. It is some kind of 'one hot encoding'. Bag of words models is working trivially : word 'alice' is represented as vector 'alice'; 'her' => will be our basis vector 'her'.  There is always one token in the bag. 
And we have another problem : the vectors are too specific. 'dog' and 'dogs' are different vectors, even there is an intuitive filling the 'dogs' is something like a bag of 'dog' and 's'. We will have very big vocabulary of of basic words-tokens (our basis vectors) without any natural reflection of relations of the words: we declare as basis or basic actually what is not a basis or basic because of the internal relations , which we ignore. 

Let us return to the chars. The chars at the beginning , end and inside a word behave differently. And if we have a standalone I it is mot the same as 'i' at the beginning of end of a word. Let us use 'modifiers'. Modifiers are not a tokens, but they will 'decorate' a characters as a rudimentary type system: 
"🔺","🔸","🔹","🔻"

  '🔺d🔹o🔹g🔻' and '🔺g🔹 o🔹 d🔻'  ==> now 'd🔻' and '🔺d' are different basis tokens corresponding to 
the characters at the beginning or end of words. And by two blue diamonds we will '🔹o🔹' modify the internat characters. 'd🔻' ; '🔺d' and '🔹 d🔹' are different basis tokens now (different basis vectors). (We use emoji as the modifiers). The two '🔹' from every side of a token are used in part for not to use 'lookaround' in regex.
And 'i🔻' ; '🔺i' ; '🔹 i🔹' ; '🔺i🔻' all are different tokens too. 
The tokens at the beginning , end , internal of a word all have very different merging statistic withint the process of BPE tokenization. We have to handle them differently and the modifiers help with this. 

Intuition we will use: 
1.Tokens are resourses. The resourses may merge and produce another resourses : new tokens. 
2.The resourses are building blocks of our words.
3.As building blocks the GENERATED tokens may be considered as basis vectors of our system.
4.The quantity of possible tokens is VERY HUGE. If we use about 100 characters (unicode scalars) 
in our words and words up to the length 10 in out texts: the number of possible words is more than 10 in power 20.
5.The number of words and building blocks which are IN REALITY used for words building is VERY SMALL.
6.All words in the sample text are generated within about 6000 merges, so, we have max up to 6000 'building tokens'.
7.Modifiers may be considered as a rudimentary type system or as a rudimentary "positional" encoding.

Because of technical reasons (to simplify the search of a tokens, not to use ''lookaround' but a simple regex ) within the course of merges the words are represented like this: 

"  🔺a  🔹d🔹  🔹v🔹  🔹e🔹  🔹n🔹  🔹t🔹  🔹u🔹  🔹r🔹  🔹e🔹  s🔻  "  <=== two empty spaces are inserted 
between every pair of modified tokens (at the end and beginning too).

If at some merge 🔹e🔹  🔹n🔹  pair will merges, we will get : 
"  🔺a  🔹d🔹  🔹v🔹  🔹e🔹🔹n🔹  🔹t🔹  🔹u🔹  🔹r🔹  🔹e🔹  s🔻  "
If at some merge 🔺a  🔹d🔹 are to be merged (as most frequent pair), we will get:
"  🔺a🔹d🔹  🔹v🔹  🔹e🔹  🔹n🔹  🔹t🔹  🔹u🔹  🔹r🔹  🔹e🔹  s🔻  "

Let us take some 'familiar' and 'not familiar' words for the system. 
Familiar means some words from the 'Alice.....' text like : 'forgetting' , 'alice', 'yourself', 'consented'.
Not familiar, for example: 'coronavirus', 'tokenization', 'antidisestablishmentarianism', 'hippopotomonstrosesquippedaliophobia'. 
We wil tokenize the words : 
a.In case the system did just 2 merges (still just characters are out tokens).
ib.In case of all tokens are merged in original words (about 6000 merges for the text).

Only 2 merges, out tokens are mostly 'decorated characters': 

/=======================================
forgetting : ["🔺f", "🔹o🔹", "🔹r🔹", "🔹g🔹", "🔹e🔹", "🔹t🔹", "🔹t🔹", "🔹i🔹", "🔹n🔹", "g🔻"]
alice : ["🔺a", "🔹l🔹", "🔹i🔹", "🔹c🔹", "e🔻"]
yourself : ["🔺y", "🔹o🔹", "🔹u🔹", "🔹r🔹", "🔹s🔹", "🔹e🔹", "🔹l🔹", "f🔻"]
consented : ["🔺c", "🔹o🔹", "🔹n🔹", "🔹s🔹", "🔹e🔹", "🔹n🔹", "🔹t🔹", "🔹e🔹", "d🔻"]
forgetting : ["🔺f", "🔹o🔹", "🔹r🔹", "🔹g🔹", "🔹e🔹", "🔹t🔹", "🔹t🔹", "🔹i🔹", "🔹n🔹", "g🔻"]
coronavirus : ["🔺c", "🔹o🔹", "🔹r🔹", "🔹o🔹", "🔹n🔹", "🔹a🔹", "🔹v🔹", "🔹i🔹", "🔹r🔹", "🔹u🔹", "s🔻"]
forgetting : ["🔺f", "🔹o🔹", "🔹r🔹", "🔹g🔹", "🔹e🔹", "🔹t🔹", "🔹t🔹", "🔹i🔹", "🔹n🔹", "g🔻"]
tokenization : ["🔺t", "🔹o🔹", "🔹k🔹", "🔹e🔹", "🔹n🔹", "🔹i🔹", "🔹z🔹", "🔹a🔹", "🔹t🔹", "🔹i🔹", "🔹o🔹", "n🔻"]
antidisestablishmentarianism : ["🔺a", "🔹n🔹", "🔹t🔹", "🔹i🔹", "🔹d🔹", "🔹i🔹", "🔹s🔹", "🔹e🔹", "🔹s🔹", "🔹t🔹", "🔹a🔹", "🔹b🔹", "🔹l🔹", "🔹i🔹", "🔹s🔹", "🔹h🔹", "🔹m🔹", "🔹e🔹", "🔹n🔹", "🔹t🔹", "🔹a🔹", "🔹r🔹", "🔹i🔹", "🔹a🔹", "🔹n🔹", "🔹i🔹", "🔹s🔹", "m🔻"]
hippopotomonstrosesquippedaliophobia : ["🔺h", "🔹i🔹", "🔹p🔹", "🔹p🔹", "🔹o🔹", "🔹p🔹", "🔹o🔹", "🔹t🔹", "🔹o🔹", "🔹m🔹", "🔹o🔹", "🔹n🔹", "🔹s🔹", "🔹t🔹", "🔹r🔹", "🔹o🔹", "🔹s🔹", "🔹e🔹", "🔹s🔹", "🔹q🔹", "🔹u🔹", "🔹i🔹", "🔹p🔹", "🔹p🔹", "🔹e🔹", "🔹d🔹", "🔹a🔹", "🔹l🔹", "🔹i🔹", "🔹o🔹", "🔹p🔹", "🔹h🔹", "🔹o🔹", "🔹b🔹", "🔹i🔹", "a🔻"]
/========================

In the case the results for familiar and unfamiliar words are similar. 
Words of both types are splitted on 'decorated characters'.

/======================

After 5746 merges (all tokens are merged into original words), we use  "❗" as "UNCNOWN" token:
/======================
forgetting : ["🔺f🔹o🔹🔹r🔹🔹g🔹🔹e🔹🔹t🔹🔹t🔹🔹i🔹🔹n🔹g🔻"]
alice : ["🔺a🔹l🔹🔹i🔹🔹c🔹e🔻"]
yourself : ["🔺y🔹o🔹🔹u🔹🔹r🔹🔹s🔹🔹e🔹🔹l🔹f🔻"]
consented : ["🔺c🔹o🔹🔹n🔹🔹s🔹🔹e🔹🔹n🔹🔹t🔹🔹e🔹d🔻"]
inquisitively : ["🔺i🔹n🔹🔹q🔹🔹u🔹🔹i🔹🔹s🔹🔹i🔹🔹t🔹🔹i🔹🔹v🔹🔹e🔹🔹l🔹y🔻"]
coronavirus : ["❗"]
tokenization : ["❗"]
antidisestablishmentarianism : ["❗"]
hippopotomonstrosesquippedaliophobia : ["❗"]
========================

We may see the big difference. Familiar words are tokenized 'by itself'. And it is not a surprise , 
with the set of tokens the unfamiliar words all become "❗": the tokens can not match any of the 
words.

The number of the merges in hyperparameter in the system. The question is:
1.We use the "Alice...." text as a learning text for our system.
2.Our task is to find some reasonable tokenization for words even not from the very text, but 
which (tokenization) is more specific than just splitting to characters. 
3.We believe the "Alice..." text, as a 'big enought' sample of English text, has internal relations 
between subparts of the text (tokens) , which (in some approximation) reflect the general 
structure of English language. 
4.We believe, the relation is more illuminative then just splitting on 'characters'.
5.We believe , the relation is more general then just the tokens which correspond to the very set of 
all different words in the "Alice ..." text.

The question: how to find the hyperparameter, which correspond to the tokenization of the text? 



The results are (Number of merges is 2079): 
/==================================================
forgetting : ["🔺f🔹o🔹🔹r🔹🔹g🔹", "🔹e🔹🔹t🔹🔹t🔹🔹i🔹🔹n🔹g🔻"]
alice : ["🔺a🔹l🔹🔹i🔹🔹c🔹e🔻"]
yourself : ["🔺y🔹o🔹🔹u🔹🔹r🔹🔹s🔹🔹e🔹🔹l🔹f🔻"]
consented : ["🔺c🔹o🔹🔹n🔹", "🔹s🔹", "🔹e🔹🔹n🔹🔹t🔹🔹e🔹d🔻"]
inquisitively : ["🔺i🔹n🔹", "🔹q🔹🔹u🔹", "🔹i🔹🔹s🔹", "🔹i🔹🔹t🔹", "🔹i🔹", "🔹v🔹🔹e🔹🔹l🔹y🔻"]
coronavirus : ["🔺c🔹o🔹", "🔹r🔹🔹o🔹🔹n🔹", "🔹a🔹", "🔹v🔹", "🔹i🔹🔹r🔹", "🔹u🔹s🔻"]
tokenization : ["🔺t🔹o🔹", "🔹k🔹", "🔹e🔹🔹n🔹", "🔹i🔹", "🔹z🔹", "🔹a🔹🔹t🔹🔹i🔹🔹o🔹n🔻"]
antidisestablishmentarianism : ["🔺a🔹n🔹", "🔹t🔹", "🔹i🔹🔹d🔹", "🔹i🔹🔹s🔹", "🔹e🔹🔹s🔹🔹t🔹", "🔹a🔹🔹b🔹", "🔹l🔹", "🔹i🔹🔹s🔹🔹h🔹", "🔹m🔹", "🔹e🔹🔹n🔹🔹t🔹", "🔹a🔹🔹r🔹", "🔹i🔹", "🔹a🔹🔹n🔹", "🔹i🔹🔹s🔹", "m🔻"]
hippopotomonstrosesquippedaliophobia : ["🔺h🔹i🔹", "🔹p🔹🔹p🔹", "🔹o🔹🔹p🔹", "🔹o🔹🔹t🔹", "🔹o🔹", "🔹m🔹", "🔹o🔹🔹n🔹", "🔹s🔹🔹t🔹", "🔹r🔹", "🔹o🔹🔹s🔹", "🔹e🔹🔹s🔹", "🔹q🔹🔹u🔹", "🔹i🔹", "🔹p🔹🔹p🔹", "🔹e🔹🔹d🔹", "🔹a🔹", "🔹l🔹🔹i🔹", "🔹o🔹🔹p🔹", "🔹h🔹🔹o🔹", "🔹b🔹", "🔹i🔹", "a🔻"]
========================








The inspection of consequtive pairs is 
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
measure to compare the 'surprise' or 'most oftenness'. The theory of Generalised Species of Strands may 
help with it. The next tokenizer I am going to implement will use the theory. 

There is also "statistics" crate in the project. I used it for experiments with text processing. 
In lib.rs there are some tests. 



The sample output of the programm may be like this: (the results are at the very botton): 

```
~>/bpe$ cargo run --bin bpe_tokenizer
Compiling bpe v0.1.0 (/home/dmitri/bpe)
    Finished dev [unoptimized + debuginfo] target(s) in 2.97s
     Running `target/debug/bpe_tokenizer`
==========================
=========================
Get initial tokens from bpe words vocab
The initial tokens correspond to the unicode scalars : chars, except ▁ end of word
{"p": 1984, "u": 4009, "x": 182, "7": 8, "4": 8, "a": 9868, "e": 15512, "d": 5504, "w": 2980, "-": 220, "8": 11, "y": 2610, "’": 712, "j": 236, ":": 249, "f": 2390, "?": 204, "i": 8676, "o": 9548, "c": 3046, "2": 12, "\'": 9, "!": 452, "g": 2958, "l": 5234, "t": 12305, "v": 974, "ù": 1, "#": 1, "h": 7932, "6": 7, "s": 7301, "q": 223, ";": 193, ".": 1227, "m": 2465, "z": 80, "▁": 34665, "b": 1768, "5": 13, "1": 66, "0": 24, "%": 1, "3": 12, "@": 1, ",": 2576, "9": 9, "k": 1299, "$": 2, "n": 8095, "r": 6685}
Number of initial tokens 51
Iteration number: 0
Max pair !!! ("e", "▁")
glued_bigram  e▁ 
Iteration number: 1
Max pair !!! ("t", "h")
glued_bigram  th 
Iteration number: 2
Max pair !!! ("t", "▁")
glued_bigram  t▁ 
Iteration number: 3
Max pair !!! ("d", "▁")
glued_bigram  d▁ 
Iteration number: 4
Max pair !!! ("s", "▁")
glued_bigram  s▁ 
Iteration number: 5
Max pair !!! (",", "▁")
glued_bigram  ,▁ 
Iteration number: 6
Max pair !!! ("i", "n")
glued_bigram  in 
Iteration number: 7
Max pair !!! ("e", "r")
glued_bigram  er 
Iteration number: 8
Max pair !!! ("th", "e▁")
glued_bigram  the▁ 
Iteration number: 9
Max pair !!! ("a", "n")
glued_bigram  an 
Iteration number: 10
Max pair !!! ("o", "u")
glued_bigram  ou 
Iteration number: 11
Max pair !!! ("y", "▁")
glued_bigram  y▁ 
Iteration number: 12
Max pair !!! ("o", "▁")
glued_bigram  o▁ 
Iteration number: 13
Max pair !!! ("o", "n")
glued_bigram  on 
Iteration number: 14
Max pair !!! ("g", "▁")
glued_bigram  g▁ 
Iteration number: 15
Max pair !!! ("e", "n")
glued_bigram  en 
Iteration number: 16
Max pair !!! (".", "▁")
glued_bigram  .▁ 
Iteration number: 17
Max pair !!! ("a", "l")
glued_bigram  al 
Iteration number: 18
Max pair !!! ("e", "d▁")
glued_bigram  ed▁ 
Iteration number: 19
Max pair !!! ("f", "▁")
glued_bigram  f▁ 
Iteration number: 20
Max pair !!! ("an", "d▁")
glued_bigram  and▁ 
Iteration number: 21
```
........................................
........................................
```
Iteration number: 1993
Max pair !!! ("ni", "ght▁")
glued_bigram  night▁ 
Iteration number: 1994
Max pair !!! ("dec", "ide")
glued_bigram  decide 
Iteration number: 1995
Max pair !!! ("porpo", "ise▁")
glued_bigram  porpoise▁ 
Iteration number: 1996
Max pair !!! ("an", "n▁")
glued_bigram  ann▁ 
Iteration number: 1997
Max pair !!! ("gir", "l▁")
glued_bigram  girl▁ 
Iteration number: 1998
Max pair !!! ("stat", "us▁")
glued_bigram  status▁ 
Iteration number: 1999
Max pair !!! ("p", "ale▁")
glued_bigram  pale▁ 
=========================
After 2000 merging of most frequent pairs: 
The tokens vocab looks like this{"ating▁": 7, "pocket▁": 7, "wonderland▁": 8, "ep▁": 1, "sharply▁": 4, "turt": 5, "ki": 5, "using▁": 8, "agree▁": 11, "goo": 5, "ll▁": 12, "high▁": 16, "beautiful▁": 13, "melancholy▁": 6, "paper▁": 5, "these▁": 17, "icul": 1, "sle": 1, "moved▁": 9, "tax▁": 6, "repla": 2, "drea": 3, "comfor": 3, "hall▁": 9, "owed▁": 2, "ck▁": 11, "fast▁": 4, "pardon▁": 6, "yer▁": 4, "w": 42, "xt▁": 1, "you’ve▁": 7, "dare▁": 5, "ty▁": 11, "rup": 1, "cannot▁": 5, "hand▁": 21, "larger▁": 7, "contact▁": 5, "o": 35, "fri": 6, "ial▁": 5, "clo": 4, "ground▁": 5, "help▁": 12, "au": 5, "bir": 3, "ho": 8, "reading▁": 7, "sir▁": 7, "exec": 3, "always▁": 13, "they▁": 133, "everybody▁": 8, ".e▁": 22, "lear": 5, "ory▁": 2, "tic": 1, "f▁": 4, "ready▁": 11, "cu": 16, "lli": 1, "for▁": 179, "ened▁": 1, "guinea-": 2, "hedgeho": 3, "puzzled▁": 9, "ou": 4, "archive▁": 13, "ur": 6, "happen▁": 8, "find▁": 21, "ck": 2, "ps▁": 4, "it’ll▁": 8, "r▁": 2, "after▁": 42, "speaking▁": 5, "ebook▁": 12, "ch▁": 17, "doub": 3, "turn▁": 13, "spla": 4, "night▁": 4, "e’": 3, "surprised▁": 7, "different▁": 10, "sur": 5, "dec": 6, "pepp": 1, "turtle▁": 57, "rema": 7, "esca": 4, "ats▁": 8, "ws▁": 7, "com": 11, "did▁": 63, "dem": 4, "dreadfully▁": 6, "h": 32, "happ": 4, "try▁": 12, "but▁": 175, "voice▁": 48, "ù": 1, "you’re▁": 23, "until▁": 5, "ations▁": 8, "lled▁": 7, "fort": 1, "live▁": 8, "disc": 1, "ear": 16, "te": 9, "round▁": 41, "can▁": 45, "ity▁": 13, "dear▁": 29, "any": 3, "form▁": 7, "oner▁": 3, "won’t▁": 24, "7": 4, "fold": 1, "puzz": 5, "minutes▁": 11, "porpoise▁": 4, "eling▁": 2, "kes▁": 7, "stupid▁": 6, "oh▁": 45, "shrie": 2, "distribu": 5, "matter▁": 9, "swim▁": 5, "dead▁": 4, "anyone▁": 5, "ell▁": 1, "fla": 5, "opened▁": 10, "wan": 7, "donate▁": 5, "perhaps▁": 17, "ently▁": 6, "gutenberg": 1, "sleepy▁": 5, "queen▁": 68, "back▁": 38, "ribu": 2, "sha": 5, "aged▁": 4, "trying▁": 14, "times▁": 6, "hurry▁": 11, "remembered▁": 5, "whether▁": 11, "taken▁": 4, "both▁": 16, "wi": 19, "an": 30, "mis": 6, "five▁": 8, "$": 2, "shore▁": 4, "dry▁": 8, "deri": 6, "www▁": 10, "down▁": 103, "inv": 7, "were▁": 85, "rabbit-hole▁": 5, "un▁": 2, "say▁": 52, "ses▁": 15, "fren": 4, "creatures▁": 10, "till▁": 21, "finger▁": 5, "united▁": 15, "sleep▁": 5, "vani": 7, "folded▁": 5, "imp": 4, "added▁": 23, "re": 83, "shed▁": 9, "call▁": 9, "non": 2, "ting▁": 23, "bread-and-butter▁": 6, "sleep": 1, "alice▁": 386, "mock▁": 57, "line▁": 6, "editi": 2, "sc": 10, "children▁": 10, "o▁": 6, "cros": 6, "ul": 3, "ded▁": 26, "e▁": 56, "him▁": 43, "leg": 5, "deli": 7, "est": 6, "ched▁": 12, "good▁": 24, "iti": 6, "a": 34, "now▁": 60, "ld▁": 3, "twelve▁": 4, "prin": 5, "them▁": 88, "mouth▁": 10, "you": 2, "hookah▁": 5, "bea": 5, "clock▁": 5, "listen▁": 7, "run▁": 4, "oots▁": 4, "put▁": 31, "ressed▁": 8, "volunteers▁": 6, "’tis▁": 4, "easily▁": 6, "become▁": 5, "mine▁": 13, "sation▁": 3, "ful▁": 16, "er": 28, "hun": 9, "doing▁": 6, "thou": 8, "beg▁": 8, "ond▁": 4, "belie": 1, "att": 14, "dinah▁": 11, "write▁": 6, "take▁": 22, "child▁": 10, "wasn’t▁": 11, "vi": 18, "grin": 5, "even": 1, "confusion▁": 5, "having▁": 10, "first▁": 51, "pe": 9, "gu": 18, "s-": 6, "moral▁": 8, "twice▁": 5, "its▁": 69, "ran▁": 16, "inter": 4, "meaning▁": 8, "must▁": 54, "of▁": 634, "while▁": 27, "hadn’t▁": 8, "u▁": 9, "replacement▁": 5, "ended▁": 1, "s▁": 55, "ons▁": 11, "iously▁": 2, "access▁": 10, "my": 2, "fancy▁": 7, "agre": 1, "lin": 7, "each▁": 9, "bright▁": 7, "lying▁": 11, "generally▁": 7, "green▁": 4, "sighed▁": 5, "cried▁": 20, "ving▁": 13, "app": 16, "possible▁": 4, "queen": 1, "gli": 1, "minute▁": 21, "sitting▁": 10, "sn’t▁": 2, "mi": 25, "ent▁": 24, "immedi": 6, "mo": 20, "particu": 1, "gh▁": 7, "willi": 1, "does▁": 11, "stop▁": 6, "unless▁": 6, "our▁": 18, "rather▁": 25, "ick▁": 10, "slates▁": 7, "owing▁": 12, "twinkling▁": 4, "rinking▁": 1, "jury-": 5, "called▁": 15, "repe": 3, "they’": 9, "be": 17, "pa": 26, "cont": 5, "ff▁": 5, "del": 2, "-t": 5, "ank▁": 3, "vely▁": 7, "stood▁": 8, "ree▁": 1, "bit▁": 16, "ac": 23, "rabbit▁": 44, "ever▁": 32, "deal▁": 12, "your▁": 71, "itness▁": 1, "ati": 10, "copi": 2, "of": 4, "per": 22, "swa": 1, "table▁": 20, "ran": 6, "gardeners▁": 8, "wonder": 3, "thin": 2, "se": 29, "pepper▁": 8, "oud▁": 2, "c": 57, "worth▁": 6, "join▁": 9, "sounded▁": 5, "those▁": 11, "shrinking▁": 4, "sounds▁": 4, "mar": 10, "str": 6, ",▁": 2568, "shoes▁": 7, "m": 60, "his▁": 96, "should": 8, "me▁": 73, "angry▁": 5, "wards▁": 5, "ic▁": 13, "asleep▁": 8, "ously▁": 5, "e-": 8, "wow▁": 6, "writ": 2, "displa": 6, "hon": 1, "dis": 6, "shan’t▁": 6, "neck▁": 7, "fini": 1, "with▁": 229, "window▁": 8, "jury▁": 17, "fetch▁": 7, "nervous▁": 5, "da": 7, "dor": 1, "course▁": 27, "uck": 4, "shing▁": 8, "shouted▁": 9, "bel": 6, "notic": 1, "z": 12, "run": 2, "fanc": 1, "ch": 36, ".’▁": 10, "gutenberg▁": 30, "dual▁": 4, "inclu": 5, "strange▁": 5, "sch": 1, "thank▁": 4, "eggs▁": 5, "wanted▁": 4, "jump": 4, "reply▁": 5, "law▁": 9, "rowful▁": 4, "kept▁": 13, "mission▁": 4, "pl": 8, "sli": 5, "dering▁": 8, "den▁": 1, "quirements▁": 4, "get▁": 47, "p▁": 14, "ours▁": 1, "he▁": 123, "sometimes▁": 5, "read": 2, "protected▁": 6, "9▁": 4, "crow": 4, "whisp": 4, "1": 13, "contemp": 4, "to-": 4, "mou": 8, "knee▁": 5, "temper▁": 5, "inches▁": 6, "works▁": 33, "reach▁": 4, "william▁": 7, "tree▁": 5, "replied▁": 29, "turns▁": 4, "father▁": 6, "another▁": 22, "cho": 8, "hastily▁": 16, "ious▁": 7, "evi": 1, "explan": 5, "ask▁": 11, "by▁": 87, "dy▁": 10, "red▁": 5, "used▁": 19, "croque": 2, "pig▁": 11, "ow": 20, "is▁": 140, "sho": 3, "provide▁": 7, "ld": 9, "sort▁": 20, "going▁": 27, "tt": 2, "outside▁": 7, "decide": 4, "pigs▁": 2, "ough": 5, "loud▁": 6, "ugh": 1, "hear▁": 15, "ure▁": 8, "ven▁": 3, "be▁": 171, "bill▁": 14, "late▁": 7, "reason▁": 9, "led▁": 36, "thinking▁": 11, "ooks▁": 2, "days▁": 8, "oop▁": 9, "ect▁": 10, "want▁": 9, "cru": 6, "\'": 4, "your": 3, "quet▁": 1, "don’t▁": 61, "copies▁": 7, "do▁": 98, "arm▁": 15, "difficulty▁": 4, "dance▁": 15, "lied▁": 2, "tering▁": 6, "hold▁": 11, "has▁": 9, "ong▁": 1, "ces▁": 4, "faces▁": 5, "means▁": 8, "gutenberg-t": 1, "eg": 2, "chi": 5, "suppor": 4, "thi": 7, "li": 44, "ark▁": 6, "liked▁": 6, "ear▁": 13, "?▁": 202, "bs▁": 5, "himself▁": 6, "still▁": 13, "simple▁": 5, "trademark▁": 12, "head▁": 53, "song▁": 7, "stand▁": 7, "carroll▁": 4, "var": 4, "fac": 1, "angrily▁": 9, "wish▁": 22, "mp▁": 4, "ti": 24, "trial▁": 7, "stly▁": 5, "shou": 2, "shar": 5, "lessons▁": 10, "min": 7, "once▁": 34, "4": 3, "exactly▁": 8, "everything▁": 12, ".gutenberg▁": 10, "web▁": 5, "j": 12, "pan": 6, "should▁": 29, "noticed▁": 8, "trouble▁": 6, "draw▁": 7, "turning▁": 13, "du": 12, "c▁": 6, "then▁": 94, "@": 1, "0▁": 11, "v": 35, "day▁": 34, "ze▁": 6, "majesty▁": 12, "off▁": 73, "ke▁": 15, "many▁": 14, "tting▁": 4, "some▁": 53, "all▁": 200, "ser": 3, "certain▁": 5, "d▁": 39, "rily▁": 1, "ll": 11, "ni": 10, "bbing▁": 5, "ways▁": 2, "again▁": 83, "beginning▁": 15, "almost▁": 8, "grunted▁": 4, "fallen▁": 4, "um▁": 1, "guinea-pigs▁": 4, "rew▁": 2, "io": 1, "sor": 9, "and▁": 952, "cal": 4, "foo": 1, "compliance▁": 5, "zes▁": 1, "ence▁": 8, "tried▁": 19, "seem▁": 8, "ages▁": 5, "ci": 14, "gre": 3, "only▁": 53, "myself▁": 7, "su": 13, "smi": 6, "tell▁": 32, "moment▁": 29, "cl": 8, "small▁": 12, "bly▁": 5, "aby▁": 1, "seemed▁": 27, "learn▁": 8, "disclaim": 4, "pping▁": 5, "nly▁": 5, "yourself▁": 10, "k▁": 26, "move▁": 4, "chapter▁": 24, "ray▁": 5, "remarked▁": 10, "der": 6, "speak▁": 15, "heard▁": 30, "coun": 2, "large▁": 33, "pit": 1, "other▁": 57, "proces": 2, "es": 16, "walked▁": 10, "alice’s▁": 18, "left▁": 14, "fic": 4, "ut▁": 1, "shri": 1, "usu": 3, "caterpillar▁": 28, "whe": 1, "ret": 6, "curiosity▁": 5, "adventures▁": 12, "ei": 1, "lou": 4, "dist": 3, "shi": 4, "ger▁": 10, "sister▁": 8, "bro": 3, "executi": 4, "\'s▁": 5, "ing▁": 60, "broken▁": 6, "ead▁": 3, "mouth": 4, "footman▁": 13, "ned▁": 10, "pret": 5, "turned▁": 18, "act▁": 3, "way▁": 61, "march▁": 34, "lobster▁": 8, "mb": 7, "have▁": 87, "ap": 6, "didn’t▁": 14, "what": 5, ".8▁": 4, "hardly▁": 12, "her": 6, "case▁": 5, "medi": 2, "sol": 2, "que": 7, "seems▁": 5, "appeared▁": 8, "suddenly▁": 13, "begin": 4, "3": 1, "located▁": 7, "no▁": 100, "hoar": 4, "out▁": 117, "im": 10, "received▁": 6, "are▁": 88, "the▁": 1834, "inting▁": 7, "ir": 16, "ire▁": 1, "volunte": 1, "!▁": 444, "was▁": 359, ".s▁": 7, "room▁": 14, "not▁": 175, "br": 6, "remember▁": 14, "read▁": 19, "electron": 2, "mouse▁": 42, "pres": 15, "ent": 15, "res": 4, "fell▁": 6, "gold": 2, "own▁": 19, "deeply▁": 4, "white▁": 30, "ents▁": 12, "ely▁": 13, "whispered▁": 5, "leave▁": 9, "es▁": 53, "rec": 2, "ose▁": 4, "indeed▁": 16, "k-": 4, "qui": 7, "lling▁": 5, "indi": 4, "foundation▁": 22, "couldn’t▁": 9, "ors▁": 4, "silent▁": 7, "cats▁": 13, "gi": 8, "about▁": 102, "disa": 4, "op": 5, "tears▁": 12, "had▁": 178, "po": 23, "sion▁": 11, "trembling▁": 6, "key▁": 10, "however▁": 21, "flamin": 2, "argument▁": 4, "people▁": 16, "continued▁": 9, "looking▁": 31, "wonder▁": 18, "b": 93, "7▁": 4, "’ll▁": 8, "morning▁": 5, "sigh": 3, "nor": 6, "cup": 4, "comp": 13, "en": 32, "you▁": 439, "ve▁": 33, "ag": 14, "tone▁": 42, "tered▁": 7, "ca": 19, "grun": 1, "the": 5, "cra": 7, "ind▁": 2, "x▁": 8, "least▁": 9, "answer▁": 9, "hed": 3, "she’s▁": 7, "fre": 4, "waited▁": 11, "made▁": 30, "grown▁": 7, "me": 14, "hou": 2, "sudden▁": 5, "suppose▁": 14, "nothing▁": 34, "ates▁": 4, "mit": 3, "ated▁": 12, "sin": 9, "ly▁": 43, "rin": 7, "#": 1, "violently▁": 4, "top▁": 8, "ds▁": 15, "ried▁": 3, "r": 25, "id▁": 13, "you’ll▁": 6, "distribution▁": 8, "lay▁": 4, "cre": 6, "asking▁": 5, "why▁": 40, "’s▁": 38, "prizes▁": 5, "impatiently▁": 5, "ately▁": 6, "tale▁": 5, "in▁": 448, "ll’s▁": 4, "rule▁": 5, "remar": 8, "row": 7, "ar▁": 4, "opportun": 1, "an▁": 64, "na": 11, "site▁": 5, "sp": 20, "ghtened▁": 3, "wal": 5, "whiting▁": 8, "part▁": 6, "mon": 5, "ym": 7, "proc": 2, "sure▁": 27, "arms▁": 6, "it": 17, "i’d▁": 11, "lon": 10, "dropped▁": 5, "viol": 4, "begun▁": 7, "sm": 5, "man▁": 7, "8": 5, "y▁": 56, "air▁": 16, "at": 22, "sig": 5, "kno": 5, "cut▁": 5, "remark▁": 10, "butter▁": 3, "crowded▁": 5, "idea▁": 15, "oun": 5, "twel": 1, "soldi": 1, "passed▁": 5, "0": 13, "ea-": 1, "ges▁": 8, "dro": 2, "inst": 6, "t": 65, "eat▁": 17, "manage▁": 7, "foot▁": 10, "subject▁": 7, "here▁": 53, "name▁": 11, "rai": 5, "ma": 25, "through▁": 16, "cked▁": 4, "ga": 6, "hurried▁": 11, "when▁": 80, "sent": 2, "pp": 10, "went▁": 83, "adv": 9, "cor": 8, "pack▁": 5, "rink▁": 1, "spo": 4, "tea▁": 13, "sub": 4, "ered▁": 6, "terms▁": 22, "old▁": 21, "pur": 6, "lar": 6, "sadly▁": 5, "doubt": 4, "or▁": 170, "ba": 6, "plain▁": 2, "haven’t▁": 8, "y-": 10, "there’s▁": 24, "for": 15, "pr": 5, "birds▁": 10, "ex": 24, "along▁": 6, "int▁": 4, "iny▁": 5, "less": 4, "tongue▁": 4, "time▁": 71, "english▁": 7, "can’t▁": 28, "followed▁": 8, "except▁": 7, "kid▁": 5, "ar": 34, "kn": 10, "knew▁": 15, "far▁": 13, "never▁": 47, ".f▁": 11, "provi": 10, "where▁": 24, "car": 19, "garden▁": 16, "pleased▁": 7, "go": 2, "information▁": 9, "uti": 3, "the-": 5, "i’m▁": 59, "felt▁": 23, "gone▁": 13, ".1▁": 6, "cid": 4, "ea": 25, "ic": 21, "ice▁": 9, "plan": 1, "?": 2, "man": 7, "second▁": 6, "t▁": 74, "hear": 4, "fr": 5, "charge▁": 6, "think▁": 53, "lizard▁": 5, "ling▁": 35, "comfortable▁": 5, "ness▁": 4, "ink": 2, "up▁": 103, "duch": 3, "door▁": 30, "ec": 12, "chorus▁": 6, "ddle▁": 2, "box▁": 11, "chim": 1, "dodo▁": 13, "abi": 2, "their▁": 52, "better▁": 14, "life▁": 13, "solic": 4, "walking▁": 5, "mice▁": 4, "repeat▁": 7, "my▁": 59, "nose▁": 8, "ching▁": 13, "ily▁": 7, "person▁": 8, "general▁": 6, "history▁": 7, "queer▁": 10, "ri": 42, "hatter▁": 55, "aw": 5, "gs▁": 14, "hi": 6, "g": 40, "ited▁": 4, "ah": 4, "somebody▁": 7, "no": 13, "pair▁": 6, "who▁": 65, "doubt▁": 4, "permission▁": 7, "am": 6, "our": 10, "side▁": 20, "severely▁": 4, "girl▁": 4, "offended▁": 10, "anim": 6, "tremb": 3, "hearing▁": 4, "copyright▁": 20, "snee": 2, "beat▁": 4, "as▁": 280, "tired▁": 7, "mble▁": 4, "great▁": 39, "end▁": 30, "simp": 4, "section▁": 7, "pin": 6, "let": 3, "fl": 9, ".▁": 1118, "sy▁": 6, "words▁": 21, "digging▁": 4, "size▁": 13, "ep": 14, "pas": 6, "beauti": 3, "rest▁": 10, "sneez": 3, "world▁": 9, "din": 9, "ude▁": 4, "6": 7, "w▁": 10, "agreement▁": 18, "dormouse▁": 39, ";▁": 193, "ection▁": 5, "tion▁": 12, "iou": 2, "anything▁": 22, "’": 7, "rules▁": 5, "natural▁": 4, "ls▁": 12, "y": 41, "goes▁": 9, "watch▁": 8, "talk▁": 14, "stay▁": 5, "dge▁": 1, "often▁": 6, "surprise▁": 5, "tures▁": 4, "all": 4, "bl": 6, "additi": 6, "politely▁": 6, "consi": 8, "youth▁": 6, "ected▁": 3, "duc": 6, "sk": 7, "explain▁": 10, "yes▁": 13, "fe": 11, "sta": 10, "treacle▁": 5, "ous▁": 6, "what’s▁": 5, "oup▁": 1, "ards▁": 4, "to▁": 807, "fir": 5, "change▁": 15, "ter": 15, "thers▁": 1, "g▁": 19, "foll": 9, "gave▁": 15, "before▁": 42, "running▁": 8, "told▁": 6, "son▁": 7, "foot": 4, "prot": 3, "bel▁": 6, "mushroom▁": 8, "getting▁": 25, "altogether▁": 5, "sh": 38, "ment▁": 10, "low▁": 25, "among▁": 12, "pil": 1, "ross▁": 1, "story▁": 9, "wling▁": 4, "lat": 8, "party▁": 12, "important▁": 13, ".org▁": 11, "ard▁": 4, "ff": 4, "gr": 16, "from▁": 52, "list": 5, "also▁": 4, "tain▁": 6, "though▁": 11, "nearly▁": 13, "editions▁": 6, "free▁": 8, "less▁": 5, "n": 40, "ge▁": 6, "ept▁": 4, "roof▁": 6, "owner▁": 5, "consider▁": 4, "sters▁": 2, "piece▁": 7, "saying▁": 15, "us▁": 17, "please▁": 22, "out": 4, "puppy▁": 6, "re▁": 3, "exclaimed▁": 6, "paid▁": 6, "des▁": 3, "ee": 10, "flamingo▁": 5, "serp": 3, "feel▁": 8, "enough▁": 18, "tw": 4, "sl": 6, "honour▁": 4, "lobsters▁": 7, "gar": 1, "anc": 8, "middle▁": 7, "serpent▁": 9, "irs▁": 8, "word▁": 11, "seen▁": 15, "cro": 2, "el": 32, "pri": 6, ".3▁": 6, "bi": 10, "ful": 1, "pic": 11, "jo": 6, "ef▁": 6, "sti": 2, "ough▁": 1, "star": 15, "am▁": 16, "beau▁": 4, "rabb": 1, "height▁": 5, "1▁": 47, "ad": 12, "sed▁": 27, "school▁": 6, "ie": 2, "indign": 5, "hea": 4, "been▁": 38, "coll": 6, "ver": 17, "shut▁": 5, "!’▁": 8, "clu": 6, "spec": 5, "kly▁": 4, "liz": 1, "keep▁": 13, "likely▁": 5, "house▁": 18, "alone▁": 5, "thought▁": 74, "looked▁": 45, "te▁": 6, "tail▁": 9, "sit▁": 11, "make▁": 30, "wat": 11, "sts▁": 8, "eagerly▁": 8, "carried▁": 4, "ining▁": 4, "sight▁": 10, "fu": 5, "den": 10, "encour": 4, "reas": 3, "er-": 7, "el▁": 1, "supp": 6, "drew▁": 5, "how": 4, "question▁": 17, "trees▁": 7, "ed": 10, "can": 6, "into▁": 67, "quite▁": 55, "sto": 12, "’t▁": 1, "if▁": 118, "answered▁": 4, "jur": 6, "mu": 15, "quadrille▁": 5, "dly▁": 18, "answ": 1, "pat": 6, "tly▁": 13, "par": 10, "cr": 7, "gener": 2, "open▁": 7, "donations▁": 15, "number▁": 8, "win": 4, "utting▁": 6, "ney▁": 1, "creating▁": 4, "bottle▁": 10, "hot▁": 6, "pla": 7, "wh": 4, "ten▁": 7, "dream▁": 7, "next▁": 30, "eng": 5, "go▁": 50, "well▁": 63, "si": 25, "char": 6, "written▁": 9, "sal": 5, "pos": 6, "on": 35, "evidence▁": 8, "curious▁": 19, "wouldn’t▁": 13, "bur": 7, "de▁": 10, "don": 3, "ii▁": 8, "across▁": 5, "whole▁": 13, "book▁": 10, "or": 29, "fy▁": 5, "sett": 4, "wood▁": 8, "twink": 1, "hard▁": 8, "judge▁": 4, "tter▁": 4, "ought▁": 17, "sides▁": 8, "acc": 14, "very▁": 148, "sense▁": 3, "grow▁": 13, "said▁": 462, "hel": 6, "pped▁": 10, "walk▁": 5, "war": 2, "direc": 12, "fear▁": 4, "ab": 9, "pre": 4, "ition▁": 4, "le▁": 32, "u": 37, "vo": 7, "several▁": 5, "wher": 5, "owl▁": 6, "sing▁": 26, "lo": 19, "solem": 7, "meant▁": 5, "who": 5, "ented▁": 4, "near▁": 15, "best▁": 12, "this▁": 183, "hur": 6, "wa": 16, "ally▁": 11, "ening▁": 8, "ob": 11, "tru": 10, "dread": 2, "cheshire▁": 7, "se-t": 1, "interesting▁": 5, "ks▁": 21, "eping▁": 5, "pe▁": 10, "roy": 2, "associated▁": 8, "ton": 2, "such▁": 47, "certainly▁": 14, "hold": 7, "one▁": 111, "ootiful▁": 4, "bread-": 1, "country▁": 5, "tice▁": 1, "wor": 7, "ser▁": 6, "sec": 6, "saw▁": 14, "ts▁": 28, "tle▁": 4, "shion▁": 4, "deep": 1, "which▁": 56, "believe▁": 9, "might▁": 28, "mitted▁": 5, "on▁": 222, "you’d▁": 10, "copy▁": 12, "under": 6, "they’re▁": 13, "ven": 2, "f": 67, "5": 6, "fall▁": 7, "der▁": 9, "interrupted▁": 9, "ined▁": 10, "sh▁": 20, "possi": 4, "copying▁": 4, "check▁": 4, "mp": 9, "ook": 1, "particular▁": 6, "eb": 3, "natur": 1, "ventured▁": 4, "ance▁": 16, "st": 58, "croquet▁": 6, "sever": 1, "mind▁": 11, "5▁": 7, "croquet-ground▁": 4, "glo": 1, "duchess▁": 39, "hare▁": 31, "ers▁": 39, "roll▁": 2, "ber▁": 1, "-": 56, "leaves▁": 6, "tarts▁": 8, "gar▁": 5, "gras": 4, "come▁": 49, "pupp": 1, "ound▁": 10, "tur": 4, "tions▁": 11, "jumped▁": 6, "sat▁": 17, "gry▁": 3, "k": 27, "n’t▁": 4, "ense▁": 2, "soon▁": 25, "catch▁": 4, "race▁": 6, "it’s▁": 57, "warran": 5, "limited▁": 5, "shriek▁": 5, "grin▁": 6, "behind▁": 13, "ra": 18, "mer": 10, "produc": 6, "ru": 19, "tain": 6, "gla": 2, "sting▁": 5, "et▁": 17, "e’s▁": 6, "sul": 3, "set▁": 26, "-hole▁": 1, "ken▁": 1, "ani": 1, "heads▁": 10, "close▁": 13, "roo": 3, "anxiously▁": 14, "shall▁": 27, "ess▁": 7, "strai": 4, "fish▁": 8, "2": 12, "others▁": 8, "pale▁": 4, "ref": 3, "4▁": 5, "long▁": 35, "silence▁": 14, "hours▁": 4, "deep▁": 7, "cop": 4, "caterpil": 1, "waiting▁": 9, "fi": 34, "cup▁": 6, "plea": 11, "ing-": 5, "sea▁": 13, "cour": 5, "bowed▁": 4, "questions▁": 4, "n▁": 19, "took▁": 24, "vol": 1, "civil▁": 4, "ann▁": 4, "wondering▁": 7, "see▁": 69, "sw": 1, "began▁": 58, "behea": 4, "af": 7, "so": 20, "making▁": 8, "feet▁": 19, "ta": 20, "de": 13, "ability▁": 4, ",’▁": 7, "ed▁": 90, "seven▁": 6, "ows▁": 8, "four▁": 8, "repeated▁": 10, "we▁": 39, "hop": 5, "al": 38, "lity▁": 3, "ming▁": 5, "ter▁": 11, "ils▁": 4, "cause▁": 5, "ns▁": 2, "gir": 3, "ted▁": 27, "ool▁": 3, ":▁": 249, "loc": 6, "ging▁": 14, "ther▁": 8, "ener": 1, "notice▁": 8, "creature▁": 4, "spoke▁": 17, "tri": 8, "exc": 3, "nur": 6, "isn’t▁": 7, "changed▁": 8, "perform": 5, "th▁": 30, "procession▁": 5, "nearer▁": 5, "over▁": 40, "ah▁": 5, "ears▁": 11, "drink▁": 7, "play": 5, "tr": 9, "evening▁": 5, "ding▁": 15, "s": 79, "in": 49, "dam": 7, "tea": 9, "she▁": 541, "know▁": 88, "body▁": 2, "cla": 5, "laws▁": 12, "together▁": 9, "flo": 4, "hearts▁": 8, "remember": 1, "co": 23, "bo": 9, "mad▁": 16, "like▁": 84, "fro": 3, "se▁": 28, "jurors▁": 4, "q": 1, "ties▁": 7, "play▁": 8, "paragraph▁": 11, "whi": 6, "gryphon▁": 55, "al▁": 26, "fe▁": 6, "rep": 6, "cer": 4, "including▁": 8, "within▁": 6, "states▁": 19, "sly▁": 1, "le": 33, "fact▁": 8, "same▁": 25, "▁": 36, "ved▁": 13, "e": 64, "away▁": 27, "slowly▁": 8, "i’ll▁": 31, "finished▁": 12, "foun": 2, "growing▁": 11, "swi": 2, "fec": 1, "loo": 5, "eag": 3, "herself▁": 83, "came▁": 42, "distributing▁": 9, "row▁": 9, "fan": 4, "gutenberg-tm▁": 56, "knave▁": 9, "ph": 7, "thr": 8, "ne": 24, "paragra": 3, "tal": 2, "ner▁": 9, "arm": 4, "ning▁": 16, "eye▁": 7, "managed▁": 4, "instantly▁": 5, "taking▁": 5, "her▁": 248, "further▁": 4, "some": 5, "three▁": 26, "tu": 13, "ine▁": 6, "right▁": 38, "as": 11, "pigeon▁": 12, "shor": 6, "conver": 3, "comply▁": 6, "conversation▁": 9, "ordered▁": 4, "nice▁": 6, "sa": 26, "marked▁": 8, "cat": 6, "beg": 1, "fur": 10, "ves▁": 13, "forth▁": 8, "up": 8, "h▁": 2, "ms▁": 1, "cau": 9, "shouldn’t▁": 5, "rea": 3, "status▁": 4, "ite▁": 5, "offic": 8, "per▁": 4, "literary▁": 13, "dra": 4, "chin▁": 7, "makes▁": 12, "ys▁": 8, "ss▁": 14, "let’s▁": 5, "ble▁": 4, "spect": 4, "eyes▁": 29, "ring▁": 9, "kit": 4, "may▁": 29, "con": 28, "asked▁": 17, "botto": 4, "fore▁": 2, "game▁": 12, "ver▁": 5, "because▁": 16, "land▁": 2, "even▁": 20, "ste▁": 4, "any▁": 77, "ure": 2, "ends▁": 7, "than▁": 26, "itself▁": 14, "m▁": 26, "sage▁": 1, "argu": 3, "we’": 4, "lew": 4, "ldr": 2, "ght▁": 11, "king▁": 82, "ings▁": 5, "swam▁": 5, "between▁": 6, "sky▁": 4, "t-": 1, "busily▁": 4, "either▁": 11, "age▁": 20, "qu": 18, "har": 3, "quietly▁": 5, "verse▁": 4, "sted▁": 11, "most▁": 14, "un": 50, "last▁": 33, "distribute▁": 7, "there▁": 77, "ther": 6, "soo▁": 7, "how▁": 73, "sel": 7, "lea": 5, "nonsense▁": 7, "sneezing▁": 6, "tro": 6, "frightened▁": 7, "mes▁": 4, "pu": 9, "dic": 7, "mean▁": 10, "screa": 7, "med▁": 9, "golden▁": 7, "something▁": 18, "fee▁": 9, "umb": 9, "hands▁": 12, "interrup": 3, "laugh": 5, "queen’s▁": 8, "ugli": 4, "als▁": 8, "license▁": 16, "exp": 10, "plan▁": 4, "lic": 6, "dered▁": 6, "face▁": 15, "happened▁": 7, "ught▁": 4, "ending▁": 9, "ye▁": 3, "timidly▁": 9, "state▁": 6, "bu": 6, "oc": 6, "bb": 4, "give▁": 16, "em": 5, "it▁": 557, "at▁": 244, "recei": 6, "ut": 11, "hal": 2, "glass▁": 10, "ement▁": 1, "to": 18, "baby▁": 14, "ebooks▁": 7, "business▁": 8, "distance▁": 8, "nobody▁": 8, "pital▁": 4, "emp": 10, "ha": 5, "%": 1, "what▁": 137, "afraid▁": 12, "flow": 4, "tre": 6, "witness▁": 10, "shrill▁": 5, "work▁": 55, "ches▁": 2, "ro": 24, "project▁": 88, "fa": 19, "yet▁": 25, "brea": 11, "form": 6, "ing": 5, "les▁": 20, "opportunity▁": 9, "ise▁": 7, "ked▁": 17, "shook▁": 9, "usual▁": 5, "mbs▁": 5, "ber": 2, "’▁": 23, "arches▁": 4, "things▁": 33, "refund▁": 10, "gh": 12, "han": 10, "ali": 5, "ow▁": 13, "lef": 2, "tis▁": 1, "ying▁": 18, "cket▁": 1, "under▁": 23, "found": 3, "half▁": 21, "ich▁": 1, "efully▁": 4, "thing▁": 52, "poor▁": 27, "pool▁": 12, "th": 33, "treac": 2, "below▁": 6, "nee": 2, "ate▁": 24, "rabbit’s▁": 4, "d-": 6, "ght": 10, "talking▁": 17, "begin▁": 13, "enti": 7, "forgotten▁": 6, "ale▁": 1, "glad▁": 11, "ry▁": 8, "dd": 3, "full▁": 19, "wee": 4, "every": 2, "too▁": 26, "off": 6, "thor": 4, "waving▁": 5, "cur": 17, "piec": 3, "without▁": 34, "royal": 5, "ke": 6, "ess": 6, "doesn’t▁": 16, "passage▁": 4, "ju": 3, "electronic▁": 27, "il▁": 6, "rate▁": 9, "bla": 5, "mor": 5, "mut": 5, "em▁": 4, "stat": 2, "execut": 2, "tra": 3, "i▁": 421, "ation▁": 26, "she": 4, "rab": 3, "p": 64, "pro": 31, "phra": 4, "home▁": 5, "feeling▁": 7, "nibb": 5, "direction▁": 5, "happens▁": 5, "est▁": 9, "little▁": 129, "nat": 4, "l": 40, "se-tree▁": 4, "tel": 10, "ant▁": 13, "chan": 6, "ster▁": 8, "will▁": 40, "perfec": 4, "use▁": 34, "finish▁": 5, "twinkle▁": 8, "9": 5, "court▁": 18, "young▁": 5, "able▁": 19, "oo": 5, "every▁": 12, "upon▁": 29, "against▁": 10, "executed▁": 6, "few▁": 10, "onal▁": 9, "new▁": 8, "mean": 1, "gloves▁": 11, "aloud▁": 5, "ang": 2, "kind▁": 8, "place▁": 9, "lory▁": 7, "i’ve▁": 34, "that’s▁": 34, "hedgehog▁": 7, "en▁": 16, "sentence▁": 8, "defec": 4, "coming▁": 10, "fan▁": 10, "executioner▁": 5, "atiently▁": 2, "gra": 11, "found▁": 35, "des": 8, "fin": 7, "i": 65, "done▁": 15, "ir▁": 2, "pi": 9, "x": 16, "gently▁": 4, "di": 27, "eared▁": 4, "wrong▁": 5, "antly▁": 5, "sulk": 5, "ce▁": 12, "8▁": 2, "could▁": 78, "soup▁": 18, "tea-": 6, "fel": 5, "3▁": 5, "so▁": 153, "stu": 6, "opp": 1, "d": 63, ",": 1, "medium▁": 5, "being▁": 19, "much▁": 52, "effor": 5, "that▁": 295, "che": 5, "do": 18, "don▁": 1, "nine▁": 5, "let▁": 20, "add": 6, "else▁": 10, "sharp▁": 6, "wise▁": 6, "would▁": 83, "just▁": 53, "more▁": 50, "really▁": 13, "look▁": 28, "st▁": 36, "understand▁": 7, "continu": 1, "difficul": 3, "hair▁": 7, "ledge▁": 4, "tive▁": 5, "l▁": 10, "er▁": 49, "soldiers▁": 10, "spea": 1, "ged▁": 11, "por": 1, "a▁": 709, "anx": 3, "ge": 6, "fully▁": 11, "la": 25, "ventur": 3, "fortun": 4, "cook▁": 13, "cat▁": 35, ".": 22, "ary▁": 7, "ood▁": 1, "chimney▁": 6, "lar▁": 4, "got▁": 47, "writing▁": 8, "confu": 7, "two▁": 40, "arch▁": 5}
Number of final  tokens 1884
=========================
OrderedSetOfTokens ["bread-and-butter▁", "croquet-ground▁", "distribution▁", "distributing▁", "gutenberg-tm▁", "conversation▁", "rabbit-hole▁", "replacement▁", "guinea-pigs▁", "caterpillar▁", "impatiently▁", "information▁", "comfortable▁", "interesting▁", "interrupted▁", "opportunity▁", "executioner▁", "wonderland▁", "melancholy▁", "dreadfully▁", "remembered▁", "volunteers▁", "quirements▁", "difficulty▁", "gutenberg-t", "everything▁", ".gutenberg▁", "compliance▁", "adventures▁", "foundation▁", "permission▁", "altogether▁", "associated▁", "particular▁", "procession▁", "distribute▁", "frightened▁", "electronic▁", "understand▁", "beautiful▁", "everybody▁", "surprised▁", "different▁", "creatures▁", "confusion▁", "generally▁", "twinkling▁", "gardeners▁", "shrinking▁", "gutenberg▁", "sometimes▁", "protected▁", "trademark▁", "beginning▁", "curiosity▁", "whispered▁", "trembling▁", "continued▁", "violently▁", "copyright▁", "agreement▁", "important▁", "exclaimed▁", "quadrille▁", "donations▁", "certainly▁", "anxiously▁", "questions▁", "wondering▁", "paragraph▁", "including▁", "instantly▁", "shouldn’t▁", "something▁", "forgotten▁", "direction▁", "speaking▁", "porpoise▁", "gutenberg", "children▁", "possible▁", "thinking▁", "yourself▁", "remarked▁", "appeared▁", "suddenly▁", "received▁", "remember▁", "couldn’t▁", "argument▁", "followed▁", "somebody▁", "severely▁", "offended▁", "dormouse▁", "anything▁", "surprise▁", "politely▁", "mushroom▁", "editions▁", "consider▁", "flamingo▁", "lobsters▁", "question▁", "answered▁", "creating▁", "evidence▁", "wouldn’t▁", "cheshire▁", "ventured▁", "repeated▁", "creature▁", "together▁", "finished▁", "literary▁", "nonsense▁", "sneezing▁", "happened▁", "business▁", "distance▁", "rabbit’s▁", "executed▁", "hedgehog▁", "sentence▁", "atiently▁", "soldiers▁", "sharply▁", "contact▁", "reading▁", "puzzled▁", "archive▁", "minutes▁", "distribu", "perhaps▁", "whether▁", "meaning▁", "sitting▁", "rinking▁", "sounded▁", "nervous▁", "shouted▁", "strange▁", "mission▁", "william▁", "replied▁", "another▁", "hastily▁", "provide▁", "outside▁", "himself▁", "carroll▁", "angrily▁", "lessons▁", "exactly▁", "noticed▁", "trouble▁", "turning▁", "majesty▁", "certain▁", "grunted▁", "disclaim", "chapter▁", "alice’s▁", "footman▁", "lobster▁", "located▁", "electron", "however▁", "looking▁", "morning▁", "suppose▁", "nothing▁", "opportun", "ghtened▁", "whiting▁", "dropped▁", "crowded▁", "subject▁", "through▁", "hurried▁", "haven’t▁", "there’s▁", "english▁", "pleased▁", "walking▁", "general▁", "history▁", "hearing▁", "section▁", "digging▁", "natural▁", "explain▁", "treacle▁", "running▁", "getting▁", "serpent▁", "thought▁", "eagerly▁", "carried▁", "written▁", "curious▁", "several▁", "ootiful▁", "country▁", "believe▁", "they’re▁", "copying▁", "croquet▁", "duchess▁", "limited▁", "silence▁", "caterpil", "waiting▁", "ability▁", "changed▁", "evening▁", "remember", "gryphon▁", "growing▁", "herself▁", "managed▁", "further▁", "ordered▁", "because▁", "between▁", "quietly▁", "interrup", "queen’s▁", "license▁", "timidly▁", "witness▁", "project▁", "talking▁", "without▁", "doesn’t▁", "passage▁", "feeling▁", "happens▁", "se-tree▁", "twinkle▁", "against▁", "difficul", "chimney▁", "writing▁", "pocket▁", "pardon▁", "you’ve▁", "cannot▁", "larger▁", "ground▁", "always▁", "guinea-", "hedgeho", "happen▁", "turtle▁", "you’re▁", "ations▁", "stupid▁", "matter▁", "anyone▁", "opened▁", "donate▁", "sleepy▁", "trying▁", "finger▁", "united▁", "folded▁", "twelve▁", "hookah▁", "listen▁", "ressed▁", "easily▁", "become▁", "sation▁", "wasn’t▁", "having▁", "hadn’t▁", "iously▁", "access▁", "bright▁", "sighed▁", "minute▁", "particu", "unless▁", "rather▁", "slates▁", "called▁", "rabbit▁", "itness▁", "pepper▁", "sounds▁", "asleep▁", "shan’t▁", "window▁", "course▁", "wanted▁", "rowful▁", "dering▁", "contemp", "temper▁", "inches▁", "father▁", "reason▁", "copies▁", "tering▁", "simple▁", "should▁", "almost▁", "fallen▁", "myself▁", "moment▁", "seemed▁", "walked▁", "sister▁", "executi", "broken▁", "turned▁", "didn’t▁", "hardly▁", "inting▁", "volunte", "deeply▁", "indeed▁", "silent▁", "people▁", "wonder▁", "answer▁", "waited▁", "sudden▁", "you’ll▁", "asking▁", "prizes▁", "remark▁", "butter▁", "passed▁", "manage▁", "tongue▁", "except▁", "garden▁", "second▁", "charge▁", "lizard▁", "chorus▁", "better▁", "repeat▁", "person▁", "hatter▁", "ection▁", "what’s▁", "change▁", "before▁", "though▁", "nearly▁", "saying▁", "please▁", "enough▁", "honour▁", "middle▁", "height▁", "school▁", "likely▁", "looked▁", "number▁", "utting▁", "bottle▁", "across▁", "mitted▁", "leaves▁", "jumped▁", "shriek▁", "behind▁", "others▁", "making▁", "notice▁", "perform", "nearer▁", "hearts▁", "jurors▁", "within▁", "states▁", "slowly▁", "paragra", "taking▁", "pigeon▁", "comply▁", "marked▁", "status▁", "itself▁", "busily▁", "either▁", "golden▁", "ending▁", "ebooks▁", "nobody▁", "afraid▁", "shrill▁", "arches▁", "things▁", "refund▁", "efully▁", "waving▁", "little▁", "finish▁", "gloves▁", "that’s▁", "coming▁", "gently▁", "medium▁", "really▁", "continu", "ating▁", "using▁", "agree▁", "paper▁", "these▁", "moved▁", "comfor", "ready▁", "it’ll▁", "after▁", "ebook▁", "night▁", "voice▁", "until▁", "round▁", "won’t▁", "eling▁", "ently▁", "queen▁", "times▁", "hurry▁", "taken▁", "shore▁", "sleep▁", "added▁", "alice▁", "mouth▁", "clock▁", "doing▁", "dinah▁", "write▁", "child▁", "first▁", "moral▁", "twice▁", "while▁", "ended▁", "fancy▁", "lying▁", "green▁", "cried▁", "immedi", "owing▁", "stood▁", "table▁", "wonder", "worth▁", "those▁", "shoes▁", "should", "angry▁", "wards▁", "ously▁", "displa", "fetch▁", "shing▁", "thank▁", "reply▁", "works▁", "reach▁", "turns▁", "explan", "croque", "going▁", "decide", "don’t▁", "dance▁", "faces▁", "means▁", "suppor", "liked▁", "still▁", "stand▁", "trial▁", "tting▁", "bbing▁", "again▁", "tried▁", "small▁", "learn▁", "pping▁", "speak▁", "heard▁", "large▁", "other▁", "proces", "march▁", "seems▁", "mouse▁", "white▁", "leave▁", "lling▁", "about▁", "tears▁", "flamin", "tered▁", "least▁", "she’s▁", "grown▁", "ately▁", "begun▁", "terms▁", "sadly▁", "plain▁", "birds▁", "along▁", "can’t▁", "never▁", "where▁", "think▁", "their▁", "ching▁", "queer▁", "doubt▁", "tired▁", "great▁", "words▁", "beauti", "world▁", "rules▁", "watch▁", "often▁", "tures▁", "additi", "youth▁", "ected▁", "thers▁", "among▁", "story▁", "wling▁", "party▁", "owner▁", "sters▁", "piece▁", "puppy▁", "indign", "house▁", "alone▁", "ining▁", "sight▁", "encour", "trees▁", "quite▁", "dream▁", "whole▁", "judge▁", "ought▁", "sides▁", "sense▁", "ition▁", "meant▁", "ented▁", "ening▁", "eping▁", "bread-", "shion▁", "which▁", "might▁", "you’d▁", "check▁", "tarts▁", "tions▁", "catch▁", "warran", "produc", "sting▁", "-hole▁", "heads▁", "close▁", "shall▁", "hours▁", "bowed▁", "civil▁", "began▁", "seven▁", "cause▁", "spoke▁", "isn’t▁", "drink▁", "knave▁", "three▁", "right▁", "conver", "forth▁", "makes▁", "let’s▁", "asked▁", "verse▁", "there▁", "hands▁", "dered▁", "state▁", "glass▁", "ement▁", "pital▁", "shook▁", "usual▁", "under▁", "thing▁", "below▁", "begin▁", "execut", "ation▁", "perfec", "court▁", "young▁", "every▁", "aloud▁", "place▁", "found▁", "eared▁", "wrong▁", "antly▁", "could▁", "being▁", "sharp▁", "would▁", "ledge▁", "fully▁", "ventur", "fortun", "high▁", "repla", "hall▁", "owed▁", "fast▁", "dare▁", "hand▁", "help▁", "they▁", "ened▁", "find▁", "turn▁", "lled▁", "live▁", "dear▁", "form▁", "oner▁", "shrie", "swim▁", "dead▁", "back▁", "aged▁", "both▁", "five▁", "down▁", "were▁", "till▁", "shed▁", "call▁", "ting▁", "sleep", "mock▁", "line▁", "editi", "ched▁", "good▁", "them▁", "oots▁", "’tis▁", "mine▁", "belie", "take▁", "inter", "must▁", "each▁", "ving▁", "queen", "sn’t▁", "willi", "does▁", "stop▁", "jury-", "they’", "vely▁", "ever▁", "deal▁", "your▁", "join▁", "neck▁", "with▁", "jury▁", "notic", "dual▁", "inclu", "eggs▁", "kept▁", "ours▁", "whisp", "knee▁", "tree▁", "ious▁", "used▁", "sort▁", "pigs▁", "loud▁", "hear▁", "bill▁", "late▁", "ooks▁", "days▁", "want▁", "quet▁", "lied▁", "hold▁", "head▁", "song▁", "wish▁", "stly▁", "once▁", "draw▁", "then▁", "many▁", "some▁", "rily▁", "ways▁", "ence▁", "seem▁", "ages▁", "only▁", "tell▁", "move▁", "left▁", "mouth", "have▁", "case▁", "begin", "room▁", "read▁", "fell▁", "ents▁", "cats▁", "sion▁", "tone▁", "made▁", "ates▁", "ated▁", "ried▁", "tale▁", "ll’s▁", "rule▁", "remar", "site▁", "part▁", "sure▁", "arms▁", "idea▁", "soldi", "foot▁", "here▁", "name▁", "cked▁", "when▁", "went▁", "pack▁", "rink▁", "ered▁", "doubt", "time▁", "knew▁", "provi", "felt▁", "gone▁", "ling▁", "ness▁", "door▁", "ddle▁", "dodo▁", "life▁", "solic", "mice▁", "nose▁", "ited▁", "pair▁", "side▁", "girl▁", "tremb", "beat▁", "mble▁", "size▁", "rest▁", "sneez", "tion▁", "goes▁", "talk▁", "stay▁", "consi", "ards▁", "gave▁", "told▁", "ment▁", "ross▁", ".org▁", "from▁", "also▁", "tain▁", "free▁", "less▁", "roof▁", "paid▁", "feel▁", "word▁", "seen▁", "ough▁", "beau▁", "been▁", "shut▁", "keep▁", "tail▁", "make▁", "drew▁", "into▁", "gener", "open▁", "next▁", "well▁", "book▁", "wood▁", "twink", "hard▁", "tter▁", "very▁", "grow▁", "said▁", "pped▁", "walk▁", "direc", "fear▁", "sing▁", "solem", "near▁", "best▁", "this▁", "ally▁", "dread", "such▁", "tice▁", "copy▁", "under", "fall▁", "ined▁", "possi", "natur", "ance▁", "sever", "mind▁", "hare▁", "roll▁", "come▁", "ound▁", "ense▁", "soon▁", "race▁", "it’s▁", "grin▁", "strai", "fish▁", "pale▁", "long▁", "deep▁", "took▁", "behea", "feet▁", "four▁", "lity▁", "ming▁", "ging▁", "ther▁", "over▁", "ears▁", "ding▁", "know▁", "body▁", "laws▁", "like▁", "ties▁", "play▁", "fact▁", "same▁", "away▁", "i’ll▁", "came▁", "ning▁", "nice▁", "offic", "chin▁", "spect", "eyes▁", "ring▁", "botto", "fore▁", "game▁", "land▁", "even▁", "ends▁", "than▁", "sage▁", "king▁", "ings▁", "swam▁", "sted▁", "most▁", "last▁", "mean▁", "screa", "laugh", "plan▁", "face▁", "ught▁", "give▁", "recei", "baby▁", "what▁", "work▁", "ches▁", "ying▁", "cket▁", "found", "half▁", "poor▁", "pool▁", "treac", "glad▁", "full▁", "every", "royal", "rate▁", "home▁", "ster▁", "will▁", "able▁", "upon▁", "onal▁", "kind▁", "lory▁", "i’ve▁", "defec", "done▁", "soup▁", "much▁", "effor", "that▁", "nine▁", "else▁", "wise▁", "just▁", "more▁", "look▁", "hair▁", "tive▁", "cook▁", "confu", "arch▁", "turt", "icul", "tax▁", "drea", "yer▁", "ial▁", "sir▁", "exec", "lear", "ory▁", "for▁", "doub", "spla", "pepp", "rema", "esca", "ats▁", "did▁", "happ", "try▁", "but▁", "fort", "disc", "can▁", "ity▁", "fold", "puzz", "kes▁", "ell▁", "ribu", "dry▁", "deri", "www▁", "say▁", "ses▁", "fren", "vani", "cros", "ded▁", "him▁", "deli", "now▁", "prin", "run▁", "put▁", "ful▁", "thou", "beg▁", "ond▁", "grin", "even", "its▁", "ran▁", "ons▁", "agre", "ent▁", "our▁", "ick▁", "repe", "cont", "ank▁", "ree▁", "bit▁", "copi", "thin", "oud▁", "his▁", "wow▁", "writ", "fini", "fanc", "jump", "law▁", "den▁", "get▁", "read", "crow", "ask▁", "red▁", "pig▁", "ough", "ure▁", "ven▁", "led▁", "oop▁", "ect▁", "your", "arm▁", "has▁", "ong▁", "ces▁", "ark▁", "ear▁", "shou", "shar", "web▁", "day▁", "off▁", "all▁", "rew▁", "and▁", "zes▁", "bly▁", "aby▁", "nly▁", "ray▁", "coun", "shri", "dist", "ger▁", "ing▁", "ead▁", "ned▁", "pret", "act▁", "way▁", "what", "medi", "hoar", "out▁", "are▁", "the▁", "ire▁", "was▁", "not▁", "pres", "gold", "own▁", "ely▁", "ose▁", "indi", "ors▁", "disa", "had▁", "key▁", "’ll▁", "sigh", "comp", "you▁", "grun", "ind▁", "top▁", "lay▁", "why▁", "proc", "i’d▁", "viol", "man▁", "air▁", "cut▁", "twel", "ges▁", "inst", "eat▁", "sent", "tea▁", "old▁", "int▁", "iny▁", "less", "kid▁", "far▁", "the-", "i’m▁", "ice▁", "plan", "hear", "duch", "box▁", "chim", "ily▁", "who▁", "anim", "snee", "end▁", "simp", "ude▁", "dge▁", "yes▁", "ous▁", "oup▁", "foll", "son▁", "foot", "prot", "bel▁", "low▁", "ard▁", "list", "ept▁", "des▁", "serp", "irs▁", "star", "rabb", "sed▁", "coll", "spec", "kly▁", "sit▁", "sts▁", "reas", "supp", "dly▁", "answ", "tly▁", "ney▁", "hot▁", "ten▁", "char", "sett", "wher", "owl▁", "se-t", "hold", "one▁", "ser▁", "saw▁", "tle▁", "deep", "der▁", "ers▁", "ber▁", "gar▁", "gras", "pupp", "sat▁", "gry▁", "n’t▁", "tain", "e’s▁", "set▁", "ken▁", "ess▁", "cup▁", "plea", "ing-", "sea▁", "cour", "ann▁", "see▁", "ows▁", "ter▁", "ils▁", "ted▁", "ool▁", "ener", "play", "she▁", "mad▁", "sly▁", "ved▁", "foun", "row▁", "ner▁", "eye▁", "her▁", "some", "ine▁", "shor", "ves▁", "ite▁", "per▁", "ble▁", "may▁", "ver▁", "ste▁", "any▁", "argu", "ght▁", "sky▁", "age▁", "ther", "soo▁", "how▁", "mes▁", "med▁", "fee▁", "ugli", "als▁", "flow", "yet▁", "brea", "form", "les▁", "ise▁", "ked▁", "mbs▁", "tis▁", "ich▁", "ate▁", "enti", "ale▁", "too▁", "thor", "piec", "stat", "phra", "nibb", "est▁", "ant▁", "chan", "use▁", "few▁", "new▁", "mean", "fan▁", "sulk", "tea-", "don▁", "let▁", "spea", "ged▁", "cat▁", "ary▁", "ood▁", "lar▁", "got▁", "two▁", "ep▁", "goo", "ll▁", "sle", "ck▁", "xt▁", "ty▁", "rup", "fri", "clo", "bir", ".e▁", "tic", "lli", "ps▁", "ch▁", "sur", "dec", "ws▁", "com", "dem", "ear", "any", "oh▁", "fla", "wan", "sha", "mis", "inv", "un▁", "imp", "non", "leg", "est", "iti", "ld▁", "you", "bea", "hun", "att", "of▁", "lin", "app", "gli", "gh▁", "ff▁", "del", "ati", "per", "swa", "ran", "mar", "str", "me▁", "ic▁", "hon", "dis", "dor", "uck", "bel", "run", ".’▁", "sch", "sli", "he▁", "to-", "mou", "cho", "evi", "by▁", "dy▁", "is▁", "sho", "ugh", "be▁", "cru", "do▁", "chi", "thi", "bs▁", "var", "fac", "mp▁", "min", "pan", "ze▁", "ke▁", "ser", "um▁", "sor", "cal", "foo", "gre", "smi", "der", "pit", "fic", "ut▁", "usu", "whe", "ret", "lou", "shi", "bro", "\'s▁", ".8▁", "her", "sol", "que", "no▁", ".s▁", "ent", "res", "es▁", "rec", "qui", "nor", "cup", "ve▁", "the", "cra", "hed", "fre", "hou", "mit", "sin", "ly▁", "rin", "ds▁", "id▁", "cre", "’s▁", "in▁", "row", "ar▁", "an▁", "wal", "mon", "lon", "sig", "kno", "oun", "ea-", "dro", "rai", "adv", "cor", "spo", "sub", "pur", "lar", "or▁", "for", ".f▁", "car", "uti", ".1▁", "cid", "man", "ink", "up▁", "abi", "my▁", "gs▁", "our", "as▁", "pin", "let", "sy▁", "pas", "din", "iou", "ls▁", "all", "duc", "sta", "to▁", "fir", "ter", "pil", "lat", "ge▁", "us▁", "out", "re▁", "gar", "anc", "cro", "pri", ".3▁", "ful", "pic", "ef▁", "sti", "am▁", "hea", "ver", "!’▁", "clu", "liz", "te▁", "wat", "den", "er-", "el▁", "how", "can", "sto", "’t▁", "if▁", "jur", "pat", "par", "win", "pla", "eng", "go▁", "sal", "pos", "bur", "de▁", "don", "ii▁", "fy▁", "acc", "hel", "war", "pre", "le▁", "who", "hur", "tru", "ks▁", "pe▁", "roy", "ton", "wor", "sec", "ts▁", "on▁", "ven", "sh▁", "ook", "glo", "tur", "mer", "gla", "et▁", "sul", "ani", "roo", "ref", "cop", "vol", ",’▁", "ed▁", "we▁", "hop", "ns▁", "gir", "loc", "tri", "exc", "nur", "th▁", "ah▁", "dam", "tea", "cla", "flo", "fro", "se▁", "whi", "al▁", "fe▁", "rep", "cer", "swi", "fec", "loo", "eag", "fan", "thr", "tal", "arm", "cat", "beg", "fur", "ms▁", "cau", "rea", "dra", "ys▁", "ss▁", "kit", "con", "ure", "we’", "lew", "ldr", "har", "sel", "lea", "tro", "dic", "umb", "exp", "lic", "ye▁", "it▁", "at▁", "hal", "emp", "tre", "ing", "ber", "han", "ali", "ow▁", "lef", "nee", "ght", "ry▁", "wee", "off", "cur", "ess", "il▁", "bla", "mor", "mut", "em▁", "tra", "she", "rab", "pro", "nat", "tel", "ang", "en▁", "gra", "des", "fin", "ir▁", "ce▁", "fel", "so▁", "stu", "opp", "che", "add", "st▁", "er▁", "por", "anx", "ki", "au", "ho", "f▁", "cu", "ou", "ur", "ck", "r▁", "e’", "te", "wi", "an", "re", "sc", "o▁", "ul", "e▁", "er", "vi", "pe", "gu", "s-", "u▁", "s▁", "my", "mi", "mo", "be", "pa", "-t", "ac", "of", "se", ",▁", "e-", "da", "ch", "pl", "p▁", "9▁", "ow", "ld", "tt", "eg", "li", "?▁", "ti", "du", "c▁", "0▁", "d▁", "ll", "ni", "io", "ci", "su", "cl", "k▁", "es", "ei", "mb", "ap", "im", "ir", "!▁", "br", "k-", "gi", "op", "po", "7▁", "en", "ag", "ca", "x▁", "me", "na", "sp", "ym", "it", "sm", "y▁", "at", "ma", "ga", "pp", "ba", "y-", "pr", "ex", "ar", "kn", "go", "ea", "ic", "t▁", "fr", "ec", "ri", "aw", "hi", "ah", "no", "am", "fl", ".▁", "ep", "w▁", ";▁", "bl", "sk", "fe", "g▁", "sh", "ff", "gr", "ee", "tw", "sl", "el", "bi", "jo", "1▁", "ad", "ie", "fu", "ed", "mu", "cr", "wh", "si", "on", "or", "ab", "vo", "lo", "wa", "ob", "mp", "eb", "st", "5▁", "ra", "ru", "4▁", "fi", "n▁", "sw", "af", "so", "ta", "de", "al", ":▁", "tr", "in", "co", "bo", "le", "ph", "ne", "tu", "as", "sa", "up", "h▁", "m▁", "t-", "qu", "un", "pu", "bu", "oc", "bb", "em", "ut", "to", "ha", "ro", "fa", "’▁", "gh", "th", "d-", "dd", "ke", "ju", "i▁", "oo", "pi", "di", "8▁", "3▁", "do", "l▁", "a▁", "ge", "la", "w", "o", "h", "ù", "7", "$", "a", "c", "m", "z", "1", "\'", "4", "j", "@", "v", "3", "b", "#", "r", "8", "0", "t", "?", "g", "6", "’", "y", "n", "u", "f", "5", "-", "k", "2", "s", "q", "▁", "e", "%", "p", "l", "9", "i", "x", "d", ",", "."]
=========================
========================
Tokenize sample word ! 'antidisestablishmentarianism▁'
Oho !! ["an", "ti", "dis", "est", "ab", "li", "sh", "m", "ent", "ar", "i", "ani", "sm", "▁"]
========================
Tokenize sample word ! 'hippopotomonstrosesquippedaliophobia▁'
hippo.... !! ["hi", "pp", "op", "o", "to", "mon", "str", "o", "se", "s", "qui", "p", "pe", "d", "ali", "op", "ho", "bi", "a▁"]
========================
Tokenize sample word ! 'hiPpopotomonStrosesquippeDaliophobia▁'
hiPpo.... !! ["hi", "UNC", "p", "op", "o", "to", "mon", "UNC", "tro", "se", "s", "qui", "p", "pe", "UNC", "ali", "op", "ho", "bi", "a▁"]
========================
Tokenize sample word ! 'PPPPPPPabacNNNNNNNNNNNNNN▁'
 The result is : ["UNC", "ab", "ac", "UNC", "▁"]
~>/bpe$ 
```
