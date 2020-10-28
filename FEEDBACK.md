# Feedback

- Das Closure Beispiel mit dem Counter und Tests gefällt mir so richtig gut.
  Ich habe noch einen Reset eingebaut und gleich mal rausgefunden, dass das Verhalten anders als in Go ist.
- Every test should contain at least one assert. :)
- Use smaller functions, then illustrate behavior with unit tests.
- What's the difference between std::String and str?
- You should add more comments to code if it represents your slides.
  One should be able to understand most of the code without the presentation.
- The unsafe part does not compile on my mac
    ```
    error: linking with `cc` failed: exit code: 1
    
    = note: Undefined symbols for architecture x86_64:
              "_tooling", referenced from:
                  slides::main::h49cef2b309cdebfd in slides.2c58vjgiwz7pgu26.rcgu.o
              "_borrow_checker", referenced from:
                  slides::main::h49cef2b309cdebfd in slides.2c58vjgiwz7pgu26.rcgu.o
            ld: symbol(s) not found for architecture x86_64
            clang: error: linker command failed with exit code 1 (use -v to see invocation)
    ```
- Can you provide an example how error-handling is typically done
  (e.g. one function handing error through to another)?

  