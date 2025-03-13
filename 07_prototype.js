// Rohit: Illustrating Prototypes in JavaScript.
// Run on Deno, NodeJS or Browser

const cat = new Object({
  species: "cat",
  habit: "carnivore",
  legs: "quadraped",
});

const percy = Object.create(cat);

// Prototype just means 'parent object' to which you can attach properties or functions to expand functionality
Object.prototype.saySomething = function () {
  console.log("Meow!");
};

percy.saySomething();

// Example with a String
const name = new String("Samar Singh Chauhan");

// Custom Function
String.prototype.makeItBig = function () {
  return this.toUpperCase();
};

console.log(name.makeItBig());

// Example with an Array
const cars = ["Mercedes", "Maruti", "Fiat", "Hyundai", "Tata"];

// Custom Function
Array.prototype.iNeedM = function () {
  return this.filter((car) => car.startsWith("M"));
};

console.log(cars.iNeedM());
