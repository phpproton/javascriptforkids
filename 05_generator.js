// Rohit: Illustrating Generator in JavaScript.
// Run on Node, Deno, Bun or Browser

// Define a generator function that yields a sequence of numbers
function* numberGenerator() {
    let number = 1;

    // Infinite loop to keep generating numbers
    while (true) {
        // Yield the current number and pause the generator
        yield number;

        // Increment the number for the next yield
        number++;
    }
}

// Create an instance of the generator
const generator = numberGenerator();

// Use the generator to get the first 5 numbers in the sequence
for (let i = 0; i < 5; i++) {
    // Get the next value from the generator
    const nextNumber = generator.next().value;

    // Log the value to the console
    console.log(`Generated number: ${nextNumber}`);
}
