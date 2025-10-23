use zero_to_production::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run().await
    /*
       from the rust book let's read a little and then try to apply it here.
       HttpServer as a struct handles all "transport level" concerns.

       After we establish a new transport level concern handler, what do we do with the connections?
       That's where the "App" struct comes in.
       App is where all the application logic lives: routing, middlewares, request handlers, etc.
     */
    /*
      from the rust book let's read a little and then try to understand further.
      Asynchronous programming in rust is built on top of the Future trait: a future stands
      for a value that may not be there yet. All futures expose a poll method which has to be called
      in order to allow the future to make progress and eventually resolve to its final value.
      You can think of rust's futures as lazy: unless polled, there is no guarantee that they will execute to completion.
      this has often been described as a pull model compared to the push model adopted by other languages.

      rust's standard library by design does not invlude an asynchronous runtime: you are supposed to bring one
      into your project as a dependency, one more crate under [dependencies] in your Cargo.toml file. This approach is extremely versatile.
      you are free to implement your own runtime, optimized to cater for the specific requirements of your usecase.

      this explains why main cannot be an asynchronous function: who is in charge to call poll on it?
      there is no special configuration syntax that tell the rust compiler that one of your dependencies is an asynch runtime
      and to be fair there is not even a standized definition of what a runtime is.
      You are therefore xpect ed to launch your async runtime at the top of your main function and then use it to drive your futures to completion.
      You might have guessed by now what the purpose of the tokio::main macros is but guesses are not enough to satisfy us: we want tos ee it.

      How? let's use the `cargo expand` macro to see what's going on under the hood.
     */
}
