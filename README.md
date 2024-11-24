# tf*idf based plain-text search engine
ref: 
<br>
https://en.wikipedia.org/wiki/Tf%E2%80%93idf 
<br>
https://en.wikipedia.org/wiki/Search_engine_indexing#The_forward_index

## terminology:
<b>`tf(t,d)`:</b> a term `t`'s frequency in document `d` divided by total number of terms in that document
<br>
<b>`idf(t, D)`:</b> log of number of documents in corpus `D` divided by number of documents where term `t` appears
<br>
<b>`tf*idf`:</b> `tf` multiplied by `idf`

## how to use:
> ii = inverted index


after building an executable, go to the directory where your corpus lies and run the executable
<br>
build a ii from the corpus using the `build_ii` command
<br>
run any queries using `?` or save the ii using `save_ii`
<br>
if you want to use a previously saved ii, use `load_ii`

### commands:
#### `load_ii` or `l`:
loads ii (a json file) from the directory where the binary lies
<br>
<b>usage:</b> 
```
load_ii <ilename>
```

#### `save_ii` or `s`:
saves ii (a json file) to the directory where the binary lies
<br>
<b>usage:</b>
``` 
save_ii <filename>
```

#### `build_ii` or `b`:
builds ii from a corpus, which has to be the current working directory
<br>
<b>usage:</b>
``` 
build_ii
```

#### `query` or `?`:
keywords to search in the ii (currently supports only single query)
<br>
<b>usage:</b> 
```
query <query>
```

#### `exit` or `e`:
exits the program

#### `help` or `h`:
prints out a manual
