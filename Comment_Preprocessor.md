comment preprocessor

step 0: check position of comments

- `WS` + comment = comment
- comment + `WS` = comment
- `.*` + comment -> warning: comment should be at the beginning of the line, format to the top
- `///.*\n` next line should be a valid expr or object_pattern_elem
- `//` + function -> warning: use `///` for function doc

step 1: replace

- `//` to @@comment which will do nothing
- `///` to @@structural_doc, `var` will be checked
- `//!` to DOC_HEADING = `line`

step 2: concatenate docs

- `(comment \n)* comment` = comment
- `(structural_doc \n)* structural_doc` = structural_doc
- `(DOC_HEADING \n)* DOC_HEADING` = DOC_HEADING

step 3: validate inner comment is a valid markdown or not, if not -> warning