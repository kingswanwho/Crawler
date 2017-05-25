/*
 * This file contains the new error kind
 */

error_chain!{
    foreign_links {
        Io(::std::io::Error);
        Hyper(::hyper::Error);
        Url(::hyper::error::ParseError);
    }

    errors {
        PoisonError(e: String) {
            description(e)
            display("{}", e)
        }
        // QueueEmpty {
        //     description("Queue has no item in it")
        //     display("Queue has no item in it")
        // }
    }
}