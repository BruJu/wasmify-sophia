//import * as rust from "..//wasm_example.js";
var rust = import("..//wasm_example.js")



rust.then(func => {
    console.log(func);
    func.run_alert("JavaScript")

}).catch(console.log)