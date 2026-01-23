We must parse all C functions and remove them if not exported by the final shared library.
This utility automates this task, outputing only those functions in each header that are not in the symbols file
run with ```header-prune <header file> <symbols file>```

Regenerate ANTLR utils
```java -jar antlr4-4.13-3-SNAPSHOT-complete.jar -Dlanguage=Rust utility/C.g4```