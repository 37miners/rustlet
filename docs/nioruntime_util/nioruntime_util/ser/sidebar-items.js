initSidebarItems({"fn":[["deserialize","Deserializes a Readable from any std::io::Read implementation."],["deserialize_default","Deserialize a Readable"],["read_multi","Reads multiple serialized items into a Vec."],["ser_vec","Utility function to serialize a writeable directly in memory using a Vec."],["serialize","Serializes a Writeable into any std::io::Write implementation."],["serialize_default","Serialize a Writeable"]],"struct":[["BinReader","Utility to read from a binary source"],["BinWriter","Utility wrapper for an underlying byte Writer. Defines higher level methods to write numbers, byte vectors, hashes, etc."],["BufReader","Wrapper around a `Buf` impl"],["IteratingReader","Reader that exposes an Iterator interface."],["StreamingReader","A reader that reads straight off a stream. Tracks total bytes read so we can verify we read the right number afterwards."]],"trait":[["Readable","Trait that every type that can be deserialized from binary must implement. Reads directly to a Reader, a utility type thinly wrapping an underlying Read implementation."],["Reader","Implementations defined how different numbers and binary structures are read from an underlying stream or container (depending on implementation)."],["Writeable","Trait that every type that can be serialized as binary must implement. Writes directly to a Writer, a utility type thinly wrapping an underlying Write implementation."],["Writer",""]]});