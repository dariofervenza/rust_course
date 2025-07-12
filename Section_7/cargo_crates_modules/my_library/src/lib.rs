mod some_module {  // we create a module like this

    pub struct Tasks {
        pub item: String,  // we have to make both the struct and the fields inside public, if not they will trigger an error qhen using from outside this scope
    }

    mod things_todo {  // we create a module inside a module here
        fn add_activity() {}
        fn update_activity() {}
        fn mark_completed() {}
    }
    pub mod items_completed {
        pub fn remove_task() {}
        fn move_back_todo() {}
    }
}

// we can install cargo modules with:
// cargo install cargo-modules

// then we can run:
// cargo modules structure  // this displays a setup with the project structure

// here it appears the 'pub' keyword --> this makes an item be accessible from outside of the module

// here our pub(crate) is the list mod, so it can be accessed from the outside
// however things_todo is pub(self), and this is equivalent to PRIVATE, 
// in items_completed we included pub keyword so it will be public

//crate my_library
//└── mod list: pub(crate)
//    ├── struct Tasks: pub(self)
//    ├── mod items_completed: pub
//    │   ├── fn move_back_todo: pub(self)
//    │   └── fn remove_task: pub(self)
//    └── mod things_todo: pub(self)
//        ├── fn add_activity: pub(self)
//        ├── fn mark_completed: pub(self)
//        └── fn update_activity: pub(self)

mod exported_module;  // the compiler will try to find a file called 'exported_module.rs' in src
use crate::exported_module::something_more;  // this this we dont have to call the function like crate::exported_module::something_more();
// INSTEAD we now can do directly something_more();

fn lets_add_task() {
    let task = some_module::Tasks {item: String::from("Task")};
    some_module::items_completed::remove_task();  // relative path to the function
    crate::some_module::items_completed::remove_task(); // absolute path with crate keyword (start at the root crate)
    // from the external module
    something_more();
} 

// besides, we do not want all out code inside this lib.rs file, therefore, we will do it in other files
// and export them

// so inside src we create a new file -> 'src/exported_module.rs'

// but we also can create nest modules in folders --> we create a folder with a file called /src/exported_module/other_module.rs'

// THEREFORE, is other_module will be inside the exported_module BUT WE have to add in the 'exported_module.rs' --> se the file is a pub mod 

use crate::exported_module::other_module::randomfunct;
// use exported_module::other_module::randomfunct; //remeber crate to go to the top of the module

// REMEMBER ALSO TO USE: cargo modules structure --> to see crate hierarchy