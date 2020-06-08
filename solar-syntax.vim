if exists("b:current_syntax")
    finish
endif

syn keyword structureKeyword type function trait let

syn keyword controllFlowKeyword if else then do match is or return

syn keyword operatorKeyword = + - ! * /  | && < > <= >= ^ ||

syn region string start='"' end='"'

syn match ident '\a+'

syn keyword structureIdent type function trait let nextgroup=ident skipwhite
