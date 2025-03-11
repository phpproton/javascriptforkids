// Rohit: Illustrating Proxy Object in JavaScript.
// Run on NodeJS or Browser

// Target object
const target = {
    name: "Rohit",
    age: 40
};

// Handler object with traps (methods to intercept operations)
const handler = {
    // Trap for getting property values
    get: function(obj, prop) {
        if (prop in obj) {
            console.log(`Getting property "${prop}": ${obj[prop]}`);
            return obj[prop];
        } else {
            console.log(`Property "${prop}" does not exist`);
            return undefined;
        }
    },

    // Trap for setting property values
    set: function(obj, prop, value) {
        if (prop === 'age' && typeof value !== 'number') {
            throw new TypeError('Age must be a number');
        }
        console.log(`Setting property "${prop}" to ${value}`);
        obj[prop] = value;
        return true; // Indicate success
    },

    // Trap for checking if a property exists
    has: function(obj, prop) {
        console.log(`Checking if property "${prop}" exists`);
        return prop in obj;
    },

    // Trap for deleting a property
    deleteProperty: function(obj, prop) {
        console.log(`Deleting property "${prop}"`);
        if (prop in obj) {
            delete obj[prop];
            return true; // Indicate success
        }
        return false; // Indicate failure
    }
};

// Create a proxy object
const proxy = new Proxy(target, handler);

// Example usage
console.log(proxy.name); // Getting property "name": Rohit
proxy.age = 44; // Setting property "age" to 44
console.log(proxy.age); // Getting property and outputting

console.log('name' in proxy); // Checking if property "name" exists
delete proxy.name; // Deleting property "name"
console.log(proxy.name); // Property "name" does not exist
