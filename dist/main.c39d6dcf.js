// modules are defined as an array
// [ module function, map of requires ]
//
// map of requires is short require name -> numeric require
//
// anything defined in a previous bundle is accessed via the
// orig method which is the require for previous bundles
parcelRequire = (function (modules, cache, entry, globalName) {
  // Save the require from previous bundle to this closure if any
  var previousRequire = typeof parcelRequire === 'function' && parcelRequire;
  var nodeRequire = typeof require === 'function' && require;

  function newRequire(name, jumped) {
    if (!cache[name]) {
      if (!modules[name]) {
        // if we cannot find the module within our internal map or
        // cache jump to the current global require ie. the last bundle
        // that was added to the page.
        var currentRequire = typeof parcelRequire === 'function' && parcelRequire;
        if (!jumped && currentRequire) {
          return currentRequire(name, true);
        }

        // If there are other bundles on this page the require from the
        // previous one is saved to 'previousRequire'. Repeat this as
        // many times as there are bundles until the module is found or
        // we exhaust the require chain.
        if (previousRequire) {
          return previousRequire(name, true);
        }

        // Try the node require function if it exists.
        if (nodeRequire && typeof name === 'string') {
          return nodeRequire(name);
        }

        var err = new Error('Cannot find module \'' + name + '\'');
        err.code = 'MODULE_NOT_FOUND';
        throw err;
      }

      localRequire.resolve = resolve;
      localRequire.cache = {};

      var module = cache[name] = new newRequire.Module(name);

      modules[name][0].call(module.exports, localRequire, module, module.exports, this);
    }

    return cache[name].exports;

    function localRequire(x){
      return newRequire(localRequire.resolve(x));
    }

    function resolve(x){
      return modules[name][1][x] || x;
    }
  }

  function Module(moduleName) {
    this.id = moduleName;
    this.bundle = newRequire;
    this.exports = {};
  }

  newRequire.isParcelRequire = true;
  newRequire.Module = Module;
  newRequire.modules = modules;
  newRequire.cache = cache;
  newRequire.parent = previousRequire;
  newRequire.register = function (id, exports) {
    modules[id] = [function (require, module) {
      module.exports = exports;
    }, {}];
  };

  var error;
  for (var i = 0; i < entry.length; i++) {
    try {
      newRequire(entry[i]);
    } catch (e) {
      // Save first error but execute all entries
      if (!error) {
        error = e;
      }
    }
  }

  if (entry.length) {
    // Expose entry point to Node, AMD or browser globals
    // Based on https://github.com/ForbesLindesay/umd/blob/master/template.js
    var mainExports = newRequire(entry[entry.length - 1]);

    // CommonJS
    if (typeof exports === "object" && typeof module !== "undefined") {
      module.exports = mainExports;

    // RequireJS
    } else if (typeof define === "function" && define.amd) {
     define(function () {
       return mainExports;
     });

    // <script>
    } else if (globalName) {
      this[globalName] = mainExports;
    }
  }

  // Override the current require with this new one
  parcelRequire = newRequire;

  if (error) {
    // throw error from earlier, _after updating parcelRequire_
    throw error;
  }

  return newRequire;
})({"Table.ts":[function(require,module,exports) {
"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});

var helper_1 = require("./helper");

var DAYS = ["Poniedziałek", "Wtorek", "Środa", "Czwartek", "Piątek", "Sobota", "Niedziela"];

var Table =
/** @class */
function () {
  function Table(data, insert_id, offset) {
    this.data = data;
    this.insert_id = insert_id;
    this.offset = offset;
  }

  Table.prototype.render = function () {
    var n_day = 0;

    var td = function td(value) {
      return "<td class=\"td_1\">" + value + "</td>";
    };

    var tr = function tr(value) {
      return "<tr class=\"tr_1\">" + value + "</tr>";
    };

    var content = "";

    for (var i = 0; i < this.data.length - 6; i += 7) {
      var day = "";

      for (var j = 0; j < 7; j++) {
        day += td(this.data[i + j].name);
      }

      content += tr(td(DAYS[n_day]) + day);
      n_day < 7 ? n_day++ : n_day = 0;
    }

    var week = helper_1.getCurrentWeeks(this.offset);
    return "<table class=\"container\"><thead><tr><th class=\"date\">" + week + "</th><th>8:00 - 9:35</th><th>9:50 - 11:25 </th><th>11:40 - 13:15</th><th>13:30 - 15:05</th>\n            <th>15:45 - 17:20</th><th>17:35 - 19:10</th><th>19:25 - 21:00</th></td></thead><tbody>" + content + "</tbody></table>";
  };

  Table.prototype.inject = function (id) {
    document.getElementById(id || this.insert_id).innerHTML = this.render();
  };

  return Table;
}();

exports.default = Table;
},{"./helper":"helper.ts"}],"helper.ts":[function(require,module,exports) {
"use strict";

var __importDefault = this && this.__importDefault || function (mod) {
  return mod && mod.__esModule ? mod : {
    "default": mod
  };
};

Object.defineProperty(exports, "__esModule", {
  value: true
});

var Table_1 = __importDefault(require("./Table"));

exports.BASE_URL = "https://wat-plan-backend.herokuapp.com/group";
exports.START_DATE = new Date(2020, 1, 24);

function toDays(date) {
  return Math.floor(date.valueOf() / 86400000);
}

function addDays(date, days) {
  var result = new Date(date);
  result.setDate(result.getDate() + days);
  return result;
}

function getDataOffset() {
  var currDays = toDays(new Date());
  var startDays = toDays(exports.START_DATE);
  return Math.floor((currDays - startDays) / 7);
}

exports.getDataOffset = getDataOffset;

function getCurrentWeeks(offset) {
  var weekOffset = getDataOffset() + offset;
  var startWeek = addDays(new Date(), weekOffset * 7);
  var endWeek = addDays(new Date(), (weekOffset + 1) * 7);
  return startWeek.toISOString().split('T')[0].slice(5) + " ~ " + endWeek.toISOString().split('T')[0].slice(5);
}

exports.getCurrentWeeks = getCurrentWeeks;

function start(group, numberOfWeeks) {
  if (numberOfWeeks === void 0) {
    numberOfWeeks = 10;
  }

  var data = fetchData(group);
  var offsetNumberOfWeeks = getDataOffset();
  console.log(offsetNumberOfWeeks);

  for (var i = offsetNumberOfWeeks; i < numberOfWeeks + offsetNumberOfWeeks; i++) {
    var div = document.createElement('div');
    div.id = 'container' + i.toString();
    document.body.appendChild(div);
    var weekData = data.slice(i * 49, (i + 1) * 49);
    var table = new Table_1.default(weekData, 'container' + i, i);
    table.inject();
  }
}

exports.default = start;

function fetchData(group) {
  var req = new XMLHttpRequest();
  var data;
  req.open('GET', exports.BASE_URL + "?group=" + group, false);

  req.onreadystatechange = function (aEvt) {
    if (req.readyState == 4) {
      if (req.status == 200) data = JSON.parse(JSON.parse(req.response));
    }
  };

  req.send(null);
  return data;
}

exports.fetchData = fetchData;
},{"./Table":"Table.ts"}],"Input.ts":[function(require,module,exports) {
"use strict";

var __importDefault = this && this.__importDefault || function (mod) {
  return mod && mod.__esModule ? mod : {
    "default": mod
  };
};

Object.defineProperty(exports, "__esModule", {
  value: true
});

var helper_1 = __importDefault(require("./helper"));

var Input =
/** @class */
function () {
  function Input(id, placeholder, insert_id, onkeydown) {
    this.id = id;
    this.placeholder = placeholder;
    insert_id ? this.insert_id = insert_id : null;
    onkeydown ? this.onKeyDown = onkeydown : null;
  }

  Input.prototype.render = function () {
    return "<div class=\"form__group field\">\n        <input type=\"input\" class=\"form__field\" placeholder=\"" + this.placeholder + "\" value=\"\" name=\"" + this.placeholder + "\" id='" + this.id + "' required />\n        <label for=\"" + this.placeholder + "\" class=\"form__label\">" + this.placeholder + "</label>\n        </div>";
  };

  Input.prototype.onKeyDown = function (e) {
    switch (e.which) {
      case 13:
        helper_1.default(this.getValue());
        break;

      default:
        break;
    }
  };

  Input.prototype.getValue = function () {
    return this.element.value;
  };

  Input.prototype.inject = function (id) {
    var outerElement = document.getElementById(id || this.insert_id || 'body');
    outerElement.innerHTML = this.render();
    this.element = outerElement.querySelector("#" + this.id);
    this.element.onkeydown = this.onKeyDown.bind(this);
  };

  return Input;
}();

exports.default = Input;
},{"./helper":"helper.ts"}],"main.ts":[function(require,module,exports) {
"use strict";

var __importDefault = this && this.__importDefault || function (mod) {
  return mod && mod.__esModule ? mod : {
    "default": mod
  };
};

Object.defineProperty(exports, "__esModule", {
  value: true
});

var Input_1 = __importDefault(require("./Input"));

var input = new Input_1.default('input_1', 'Grupa', 'search_bar');
input.inject();
},{"./Input":"Input.ts"}],"../../../../../../../usr/local/lib/node_modules/parcel/src/builtins/hmr-runtime.js":[function(require,module,exports) {
var global = arguments[3];
var OVERLAY_ID = '__parcel__error__overlay__';
var OldModule = module.bundle.Module;

function Module(moduleName) {
  OldModule.call(this, moduleName);
  this.hot = {
    data: module.bundle.hotData,
    _acceptCallbacks: [],
    _disposeCallbacks: [],
    accept: function (fn) {
      this._acceptCallbacks.push(fn || function () {});
    },
    dispose: function (fn) {
      this._disposeCallbacks.push(fn);
    }
  };
  module.bundle.hotData = null;
}

module.bundle.Module = Module;
var checkedAssets, assetsToAccept;
var parent = module.bundle.parent;

if ((!parent || !parent.isParcelRequire) && typeof WebSocket !== 'undefined') {
  var hostname = "" || location.hostname;
  var protocol = location.protocol === 'https:' ? 'wss' : 'ws';
  var ws = new WebSocket(protocol + '://' + hostname + ':' + "43125" + '/');

  ws.onmessage = function (event) {
    checkedAssets = {};
    assetsToAccept = [];
    var data = JSON.parse(event.data);

    if (data.type === 'update') {
      var handled = false;
      data.assets.forEach(function (asset) {
        if (!asset.isNew) {
          var didAccept = hmrAcceptCheck(global.parcelRequire, asset.id);

          if (didAccept) {
            handled = true;
          }
        }
      }); // Enable HMR for CSS by default.

      handled = handled || data.assets.every(function (asset) {
        return asset.type === 'css' && asset.generated.js;
      });

      if (handled) {
        console.clear();
        data.assets.forEach(function (asset) {
          hmrApply(global.parcelRequire, asset);
        });
        assetsToAccept.forEach(function (v) {
          hmrAcceptRun(v[0], v[1]);
        });
      } else if (location.reload) {
        // `location` global exists in a web worker context but lacks `.reload()` function.
        location.reload();
      }
    }

    if (data.type === 'reload') {
      ws.close();

      ws.onclose = function () {
        location.reload();
      };
    }

    if (data.type === 'error-resolved') {
      console.log('[parcel] ✨ Error resolved');
      removeErrorOverlay();
    }

    if (data.type === 'error') {
      console.error('[parcel] 🚨  ' + data.error.message + '\n' + data.error.stack);
      removeErrorOverlay();
      var overlay = createErrorOverlay(data);
      document.body.appendChild(overlay);
    }
  };
}

function removeErrorOverlay() {
  var overlay = document.getElementById(OVERLAY_ID);

  if (overlay) {
    overlay.remove();
  }
}

function createErrorOverlay(data) {
  var overlay = document.createElement('div');
  overlay.id = OVERLAY_ID; // html encode message and stack trace

  var message = document.createElement('div');
  var stackTrace = document.createElement('pre');
  message.innerText = data.error.message;
  stackTrace.innerText = data.error.stack;
  overlay.innerHTML = '<div style="background: black; font-size: 16px; color: white; position: fixed; height: 100%; width: 100%; top: 0px; left: 0px; padding: 30px; opacity: 0.85; font-family: Menlo, Consolas, monospace; z-index: 9999;">' + '<span style="background: red; padding: 2px 4px; border-radius: 2px;">ERROR</span>' + '<span style="top: 2px; margin-left: 5px; position: relative;">🚨</span>' + '<div style="font-size: 18px; font-weight: bold; margin-top: 20px;">' + message.innerHTML + '</div>' + '<pre>' + stackTrace.innerHTML + '</pre>' + '</div>';
  return overlay;
}

function getParents(bundle, id) {
  var modules = bundle.modules;

  if (!modules) {
    return [];
  }

  var parents = [];
  var k, d, dep;

  for (k in modules) {
    for (d in modules[k][1]) {
      dep = modules[k][1][d];

      if (dep === id || Array.isArray(dep) && dep[dep.length - 1] === id) {
        parents.push(k);
      }
    }
  }

  if (bundle.parent) {
    parents = parents.concat(getParents(bundle.parent, id));
  }

  return parents;
}

function hmrApply(bundle, asset) {
  var modules = bundle.modules;

  if (!modules) {
    return;
  }

  if (modules[asset.id] || !bundle.parent) {
    var fn = new Function('require', 'module', 'exports', asset.generated.js);
    asset.isNew = !modules[asset.id];
    modules[asset.id] = [fn, asset.deps];
  } else if (bundle.parent) {
    hmrApply(bundle.parent, asset);
  }
}

function hmrAcceptCheck(bundle, id) {
  var modules = bundle.modules;

  if (!modules) {
    return;
  }

  if (!modules[id] && bundle.parent) {
    return hmrAcceptCheck(bundle.parent, id);
  }

  if (checkedAssets[id]) {
    return;
  }

  checkedAssets[id] = true;
  var cached = bundle.cache[id];
  assetsToAccept.push([bundle, id]);

  if (cached && cached.hot && cached.hot._acceptCallbacks.length) {
    return true;
  }

  return getParents(global.parcelRequire, id).some(function (id) {
    return hmrAcceptCheck(global.parcelRequire, id);
  });
}

function hmrAcceptRun(bundle, id) {
  var cached = bundle.cache[id];
  bundle.hotData = {};

  if (cached) {
    cached.hot.data = bundle.hotData;
  }

  if (cached && cached.hot && cached.hot._disposeCallbacks.length) {
    cached.hot._disposeCallbacks.forEach(function (cb) {
      cb(bundle.hotData);
    });
  }

  delete bundle.cache[id];
  bundle(id);
  cached = bundle.cache[id];

  if (cached && cached.hot && cached.hot._acceptCallbacks.length) {
    cached.hot._acceptCallbacks.forEach(function (cb) {
      cb();
    });

    return true;
  }
}
},{}]},{},["../../../../../../../usr/local/lib/node_modules/parcel/src/builtins/hmr-runtime.js","main.ts"], null)
//# sourceMappingURL=/main.c39d6dcf.js.map