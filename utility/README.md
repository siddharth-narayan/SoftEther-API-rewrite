We must parse all C functions and remove them if not exported by the final shared library.

```java -jar antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust utility/C.g4```