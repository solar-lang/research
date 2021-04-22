## Notes

### solar repl

it might be nice, to have a function in the repl, to show all variables in the local scope.
For IDE stuff etc.

---

it might be nice, to have a session decoupled from a terminal.
e.g. multiple terminals can access the same session. this way one could do this script
    watch 'solar repl --session 12345 -c "current_scope : tojson" | jq'
and run it in a separate window to display all variables in the current scope
