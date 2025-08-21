# Async as a library

-> async is a global function, that takes a closure taking one argument
-> async function return an Future<T>

```
fun something() -> Future<ShoppingItem> =
    async (await) =>
        let allItems = await (get "localhost:8081/api/v1/itemList")
        let id = allItems / first / map uuid / expect "to find at least one item" in
        post "localhost:8081/api/v1/item/order/" ++ str uuid
```
