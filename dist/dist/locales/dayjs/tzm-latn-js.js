(self["webpackChunkconverse_js"] = self["webpackChunkconverse_js"] || []).push([[3933],{

/***/ 6170:
/***/ (function(module, __unused_webpack_exports, __webpack_require__) {

!function (a, s) {
   true ? module.exports = s(__webpack_require__(8570)) : 0;
}(this, function (a) {
  "use strict";

  function s(a) {
    return a && "object" == typeof a && "default" in a ? a : {
      default: a
    };
  }

  var n = s(a),
      i = {
    name: "tzm-latn",
    weekdays: "asamas_aynas_asinas_akras_akwas_asimwas_asiḍyas".split("_"),
    months: "innayr_brˤayrˤ_marˤsˤ_ibrir_mayyw_ywnyw_ywlywz_ɣwšt_šwtanbir_ktˤwbrˤ_nwwanbir_dwjnbir".split("_"),
    weekStart: 6,
    weekdaysShort: "asamas_aynas_asinas_akras_akwas_asimwas_asiḍyas".split("_"),
    monthsShort: "innayr_brˤayrˤ_marˤsˤ_ibrir_mayyw_ywnyw_ywlywz_ɣwšt_šwtanbir_ktˤwbrˤ_nwwanbir_dwjnbir".split("_"),
    weekdaysMin: "asamas_aynas_asinas_akras_akwas_asimwas_asiḍyas".split("_"),
    ordinal: function (a) {
      return a;
    },
    formats: {
      LT: "HH:mm",
      LTS: "HH:mm:ss",
      L: "DD/MM/YYYY",
      LL: "D MMMM YYYY",
      LLL: "D MMMM YYYY HH:mm",
      LLLL: "dddd D MMMM YYYY HH:mm"
    },
    relativeTime: {
      future: "dadkh s yan %s",
      past: "yan %s",
      s: "imik",
      m: "minuḍ",
      mm: "%d minuḍ",
      h: "saɛa",
      hh: "%d tassaɛin",
      d: "ass",
      dd: "%d ossan",
      M: "ayowr",
      MM: "%d iyyirn",
      y: "asgas",
      yy: "%d isgasn"
    }
  };
  return n.default.locale(i, null, !0), i;
});

/***/ })

}]);
//# sourceMappingURL=tzm-latn-js.js.map