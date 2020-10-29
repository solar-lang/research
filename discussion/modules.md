# Module hierachy

This is hwo modules are supposed to be used.


Let's suppose we have this structure:

    main.sol                (file)
        collections         (dir)
            maps            (dir)
                hashmap.sol (file)
                btree.sol   (file)
            lists           (dir)
                array.sol   (file)
                list.sol    (dir)

and `list.sol` looks like this:
```lisp
-- collections/lists/list.sol
pub type List T
- value T
- next Optional List T
```

we can access List from `main.sol` like this:

```lisp
-- main.sol
let x: collections.lists.List

-- or

use collections.(list, maps)
let x: lists.List
```


Levels of publicity:
-   `pub` can be accessed from anywhere in library
-   `mod` can be accessed from module/directory (and subdirectories??)
