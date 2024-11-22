# tf*idf based plain-text search engine
ref: https://en.wikipedia.org/wiki/Tf%E2%80%93idf

## terminology:
<b>tf(t,d):</b> a term t's frequency in document d divided by total number of terms in that document
<br>
<b>idf(t, D):</b> log of number of documents in corpus D divided by number of documents where term t appears
<br>
<b>tf*idf:</b> tf multiplied by idf