// This is intended to represent git2::Config
// https://docs.rs/git2/0.6.8/git2/struct.Config.html
mod data_store {
    pub struct DataStore {}

    impl DataStore {
        pub fn get_bool(&self, _key: &str) -> bool {
            let fake_value = true;
            fake_value
        }
        pub fn get_string(&self, _key: &str) -> String {
            return "fake_value".to_string();
        }
    }
}

// // ---------------------------------------------------------------------------------
// // The GetValue trait is used to add a generic method `get_value` to the DataStore.
// trait GetValue {
//     fn get_value<T>(&self, key: &str) -> T
//     where
//         T: DataStoreGetValue,
//     {
//         T::data_store_get_value(self, key)
//     }
// }

// impl GetValue for data_store::DataStore {}
// // ---------------------------------------------------------------------------------

// ---------------------------------------------------------------------------------
// The DataStoreGetValue trait is used to implement the ability to fetch an entry of type T from
// the data store for types T such as String and bool.
trait DataStoreGetValue {
    fn data_store_get_value(key: &str, data_store: &data_store::DataStore) -> Self;
}

impl DataStoreGetValue for bool {
    fn data_store_get_value(key: &str, data_store: &data_store::DataStore) -> Self {
        data_store.get_bool(key)
    }
}

impl DataStoreGetValue for String {
    fn data_store_get_value(key: &str, data_store: &data_store::DataStore) -> Self {
        data_store.get_string(key)
    }
}
// ---------------------------------------------------------------------------------

pub fn main() {
    let data_store = data_store::DataStore {};

    // Use the provided API which requires specifying the type explicitly:
    println!("{:?}", data_store.get_bool("my_bool_key"));
    println!("{:?}", data_store.get_string("my_string_key"));

    // Use a static method on the value types
    println!(
        "{:?}",
        bool::data_store_get_value("my_bool_key", &data_store)
    );
    println!(
        "{:?}",
        String::data_store_get_value("my_bool_key", &data_store)
    );

    // // Use a type-generic API that we have addded
    // println!("{:?}", data_store.get_value::<bool>("my_bool_key"));
    // println!("{:?}", data_store.get_value::<String>("my_bool_key"));
}
