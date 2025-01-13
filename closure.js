// Rohit: Illustrating Closures in JavaScript.
// Run on NodeJS or Browser

// Closure is nothing but just a 'function inside a function'

// Make General Multipliers with Closures
function makeMultiplier(multiplier) {
  // This function below encloses the 'multiplier' parameter with it as soon as it is returned on lines 16 or 17
  return function (number) {
    console.log(
      `Number ${number} multiplied by ${multiplier} is ${number * multiplier}.`,
    );
    return number * multiplier;
  };
}

const twiceMultiplier = makeMultiplier(2);
const thriceMultiplier = makeMultiplier(3);

twiceMultiplier(24);
thriceMultiplier(24);

// Another Example: UrlMaker
function makeUrl(domain) {
  // This function below encloses the 'domain' parameter with it as soon as it is returned on lines 32 or 33
  return function (url) {
    console.log(`Your URL is: http://${url}.${domain}/`);
    return url + domain;
  };
}

const comMaker = makeUrl("com");
const netMaker = makeUrl("net");

comMaker("github");
netMaker("sourceforge");

