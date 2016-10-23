// Bind the `deeply::nested::function` path to `other_function`.
use self::deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

pub mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`")
        }
    }
}

pub fn main() {
    // Easier access to `deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use self::deeply::nested::function;
        function();

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        println!("Leaving block");
    }

    function();
}
