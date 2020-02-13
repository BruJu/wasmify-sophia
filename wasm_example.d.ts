/* tslint:disable */
/* eslint-disable */
/**
*/
export class DataFactory {
  free(): void;
/**
*/
  constructor();
/**
* @param {string} value 
* @returns {Term} 
*/
  namedNode(value: string): Term;
/**
* @param {string | undefined} value 
* @returns {Term} 
*/
  blankNode(value?: string): Term;
/**
* @param {string} value 
* @param {string} language 
* @returns {Term} 
*/
  literalFromString(value: string, language: string): Term;
/**
* @param {string} value 
* @param {any} named_node 
* @returns {Term} 
*/
  literalFromNamedNode(value: string, named_node: any): Term;
/**
* @param {string} value 
* @returns {Term} 
*/
  variable(value: string): Term;
/**
* @returns {Term} 
*/
  defaultGraph(): Term;
/**
* @param {any} subject 
* @param {any} predicate 
* @param {any} object 
* @param {any | undefined} graph 
* @returns {Quad} 
*/
  quad(subject: any, predicate: any, object: any, graph?: any): Quad;
/**
* @param {any} original 
* @returns {Term} 
*/
  fromTerm(original: any): Term;
/**
* @param {any} original 
* @returns {Quad} 
*/
  fromQuad(original: any): Quad;
}
/**
* A sample JSTerm to be used in the RDF/JS interface
* David Pojunas
* Pierre-Antoine Champin
*/
export class JSDataset {
  free(): void;
/**
*/
  constructor();
/**
* @returns {DataFactory} 
*/
  cloneFactory(): DataFactory;
/**
* @param {string} content 
*/
  load(content: string): void;
/**
* @returns {any} 
*/
  getTerms(): any;
/**
* @param {any} quad 
*/
  add(quad: any): void;
/**
* @param {any} quad 
*/
  delete(quad: any): void;
/**
* @param {any} quad 
* @returns {boolean} 
*/
  has(quad: any): boolean;
/**
* @param {any | undefined} subject 
* @param {any | undefined} predicate 
* @param {any | undefined} object 
* @param {any | undefined} graph 
* @returns {JSDataset} 
*/
  match(subject?: any, predicate?: any, object?: any, graph?: any): JSDataset;
/**
* @returns {any} 
*/
  quads(): any;
}
/**
*/
export class Quad {
  free(): void;
/**
* @returns {boolean} 
*/
  is_connected_to_rust(): boolean;
/**
* @returns {string} 
*/
  toString(): string;
/**
* @param {any | undefined} other 
* @returns {boolean} 
*/
  equals(other?: any): boolean;
  graph: any;
  object: any;
  predicate: any;
  subject: any;
}
/**
*/
export class Term {
  free(): void;
/**
* @returns {boolean} 
*/
  is_connected_to_rust(): boolean;
/**
* @param {any | undefined} other 
* @returns {boolean} 
*/
  equals(other?: any): boolean;
/**
* @returns {string} 
*/
  toString(): string;
  datatype: any;
  language: string;
  readonly termType: string;
  value: string;
}

/**
* If `module_or_path` is {RequestInfo}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {RequestInfo | BufferSource | WebAssembly.Module} module_or_path
*
* @returns {Promise<any>}
*/
export default function init (module_or_path?: RequestInfo | BufferSource | WebAssembly.Module): Promise<any>;
        