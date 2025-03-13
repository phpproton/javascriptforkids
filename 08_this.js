// Rohit: Illustrating this Context in JavaScript.
// Run on Deno, NodeJS or Browser

const person1 = {
  name: "Samar",
  age: 14,
  class: 10,
  speak: function () {
    console.log(`${this.name} speaks Russian.`);
  },
};

const person2 = {
  name: "Samir",
  age: 11,
  class: 7,
};

person1.speak();
// bind returns a new function so add () to execute it instantly
person1.speak.bind(person2)();

