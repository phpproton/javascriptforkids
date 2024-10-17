// Rohit: Illustrating the Callback Hell in JavaScript.
// Run on NodeJS or Browser

function getData(dataId, getNextData) {
    console.log(`Getting data for ${dataId}.....`);

    setTimeout(function() {
        console.log(`Fetched data for ${dataId}.`);
        getNextData(dataId); // Trigger the next data fetch
    }, 2000);
    
}

getData(1, function() { // First call
    getData(2, function() { // Second call nested
        getData(3, function() { // Third call
             getData(4, function() { // Fourth call
                getData(5, function() { // Fifth call
                    console.log('ALL DATA FETCHED.');
                });
            });
        });
    });
});
