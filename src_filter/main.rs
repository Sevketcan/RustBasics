// Define a struct to represent the filter condition
struct FilterCondition<T> {
    condition: T, // Field to store the desired value
}

impl<T: PartialEq> FilterCondition<T> {
    // Method to check if an item matches the condition
    fn is_match(&self, item: &T) -> bool {
        &self.condition == item
    }
}

// Custom filter function
fn custom_filter<T: PartialEq>(
    collection: &Vec<T>, 
    filter: &FilterCondition<T>
) -> Vec<T> 
where 
    T: Clone, // Ensure elements can be cloned to create the new filtered collection
{
    let mut filtered_result = Vec::new(); // Create an empty vector for the result

    // Iterate over the collection and check each element against the filter condition
    for item in collection {
        if filter.is_match(item) {
            filtered_result.push(item.clone());
        }
    }

    filtered_result
}

fn main() {
    // Step 1: Create a collection (e.g., a vector of integers)
    let collection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Step 2: Initialize a FilterCondition object with the desired value
    let condition = FilterCondition { condition: 5 };

    // Step 3: Call the custom_filter function with the collection and condition
    let filtered_result = custom_filter(&collection, &condition);

    // Step 4: Print the filtered result
    println!("Original collection: {:?}", collection);
    println!("Filtered result: {:?}", filtered_result);
}
