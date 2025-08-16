// Rohit: Illustrating Generics in TypeScript.
// Run on Node, Deno, or Bun.

// Function
const add = (num1: number, num2: number): number => num1 + num2
console.log(`Sum: ${add(2, 3)}`)

function greet(name: string = 'Guest'): string {
  return 'Hello ' + name
}
console.log(`${greet()}`)

// Destructuring
function getCoordinates() {
  return [21, 42]
}
const [x, y] = getCoordinates()
console.log(`x: ${x}, y:${y}`)

function getUser() {
  return {
    name: 'Rohit',
    age: 45
  }
}
// Block Scoping
{
  const { name, age } = getUser()

  console.log(`Name: ${name}, Age: ${age}`)
}

// Closure
function makeAdder(x: number): (y: number) => number {

  return function(y: number): number {
    return x + y
  }

}

const fiveAdder = makeAdder(5)
console.log(fiveAdder(12))

// Higher-order function
function applyNumOperation(numarray: number[], operation: (num: number) => number) {
  return numarray.map(operation)
}

const doubledNum = applyNumOperation([1, 2, 3], x => x * 2)

console.log(doubledNum)

// Generic Implementation
function applyOperation<T, U>(
  array: T[],
  operation: (item: T) => U): U[] {
  return array.map(operation)
}

const doubled = applyOperation([4, 5, 6], x => x * 2)
console.log(doubled)

const strings = applyOperation([4, 5, 6], x => `Number: ${x}`)
console.log(strings)

const lengths = applyOperation(['apple', 'mango', 'orange'], x => x.length)
console.log(lengths)

