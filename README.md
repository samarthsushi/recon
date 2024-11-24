# tf*idf based plain-text search engine
ref: https://en.wikipedia.org/wiki/Tf%E2%80%93idf

## terminology:
<b>`tf(t,d)`:</b> a term `t`'s frequency in document `d` divided by total number of terms in that document
<br>
<b>`idf(t, D)`:</b> log of number of documents in corpus `D` divided by number of documents where term `t` appears
<br>
<b>`tf*idf`:</b> `tf` multiplied by `idf`

## how to use:
> forward index = fi


after building an executable, go to the directory where your corpus lies and run the executable
<br>
build a fi from the corpus using the `build_fi` command
<br>
run any queries using `?` or save the fi using `save_fi`
<br>
if you want to use a previously saved fi, use `load_fi`

### commands:
1. `load_fi` or `l`:
<br>
loads fi (a json file) from the directory where the binary lies
<br>
usage: `load_fi <filename>`

2. `save_fi` or `s`:
<br>
saves fi (a json file) to the directory where the binary lies
<br>
usage: `save_fi <filename>`

3. `build_fi` or `b`:
<br>
builds fi from a corpus, which has to be the current working directory
<br>
usage: `build_fi`

4. `query` or `?`:
<br>
keywords to search in the fi (currently supports only single query)
<br>
usage: `query <query>`

5. `exit` or `e`:
<br>
exits the program

6. `help` or `h`:
<br>
prints out a manual