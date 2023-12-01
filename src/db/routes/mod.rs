pub mod kvs;
pub mod root;
pub mod version;

// In this module, we define the route handlers
// for the database server HTTP API.
//
// File format: <path>.rs
// Example: version.rs of /version.
//
// Inside the file, we define the public "handler"
// function which takes a reference to a Request
// and returns a Response.
//
// Inside the handler function, we define the functions
// that handle different HTTP methods.
// Function name format: <method>
// Example: get, post, put, delete, etc.
//
// Note: Avoid wildcard imports.
