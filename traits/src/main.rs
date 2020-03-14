// To create a trait we just need to specify word trait and the name of the trait.
// It might have multiple functions which structs must implement, traits are like interfaces.
// Function for trait look like normal one, but instead the function boyd it has ;
// pub is used here if someone else would like to implement this trait
pub trait Summary {
    fn summarize(&self) -> String;
    // We can call even function in trait itself
    // But it isn't possible to call the dafult implementation from
    // an overriding implementation of that same method.
    fn summarize_default(&self) -> String {
        format!("This is default function + {}", self.summarize())
    }
}

// Traits could have default implementations which could be overriden afterwards.
pub trait Post {
    fn post(&self) {
        println!("Your items has been posted.")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// To implement trait we should use impl "Trait name" for "Struct name"
// And fn implementation
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// To implement default trait we specify an empty impl block
impl Post for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Post for Tweet {
    fn post(&self) {
        println!("Your tweet has been posted.")
    }
}

// We can specify the trait as parameter
// That means that notify function accepts any type which has implemented
// It can call only functions which are part of the trait
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

// This function is completely the same as previous one,
// but it has more verbose way to define it.
pub fn notify_same<T: Summary>(item: T) {}

pub fn notify_same2<T: Summary>(item: T, item2: T) {}

// Multiple trait implementation
pub fn some_function(item: impl Summary + Display) {}

pub fn some_function2<T: Summary + Display>(item: T) {}

// Clearer Trait bounds with where clause
// isntead of this
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
// we can do
pub fn some<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

// We can use trait as return value as well
fn returns_trait() -> impl Summary {
    // But we can't return different types which implements same trait YET
    //TODO: come back and update this
}

fn main() {
    let tweet = Tweet {
        username: String::from("accierro"),
        content: String::from("check my instagram and tweeter"),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());
    tweet.post();
    println!("{}", tweet.summarize_default())
}
