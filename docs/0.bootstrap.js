(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/adventofcode2022.js":
/*!**********************************!*\
  !*** ../pkg/adventofcode2022.js ***!
  \**********************************/
/*! exports provided: exec_test, exec_1, __wbg_log_55cec5609e5b22dc */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _adventofcode2022_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./adventofcode2022_bg.wasm */ \"../pkg/adventofcode2022_bg.wasm\");\n/* harmony import */ var _adventofcode2022_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./adventofcode2022_bg.js */ \"../pkg/adventofcode2022_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"exec_test\", function() { return _adventofcode2022_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"exec_test\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"exec_1\", function() { return _adventofcode2022_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"exec_1\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_log_55cec5609e5b22dc\", function() { return _adventofcode2022_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_log_55cec5609e5b22dc\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/adventofcode2022.js?");

/***/ }),

/***/ "../pkg/adventofcode2022_bg.js":
/*!*************************************!*\
  !*** ../pkg/adventofcode2022_bg.js ***!
  \*************************************/
/*! exports provided: exec_test, exec_1, __wbg_log_55cec5609e5b22dc */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"exec_test\", function() { return exec_test; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"exec_1\", function() { return exec_1; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_log_55cec5609e5b22dc\", function() { return __wbg_log_55cec5609e5b22dc; });\n/* harmony import */ var _adventofcode2022_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./adventofcode2022_bg.wasm */ \"../pkg/adventofcode2022_bg.wasm\");\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachedUint8Memory0 = new Uint8Array();\n\nfunction getUint8Memory0() {\n    if (cachedUint8Memory0.byteLength === 0) {\n        cachedUint8Memory0 = new Uint8Array(_adventofcode2022_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachedUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachedInt32Memory0 = new Int32Array();\n\nfunction getInt32Memory0() {\n    if (cachedInt32Memory0.byteLength === 0) {\n        cachedInt32Memory0 = new Int32Array(_adventofcode2022_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachedInt32Memory0;\n}\n/**\n* @param {string} name\n* @returns {string}\n*/\nfunction exec_test(name) {\n    try {\n        const retptr = _adventofcode2022_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n        const ptr0 = passStringToWasm0(name, _adventofcode2022_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _adventofcode2022_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        const len0 = WASM_VECTOR_LEN;\n        _adventofcode2022_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"exec_test\"](retptr, ptr0, len0);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        return getStringFromWasm0(r0, r1);\n    } finally {\n        _adventofcode2022_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n        _adventofcode2022_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n    }\n}\n\n/**\n* @param {string} input\n* @returns {string}\n*/\nfunction exec_1(input) {\n    try {\n        const retptr = _adventofcode2022_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n        const ptr0 = passStringToWasm0(input, _adventofcode2022_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _adventofcode2022_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        const len0 = WASM_VECTOR_LEN;\n        _adventofcode2022_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"exec_1\"](retptr, ptr0, len0);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        return getStringFromWasm0(r0, r1);\n    } finally {\n        _adventofcode2022_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n        _adventofcode2022_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n    }\n}\n\nfunction __wbg_log_55cec5609e5b22dc(arg0, arg1) {\n    log(getStringFromWasm0(arg0, arg1));\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/adventofcode2022_bg.js?");

/***/ }),

/***/ "../pkg/adventofcode2022_bg.wasm":
/*!***************************************!*\
  !*** ../pkg/adventofcode2022_bg.wasm ***!
  \***************************************/
/*! exports provided: memory, exec_test, exec_1, __wbindgen_add_to_stack_pointer, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./adventofcode2022_bg.js */ \"../pkg/adventofcode2022_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/adventofcode2022_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var adventofcode2022__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! adventofcode2022 */ \"../pkg/adventofcode2022.js\");\n\n\nwindow.log = console.log.bind(console);\n\ndocument.querySelectorAll('.input').forEach((el) => {\n  const lines = el.value.split('\\n').length;\n  el.setAttribute('rows', lines);\n\n  el.addEventListener('input', () => {\n    const lines = el.value.split('\\n').length;\n    el.setAttribute('rows', lines);\n  });\n});\n\ndocument.querySelectorAll('h3').forEach((el) => {\n  const id = el.parentElement.getAttribute('data-id');\n  el.innerHTML = `Day ${id}`;\n  el.addEventListener('click', () => {\n    const opened = document.querySelector('.opened')\n    if (opened) opened.classList.remove('opened');\n    el.parentElement.classList.add('opened');\n  });\n});\n\ndocument.querySelector('h3').parentElement.classList.add('opened');\n\ndocument.querySelectorAll('.exec').forEach((el) => {\n  el.addEventListener('click', () => {\n    const target = el.parentElement.parentElement;\n    const id = target.getAttribute('data-id');\n    const input = target.querySelector('.input').value;\n    const output = target.querySelector('.output');\n    el.setAttribute('disabled', 'disabled');\n    output.innerHTML = adventofcode2022__WEBPACK_IMPORTED_MODULE_0__[`exec_${id}`](input) || 'Nothing to execute';\n    el.removeAttribute('disabled');\n  });\n});\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);