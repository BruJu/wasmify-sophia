# This file modify the wasm_example.js file to integrate polymorphism

def modify_js_file(js_file_name):
    datafactory_polymorphism = {
        'text': '''
    /* ==== ADDED BY js_integrate_polymorphism.py ==== */

    DataFactory.prototype.literal = function(value, languageOrDatatype) {
        if (languageOrDatatype === null || languageOrDatatype === undefined) {
            return undefined;
        } else if (Object.prototype.toString.call(languageOrDatatype) === "[object String]") {
            return this.literalFromString(value, languageOrDatatype);
        } else {
            return this.literalFromNamedNode(value, languageOrDatatype);
        }
    }

    /* ==== END ADDED BY js_integrate_polymorphism.py ==== */

    ''',
        'before': 'self.wasm_bindgen = Object.assign(init, __exports);'
    }

    bypass_check = False

    with open(js_file_name, "r") as f:
        lines = f.readlines()
    
    with open(js_file_name, "w") as f:
        for line in lines:
            if not bypass_check and line.find(datafactory_polymorphism['before']) != -1:
                f.write(datafactory_polymorphism['text'])
                bypass_check = True
            
            f.write(line)


if __name__ == '__main__':
    modify_js_file('wasm_example.js')
