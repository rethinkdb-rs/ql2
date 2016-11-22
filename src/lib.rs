//! RethinkDB protocol implementation in Rust

extern crate protobuf;

mod proto;

/// *Arrays* are lists of zero or more elements.
///
/// ```json
/// [1, 2, 3]
/// []
/// [{user: 'Bob', posts: 23}, {user: 'Jason', posts: 10}]
/// ```
/// Anything valid in a JSON array is valid in RethinkDB.
/// The elements may be any of the basic values, objects,
/// or other arrays. Arrays in RethinkDB are loaded fully
/// into memory before they’re returned to the user, so
/// they’re inefficient at large sizes. RethinkDB defaults
/// to supporting arrays of up to 100,000 elements; this
/// may be set to a different value at runtime for reading
/// by using the `array_limit` option to run.
pub struct Array(proto::Term);

/// *Booleans* are `true` and `false`
pub struct Bool(proto::Term);

/// *Databases* are RethinkDB databases.
///
/// This is the return type of `db`.
pub struct Db(proto::Term);

/// *Functions* can be passed as parameters to certain ReQL
/// commands.
pub struct Function(proto::Term);

/// *Grouped data* is created by the `group` command.
///
/// The command partitions a stream into multiple groups
/// based on specified fields or functions. ReQL commands
/// called on GroupedData operate on each group
/// individually. For more details, read the group
/// documentation. Depending on the input to group,
/// grouped data may have the type of GroupedStream.
pub struct GroupedData(proto::Term);
pub struct GroupedStream(proto::Term);

/// *Minval* and *maxval* are used with some commands such
/// as `between` to specify absolute lower and upper bounds
/// (e.g., `between(r.minval, 1000)` would return all
/// documents in a table whose primary key is less than
/// 1000).
pub struct MaxVal(proto::Term);
pub struct MinVal(proto::Term);

/// *Null* is a value distinct from the number zero, an
/// empty set, or a zero-length string.
///
/// Natively this is
/// `None`. It is often used to explicitly denote the
/// absence of any other value. The root node of a tree
/// structure might have a parent of `null`, or a required
/// but as yet non-initialized key might be given a value
/// of `null`.
pub struct Null(proto::Term);

/// *Numbers* are any real number: `5`, `3.14159`, `-42`.
///
/// RethinkDB uses double precision (64-bit) floating point
/// numbers internally. Neither infinity nor NaN are allowed.
pub struct Number(proto::Term);

/// *Objects* are JSON data objects, standard key-value pairs.
///
/// ```json
/// {
///     username: 'bob',
///     posts: 23,
///     favorites: {
///         color: 'blue',
///         food: 'tacos'
///     },
///     friends: ['agatha', 'jason']
/// }
/// ```
///
/// Any valid JSON object is a valid RethinkDB object, so
/// values can be any of the basic values, arrays, or
/// other objects. Documents in a RethinkDB database are
/// objects. Like JSON, key names must be strings, not
/// integers.
pub struct Object(proto::Term);

/// *Binary* objects are similar to BLOBs in SQL databases:
/// files, images and other binary data.
///
/// See Storing binary objects for details.
pub struct Binary(proto::Term);

/// *Geometry* data types for geospatial support, including
/// points, lines, and polygons.
pub struct Geometry(proto::Term);

/// *Times* are RethinkDB’s native date/time type, stored
/// with millisecond precision.
///
/// You can use native date/time types in supported
/// languages, as the conversion will be done by the driver.
/// See Dates and times in RethinkDB for details.
pub struct Time(proto::Term);

/// *Selections* represent subsets of tables, for example,
/// the return values of `filter` or `get`.
///
/// There are three kinds of selections: *Selection<Object>*,
/// *Selection<Array>* and *Selection<Stream>*. The
/// difference between selections and their non-selection
/// counterparts is that selections are writable—their
/// return values can be passed as inputs to ReQL commands
/// that modify the database. For instance, the get command
/// will return a Selection<Object> that could then be
/// passed to an update or delete command.
///
/// (Note: *singleSelection* is an older term for
/// Selection<Object>; they mean the same thing.)
/// Some commands (`order_by` and `between`) return a data
/// type similar to a selection called a *table_slice*.
/// In most cases a table_slice behaves identically to a
/// selection, but `between` can only be called on a table
/// or a table_slice, not any other kind of selection.
pub struct Selection<T>(T);

/// *Streams* are lists like arrays, but they’re loaded in
/// a lazy fashion.
///
/// Operations that return streams return a `cursor`.
/// A cursor is a pointer into the result set. Instead of
/// reading the results all at once like an array, you
/// loop over the results, retrieving the next member of the
/// set with each iteration. This makes it possible to
/// efficiently work with large result sets.
/// 
/// (See “Working with Streams,” below, for some tips.)
/// Streams are read-only; you can’t pass one as an input
/// to an ReQL command meant to modify its input like
/// `update` or `delete`.
pub struct Stream(proto::Term);

/// *Strings* are any valid UTF-8 string: `"superhero"`,
/// `"ünnëcëssärÿ ümläüts"`. Strings may include the null
/// code point (U+0000).
pub struct String(proto::Term);

/// *Tables* are RethinkDB database tables.
///
/// They behave like selections—they’re writable, as you can
/// insert and delete documents in them. ReQL methods that
/// use an index, like `get_all`, are only available on
/// tables.
pub struct Table(proto::Term);
pub struct TableSlice(proto::Term);
