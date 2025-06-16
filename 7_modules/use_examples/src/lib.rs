mod same_name {
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result {
        // --snip--
        fmt::Result::Ok(())
    }

    fn function2() -> io::Result<()> {
        // --snip--
        io::Result::Ok(())
    }
}

mod alias {
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
        // --snip--
        Result::Ok(())
    }

    fn function2() -> IoResult<()> {
        // --snip--
        IoResult::Ok(())
    }
}
