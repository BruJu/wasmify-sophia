# Reflexion on building a Stream class that resorts to RIO.

[sophia_benchmark](https://github.com/pchampin/sophia_benchmark) uses 
[ReadableStreams](https://nodejs.org/api/stream.html#stream_readable_streams) to
read the content of a file.

The implementation of n3js can receive this stream and build quads from it,
which is the first test proposed by sophia benchmark.

As we don't implement a Stream API, we resort to n3js interface, which is nice
because we are able to run the other test (the research of quads matching
certain criteria).

This document lists my reflection on how to implement the Stream API (and why
it doesn't work)


## Rio Api

To receive Data, riot parsers are built with a BufRead object.

A BufRead can easily built from a `Read` object, which is simply an object that
implements the `read` function that fills an array and returns the number of
read bytes.

~~The main problem with this is that it means that a Rio Parser is pulling the
chunks it needs.~~

https://doc.rust-lang.org/std/io/trait.Read.html

A Readable object can actually block the thread until data have been read.


## ReadableStream

It is possible to convert a ReadableStream (which is an eventlistner) into
a promise

```js
function readStream(stream, encoding = "utf8") {
    
    stream.setEncoding(encoding);

    return new Promise((resolve, reject) => {
        let data = "";
        
        stream.on("data", chunk => data += chunk);
        stream.on("end", () => resolve(data));
        stream.on("error", error => reject(error));
    });
}

const text = await readStream(process.stdin);
```
(Source : to be found)


js_sys promise exists

## Resynchronize from a future


```javascript
    await promise;
```

await also exists in rust

```js
    promise.await?;

```

## Other concerns andn otes

```rust
async fn foo() -> Result<JsValue, JsValue> {



}
```
Result is actually converted inot a promise.

The exposed api by ReadableStream matches https://rdf.js.org/stream-spec/















