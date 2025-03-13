// Rohit: Illustrating Promises in JavaScript.
// Run on NodeJS or Browser

// resolve and reject are just handlers/hooks/lambdas/closures/delegates/interceptors/middlewares
// All the above shibboleths mentioned are just crazy buzzwords for one thing -> Callback Function
const promise = new Promise((resolve, reject) => {
  const data = {
    firstName: "Samar",
  };

  console.log(`Initial Data: ${JSON.stringify(data)}`);

  setTimeout(() => {
    data.middleName = "Singh";
    resolve(data);
  }, 2000);
});

promise
  .then((data) => {
    console.log(`Data updated with middleName: ${JSON.stringify(data)}`);
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        data.lastName = "Chauhan";
        resolve(data);
      }, 2000);
    });
  })
  .then((data) => {
    console.log(`Data updated with lastName: ${JSON.stringify(data)}`);
  })

  .finally(() => console.log(`COMPLETED.`));
