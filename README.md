# Rust Iterator Exhaustion Example

This repository demonstrates a common error in Rust involving the exhaustion of iterators.  The `bug.rs` file contains code that will panic at runtime due to an attempt to access elements of an iterator after it has been fully consumed.

The `bugSolution.rs` file provides a corrected version of the code that addresses this issue.