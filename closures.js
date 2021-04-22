
let cc = function() {
  let x = 7

  // all have the same environment
  //
  // *{x: Int}
  let setx = (n) => x=n;
  let getx = () => x;
  let incr = () => ++x;

  // *{x: Int}
  let nested = () => {
    // for nested this is seen like a var on the stack
    // but must be resolved on heap *{y: Int}
    let y = 0;

    // *{x: Int, y: Int}
    let sum = () => x + y;
    let sety = (n) => y=n;
    let setx2 = (n) => x=n;

    /*
     * one way to resolve the environment of the nested closure `sum` would be a struct like this:
     * *{
     *    _parent: *{ x: Int }
     *    y: Int
     *
     *    const Int* x() {  // <- this here will be implicitly done by the compiler
     *      return _parent->x
     *    }
     * }

    return [sety, sum, setx2]
  }

  x = 99

  return [setx, getx, incr, nested]
}


let [se,ge,inc, nested] = cc()
let print = n => console.log(n)

print(ge())   // 99
inc()
print(ge())  // 100
se(7)
print(ge())  // 7


let [sety, sum, setx2] = nested()

setx2(456)

print(ge()) // 456


/*
 * compiling closures:
 * - get *all* variables on stack, that appear in a closure
 * - do not put them on stack. Put them all on heap.
 *    - that part of the heap may be interpreted as a struct {x, y, z}
 *    - a pointer to that struct is now the environment of the closure
 *
 * problem: What if the closure captures values inside a closure?
 */


/*
 * alternative:
 *  (this way all closures are immutable. e.g. foreach can not change outer scope)
 *
 * recognize all variables vs in the environment of closure x at compile time
 * the moment clusure x is created:
 * - allocate sizeof(vs) bytes of memory, save address in pointer p
 * - copy values of variables vs to memory
 * - pointer p is now environment of closure  x
 *
 * What's not possible?
 * This is not possible:
 *
 * sum = 0
 * list.forEach(value => { sum += value })
 *
 * sum would remain 0
 *
 * in the background an inassesible version of sum would get incremented.
 *
 * works better for immutable data structures
 */
