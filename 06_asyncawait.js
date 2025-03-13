// Rohit: Function to fetch data from JSONPlaceholder API
// Run on NodeJS or Browser

async function fetchData(url) {
    try {
        // Use the fetch API to make a GET request
        const response = await fetch(url);

        // Check if the response is ok (status code 200-299)
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        // Parse the JSON data from the response
        const data = await response.json();

        // Return the parsed data
        return data;
    } catch (error) {
        // Handle any errors that occur during the fetch
        console.error('Error fetching data:', error);
    }
}

// Main function to run the example
async function main() {
    // URL to fetch a list of posts from JSONPlaceholder
    const url = 'https://jsonplaceholder.typicode.com/posts';

    // Fetch the data
    const posts = await fetchData(url);

    // Log the fetched posts to the console
    console.log('Fetched Posts:', posts);
}

// Run the main function
main();
