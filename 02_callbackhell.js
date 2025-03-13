// Rohit: Illustrating the Callback Hell in JavaScript.
// Run on Deno, NodeJS or Browser

function getData(dataId, getNextData) {
    console.log(`Getting data for ${dataId}...`);

    setTimeout(function() {
        console.log(`Fetched data for ${dataId}\n`);
        setTimeout(function() {getNextData(dataId)}, 1000); // Trigger the next data fetch after 1 second
    }, 2000);
    
}

getData(1, function() { // First call
    getData(2, function() { // Second nested call 
        getData(3, function() { // Third call
             getData(4, function() { // Fourth call
                getData(5, function() { // Fifth call
                    console.log('ALL DATA FETCHED.');
                });
            });
        });
    });
});
