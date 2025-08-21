## No Overloading

Easier to resolve, especially with unspecified types / generics functions

Question: How do we enable Vec*Vec ^ Vec*Matrix ?

Question: Will `function add(Vec T, Vec T) Vec T` work?

## Default Generics

    Res<T, E = ToString>

usage:

    function getUser(id: int) Res<User> ...

## Traits | Interfaces

- only dynamic / pointer indirected to keep things simple

## Method resolve

- imports
- package symbols
- local scope (e.g. let bindings)
- package of 1st argument

## Smart Pointers:

note: MAYBE just use GC Pointers alltogether.

    Rc T, Gc T, RawPtr T
    are all
    *T
