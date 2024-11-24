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
> fi = forward index


after building an executable, go to the directory where your corpus lies and run the executable
<br>
build a fi from the corpus using the `build_fi` command
<br>
run any queries using `?` or save the fi using `save_fi`
<br>
if you want to use a previously saved fi, use `load_fi`

### commands:
#### `load_fi` or `l`:
loads fi (a json file) from the directory where the binary lies
<br>
<b>usage:</b> 
```
load_fi <filename>
```

#### `save_fi` or `s`:
saves fi (a json file) to the directory where the binary lies
<br>
<b>usage:</b>
``` 
save_fi <filename>
```

#### `build_fi` or `b`:
builds fi from a corpus, which has to be the current working directory
<br>
<b>usage:</b>
``` 
build_fi
```

#### `query` or `?`:
keywords to search in the fi (currently supports only single query)
<br>
<b>usage:</b> 
```
query <query>
```

#### `exit` or `e`:
exits the program

#### `help` or `h`:
prints out a manual
